mod renderer;
mod vr_app;

fn main() {
    let mut app = vr_app::VrApp::new();
    app.run();
}
