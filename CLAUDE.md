# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the `agentx` Agent Studio application, part of the gpui-component workspace. It demonstrates building a full-featured desktop application with GPUI Component, showcasing:

- A dock-based layout system with multiple panels (left, right, bottom, center)
- Custom title bar with menu integration and panel management
- Code editor with LSP support (diagnostics, completion, hover, code actions)
- AI conversation UI components (agent messages, user messages, tool calls, todo lists)
- Task list panel with collapsible sections and mock data loading
- Chat input panel with context controls
- Persistent layout state management with versioning
- Theme support and customization

## Architecture

### Application Structure

- **Main Entry**: `src/main.rs` creates the application window with a `DockWorkspace`
- **DockWorkspace**: The root container managing the dock area, title bar, and layout persistence
- **Panels**: Individual UI components implementing the `DockPanel` trait and wrapped in `DockPanelContainer`
- **Dock System**: Uses `DockArea` from gpui-component for flexible panel layout

### Key Components

1. **DockWorkspace** (`src/main.rs`):
   - Manages the main dock area with version-controlled layout persistence
   - Saves layout state to `target/docks-agentx.json` (debug) or `docks-agentx.json` (release)
   - Handles layout loading, saving (debounced by 10 seconds), and version migration
   - Provides actions for adding panels and toggling visibility via dropdown menu in title bar

2. **Panel System** (`src/lib.rs`):
   - `DockPanelContainer`: Wrapper for panels implementing the `Panel` trait from gpui-component
   - `DockPanel`: Custom trait that panels implement to define title, description, behavior
   - Panel registration happens in `init()` via `register_panel()` with deserialization from saved state
   - All panels are registered under the name `"DockPanelContainer"` with state determining the actual panel type

3. **Conversation UI Components** (`src/components/`):
   - **AgentMessage**: Displays AI agent responses with markdown support and streaming capability
   - **UserMessage**: Shows user messages with text and file/resource attachments
   - **ToolCallItem**: Renders tool calls with status badges (pending, running, success, error)
   - **AgentTodoList**: Interactive todo list with status tracking (pending, in_progress, completed)
   - All components follow a builder pattern for configuration

4. **Panel Implementations**:
   - **ConversationPanel** (`src/conversation.rs`): Mock conversation UI showcasing all message types
   - **CodeEditorPanel** (`src/editor.rs`): High-performance code editor with LSP integration and tree-sitter
   - **ListTaskPanel** (`src/task_list.rs`): Task list with collapsible sections, loads from `mock_tasks.json`
   - **ChatInputPanel** (`src/chat_input.rs`): Input field with "Add context" button and send controls

### Layout Persistence

The dock layout system uses versioned states:
- Current version: 5 (defined in `MAIN_DOCK_AREA` in `src/main.rs`)
- When version mismatch detected, prompts user to reset to default layout
- Layout automatically saved 10 seconds after changes (debounced)
- Layout saved on app quit via `on_app_quit` hook
- State includes panel positions, sizes, active tabs, and visibility

## Development Commands

### Build and Run

```bash
# Run from the agentx directory
cargo run

# Or from the workspace root with explicit target
cargo run --example agentx

# Run the full component gallery (workspace root)
cd ../.. && cargo run
```

### Build Only

```bash
cargo build --example agentx

# Check for compilation errors without building binaries
cargo check --example agentx
```

### Development with Performance Profiling (macOS)

```bash
# Enable Metal HUD to see FPS and GPU metrics
MTL_HUD_ENABLED=1 cargo run --example agentx

# Profile with samply (requires: cargo install samply)
samply record cargo run --example agentx
```

### Logging

The application uses `tracing` for logging. Control log levels via `RUST_LOG`:

```bash
# Enable trace logging for gpui-component
RUST_LOG=gpui_component=trace cargo run

# Enable debug logging for everything
RUST_LOG=debug cargo run
```

## GPUI Component Integration

### Initialization Pattern

Always call `gpui_component::init(cx)` before using any GPUI Component features. This Agent Studio extends initialization with custom setup:

```rust
pub fn init(cx: &mut App) {
    // Set up logging first
    tracing_subscriber::registry()...

    // Initialize gpui-component (required)
    gpui_component::init(cx);

    // Initialize app-specific state and modules
    AppState::init(cx);
    themes::init(cx);
    editor::init();
    menu::init(cx);

    // Bind keybindings
    cx.bind_keys([...]);

    // Register custom panels
    register_panel(cx, PANEL_NAME, |_, _, info, window, cx| {
        // Panel factory logic
    });
}
```

### Root Element Requirement

The first level element in a window must be a `Root` from gpui-component:

```rust
cx.new(|cx| Root::new(view, window, cx))
```

This provides essential UI layers (sheets, dialogs, notifications). For custom title bars, use `DockRoot` pattern (see `src/lib.rs:167`).

### Creating Custom Panels

To add a new panel type:

1. Implement the `DockPanel` trait (defined in `src/lib.rs`):
   - `title()`: Panel display name (static)
   - `description()`: Panel description (static)
   - `new_view()`: Create the panel view entity
   - Optional: `closable()`, `zoomable()`, `paddings()`, `on_active()`

2. Add a new variant to `DockPanelState` enum

3. Update `DockPanelState::to_story()` match statement to handle the new panel type

4. Add to default layout in `reset_default_layout()` or `init_default_layout()` in `src/main.rs`

Example panel structure:
```rust
pub struct MyPanel {
    focus_handle: FocusHandle,
}

impl DockPanel for MyPanel {
    fn title() -> &'static str { "My Panel" }
    fn description() -> &'static str { "Description here" }
    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        cx.new(|cx| Self::new(window, cx))
    }
}
```

## Key Concepts

### Dock Placement

Panels can be added to four dock areas: `Center`, `Left`, `Right`, `Bottom`

Dock areas are collapsible (except Center) and support resizing.

### Window Management

- Window bounds are centered and sized to 85% of display (max 1600x1200)
- Minimum window size: 480x320 pixels
- Custom titlebar on macOS/Windows via `TitleBar::title_bar_options()`
- Client decorations on Linux with transparent background

### State Management

- **Global state** via `AppState` for tracking invisible panels
- **Panel state** serialization via `dump()` and deserialization via panel registry
- **Layout state** includes panel positions, sizes, active tabs, and version
- **Mock data** loaded from `mock_tasks.json` for the task list panel

### Message Components Architecture

The conversation UI uses a builder pattern with type-safe components:

- **UserMessage**: `MessageContent::text()` and `MessageContent::resource()` for attachments
- **AgentMessage**: Supports streaming via `add_chunk()`, completed state, thinking indicator
- **ToolCallItem**: Status progression (pending → running → success/error)
- **AgentTodoList**: Entries with priority (high/normal/low) and status tracking

All components are exported from `src/components/mod.rs` for easy reuse.

## Testing

Run the complete story gallery from workspace root:

```bash
cd ../.. && cargo run
```

This displays all GPUI components in a comprehensive gallery interface.

The Agent Studio itself serves as a test bed for:
- Dock layout persistence and restoration
- Panel lifecycle management
- Custom UI components (messages, todos, tool calls)
- LSP integration in code editor
- Theme switching and customization

## Workspace Structure

This Agent Studio is part of a Cargo workspace at `../../`:

- `crates/ui`: Core gpui-component library
- `crates/story`: Story framework and component gallery
- `crates/macros`: Procedural macros for GPUI components
- `crates/assets`: Asset handling and management
- `examples/agentx`: This Agent Studio application
- `examples/hello_world`, `examples/input`, etc.: Other examples
- `crates/ui/src/icon.rs`: IconName definitions for the Icon component
- `crates/story/src/*.rs`: Component examples and documentation

### Important Files in agentx

- `src/main.rs`: Application entry, DockWorkspace, layout persistence
- `src/lib.rs`: Panel system, DockPanel trait, initialization, window utilities
- `src/components/`: Reusable conversation UI components
- `src/editor.rs`: Code editor with LSP integration
- `src/task_list.rs`: Task list panel with collapsible sections
- `src/conversation.rs`: Conversation panel with mock data
- `src/chat_input.rs`: Chat input panel
- `src/title_bar.rs`: Custom application title bar
- `src/themes.rs`: Theme configuration and management
- `src/menu.rs`: Application menu setup
- `mock_tasks.json`: Mock task data for the task list panel 

## Dependencies

Key dependencies defined in `Cargo.toml`:

### Core Framework
- `gpui = "0.2.2"`: Core GPUI framework for UI rendering
- `gpui-component`: UI component library (workspace member)
- `gpui-component-assets`: Asset integration (workspace member)

### Language Support
- `tree-sitter-navi = "0.2.2"`: Syntax highlighting for the code editor
- `lsp-types`: Language Server Protocol type definitions
- `color-lsp = "0.2.0"`: LSP implementation for color support

### Utilities
- `serde`, `serde_json`: Serialization for layout persistence and mock data
- `rand = "0.8"`: Random number generation for UI demos
- `autocorrect = "2.14.2"`: Text correction utilities
- `chrono = "0.4"`: Date and time handling
- `smol`: Async runtime utilities
- `tracing`, `tracing-subscriber`: Logging and diagnostics

### Workspace Dependencies

All workspace-level dependencies are defined in the root `Cargo.toml` and shared across examples.

## Coding Style and Conventions

### GPUI Patterns
- Use `cx.new()` for creating entities (not `cx.build()` or direct construction)
- Prefer `Entity<T>` over raw views for state management and lifecycle control
- Use GPUI's reactive patterns: subscriptions, notifications, actions for communication
- Implement `Focusable` trait for interactive panels to support focus management

### UI Conventions
- Mouse cursor: use `default` not `pointer` for buttons (desktop convention, not web)
- Default component size: `md` for most components (consistent with macOS/Windows)
- Use `px()` for pixel values, `rems()` for font-relative sizing
- Apply responsive layout with flexbox: `v_flex()`, `h_flex()`

### Component Design
- Follow existing patterns for component creation and layout
- Use builder pattern for component configuration (e.g., `.label()`, `.icon()`, `.ghost()`)
- Keep components stateless when possible (implement `RenderOnce`)
- For stateful components, use `Entity<T>` and implement `Render`

### Architecture Guidelines
- Separate UI components from business logic
- Use the `DockPanel` trait for all dockable panels
- Keep panel state serializable for layout persistence
- Export reusable components from appropriate module files

### Code Organization
- Place reusable UI components in `src/components/`
- Keep panel implementations in dedicated files at `src/` root
- Use `mod.rs` files to re-export public APIs
- Group related functionality in submodules
