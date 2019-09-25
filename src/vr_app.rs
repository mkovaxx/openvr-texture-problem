use crate::renderer;
use glium;
use glium::backend::glutin::glutin::GlRequest;
use glium::GlObject;
use openvr;
use std::rc::Rc;

pub struct VrApp {
    event_loop: glium::glutin::EventsLoop,
    display: Rc<glium::Display>,
    left_eye: VrEye,
    right_eye: VrEye,
    renderer: renderer::Renderer,
    vr_device: VrDevice,
}

impl VrApp {
    pub fn new() -> VrApp {
        let event_loop = glium::glutin::EventsLoop::new();

        let window_builder = glium::glutin::WindowBuilder::new()
            .with_dimensions(glium::glutin::dpi::LogicalSize::new(256.0, 256.0));
        let context = glium::glutin::ContextBuilder::new()
            .with_gl(GlRequest::Specific(glium::glutin::Api::OpenGl, (3, 1)))
            .with_gl_profile(glium::glutin::GlProfile::Core)
            .with_gl_robustness(glium::glutin::Robustness::RobustLoseContextOnReset)
            .build_windowed(window_builder, &event_loop)
            .unwrap();
        let display: Rc<glium::Display> = Rc::new(glium::Display::from_gl_window(context).unwrap());

        println!("OpenGL vendor: {}", display.get_opengl_vendor_string());
        println!("OpenGL renderer: {}", display.get_opengl_renderer_string());
        println!("OpenGL version: {}", display.get_opengl_version_string());

        let device: VrDevice = {
            let context = unsafe {
                openvr::init(openvr::ApplicationType::Scene).expect("Failed to initialize OpenVR")
            };

            let system = context.system().expect("Failed to get system interface");

            let compositor = context
                .compositor()
                .expect("Failed to create IVRCompositor subsystem");

            system
                .device_to_absolute_tracking_pose(openvr::TrackingUniverseOrigin::Standing, 0.005);

            VrDevice {
                context: context,
                system: system,
                compositor: compositor,
            }
        };

        let target_size = device.system.recommended_render_target_size();
        println!("target size: {:?}", target_size);

        let left_eye = VrEye::new(&display, target_size);
        let right_eye = VrEye::new(&display, target_size);

        let renderer = renderer::Renderer::new(&display);

        VrApp {
            event_loop: event_loop,
            display: display,
            left_eye: left_eye,
            right_eye: right_eye,
            renderer: renderer,
            vr_device: device,
        }
    }

    pub fn run(&mut self) {
        let mut running = true;

        while running {
            self.vr_device
                .compositor
                .wait_get_poses()
                .expect("Getting poses");

            let mut buffer = self.display.as_ref().draw();
            self.renderer.render_test(&mut buffer);
            buffer.finish().unwrap();

            // if this block is executed, the texture will read all black in the shader
            /*
            unsafe {
                self.left_eye.submit(&self.vr_device.compositor, openvr::Eye::Left);
                self.right_eye.submit(&self.vr_device.compositor, openvr::Eye::Right);
            }
            */

            self.event_loop.poll_events(|event| match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::CloseRequested => running = false,
                    _ => (),
                },
                _ => (),
            });
        }
    }
}

struct VrDevice {
    // NOTE(mkovacs): The context must be kept around, otherwise other fields become invalid.
    context: openvr::Context,
    system: openvr::System,
    compositor: openvr::Compositor,
}

struct VrEye {
    display: Rc<glium::Display>,
    color: glium::Texture2d,
    depth: glium::framebuffer::DepthRenderBuffer,
}

impl VrEye {
    fn new(display: &Rc<glium::Display>, size: (u32, u32)) -> VrEye {
        let color = glium::texture::Texture2d::empty_with_format(
            display.as_ref(),
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            size.0,
            size.1,
        )
        .unwrap();

        let depth = glium::framebuffer::DepthRenderBuffer::new(
            display.as_ref(),
            glium::texture::DepthFormat::I24,
            size.0,
            size.1,
        )
        .unwrap();

        VrEye {
            display: display.clone(),
            color: color,
            depth: depth,
        }
    }

    unsafe fn submit(&self, compositor: &openvr::Compositor, eye: openvr::Eye) {
        compositor
            .submit(
                eye,
                &openvr::compositor::texture::Texture {
                    handle: openvr::compositor::texture::Handle::OpenGLTexture(
                        self.color.get_id() as usize
                    ),
                    color_space: openvr::compositor::texture::ColorSpace::Auto,
                },
                None,
                None,
            )
            .expect("Submitting frame");
    }
}
