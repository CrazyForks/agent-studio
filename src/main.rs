use agentx::workspace::open_new;
use gpui::Application;
use gpui_component_assets::Assets;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        agentx::init(cx);

        open_new(cx, |_, _, _| {
            // do something
        })
        .detach();
    });
}
