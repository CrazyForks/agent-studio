use gpui::{
    px, App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Pixels, Render, Styled, Window,
};

use gpui_component::{
    input::InputState,
    list::{ListDelegate, ListItem, ListState},
    select::SelectState,
    v_flex, ActiveTheme, IndexPath,
};

use crate::components::ChatInputBox;

/// Delegate for the context list in the chat input popover
struct ContextListDelegate {
    items: Vec<ContextItem>,
}

#[derive(Clone)]
struct ContextItem {
    name: &'static str,
    icon: &'static str,
}

impl ContextListDelegate {
    fn new() -> Self {
        Self {
            items: vec![
                ContextItem {
                    name: "Files",
                    icon: "file",
                },
                ContextItem {
                    name: "Folders",
                    icon: "folder",
                },
                ContextItem {
                    name: "Code",
                    icon: "code",
                },
                ContextItem {
                    name: "Git Changes",
                    icon: "git-branch",
                },
                ContextItem {
                    name: "Terminal",
                    icon: "terminal",
                },
                ContextItem {
                    name: "Problems",
                    icon: "alert-circle",
                },
                ContextItem {
                    name: "URLs",
                    icon: "link",
                },
            ],
        }
    }
}

impl ListDelegate for ContextListDelegate {
    type Item = ListItem;

    fn items_count(&self, _: usize, _: &App) -> usize {
        self.items.len()
    }

    fn render_item(&self, ix: IndexPath, _: &mut Window, _: &mut App) -> Option<Self::Item> {
        let item = self.items.get(ix.row)?;
        Some(ListItem::new(ix).child(item.name))
    }

    fn set_selected_index(
        &mut self,
        _: Option<IndexPath>,
        _: &mut Window,
        _: &mut Context<ListState<Self>>,
    ) {
    }

    fn confirm(&mut self, _: bool, _: &mut Window, _cx: &mut Context<ListState<Self>>) {
        // Handle item selection - for now just close the popover
    }

    fn cancel(&mut self, _: &mut Window, _cx: &mut Context<ListState<Self>>) {
        // Close the popover on cancel
    }
}

pub struct ChatInputPanel {
    focus_handle: FocusHandle,
    input_state: Entity<InputState>,
    context_list: Entity<ListState<ContextListDelegate>>,
    context_popover_open: bool,
    mode_select: Entity<SelectState<Vec<&'static str>>>,
}

impl crate::dock_panel::DockPanel for ChatInputPanel {
    fn title() -> &'static str {
        "Chat Input"
    }

    fn description() -> &'static str {
        "A chat input box for sending messages."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
    fn paddings() -> Pixels {
        px(0.)
    }
}

impl ChatInputPanel {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut App) -> Self {
        let input_state = cx.new(|cx| {
            InputState::new(window, cx)
                .auto_grow(2, 8) // Auto-grow from 2 to 8 rows
                .soft_wrap(true) // Enable word wrapping
                .placeholder("Ask, search, or make anything...")
        });

        let context_list =
            cx.new(|cx| ListState::new(ContextListDelegate::new(), window, cx).searchable(true));

        let mode_select = cx.new(|cx| {
            SelectState::new(
                vec!["Auto", "Ask", "Plan", "Code", "Explain"],
                Some(IndexPath::default()), // Select "Auto" by default
                window,
                cx,
            )
        });

        Self {
            focus_handle: cx.focus_handle(),
            input_state,
            context_list,
            context_popover_open: false,
            mode_select,
        }
    }
}

impl Focusable for ChatInputPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ChatInputPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .justify_end()
            .bg(cx.theme().background)
            .child(
                ChatInputBox::new("chat-input-box", self.input_state.clone())
                    .title("Send a message")
                    .context_list(self.context_list.clone(), cx)
                    .context_popover_open(self.context_popover_open)
                    .on_context_popover_change(cx.listener(|this, open: &bool, _, cx| {
                        this.context_popover_open = *open;
                        cx.notify();
                    }))
                    .mode_select(self.mode_select.clone()),
            )
    }
}
