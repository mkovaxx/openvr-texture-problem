use crate::renderer;
use glium;
use glium::backend::glutin::glutin::GlRequest;
use glium::GlObject;
use openvr;
use std::rc::Rc;

pub struct VrApp {
    display: Rc<glium::Display>,
    left_eye: VrEye,
    right_eye: VrEye,
    renderer: renderer::Renderer,
    vr_device: VrDevice,
}

impl VrApp {
    pub fn new(event_loop: &glium::glutin::event_loop::EventLoop<()>) -> VrApp {
        let window_builder = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(256.0, 256.0));
        let context = glium::glutin::ContextBuilder::new()
            .with_gl(GlRequest::Specific(glium::glutin::Api::OpenGl, (3, 2)))
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
            display: display,
            left_eye: left_eye,
            right_eye: right_eye,
            renderer: renderer,
            vr_device: device,
        }
    }

    pub fn run(self, event_loop: glium::glutin::event_loop::EventLoop<()>) {
        let mut is_submit_enabled = false;

        event_loop.run(move |glutin_event, _target, control_flow| {
            *control_flow = glium::glutin::event_loop::ControlFlow::Poll;
            self.vr_device
                .compositor
                .wait_get_poses()
                .expect("Getting poses");

            let mut buffer = self.display.as_ref().draw();
            self.renderer.render_test(&mut buffer);
            buffer.finish().unwrap();

            if is_submit_enabled {
                // if this block is executed, the texture will read all black in the shader
                unsafe {
                    self.left_eye.submit(&self.vr_device.compositor, openvr::Eye::Left);
                    self.right_eye.submit(&self.vr_device.compositor, openvr::Eye::Right);
                }
            }

            match glutin_event {
                glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    },
                    glium::glutin::event::WindowEvent::MouseInput {..} => {
                        is_submit_enabled = true;
                    },
                    _ => {},
                },
                _ => {},
            }
        });
    }
}

struct VrDevice {
    // NOTE(mkovacs): The context must be kept around, otherwise other fields become invalid.
    #[allow(dead_code)]
    context: openvr::Context,
    system: openvr::System,
    compositor: openvr::Compositor,
}

struct VrEye {
    color: glium::Texture2d,
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

        VrEye {
            color: color,
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
