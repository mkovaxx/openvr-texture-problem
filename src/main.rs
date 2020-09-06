mod renderer;
mod vr_app;

fn main() {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let app = vr_app::VrApp::new(&event_loop);
    app.run(event_loop);
}
