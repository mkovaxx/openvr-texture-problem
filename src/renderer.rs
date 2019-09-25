use glium;
use glium::uniform;
use std::rc::Rc;

const IMAGE_SIZE: usize = 512;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

glium::implement_vertex!(Vertex, position);

pub struct Renderer {
    program: glium::Program,
    fullscreen_quad_vertex_buffer: glium::VertexBuffer<Vertex>,
    fullscreen_quad_index_buffer: glium::IndexBuffer<u32>,
    texture: glium::texture::Texture2d,
}

impl Renderer {
    pub fn new(display: &Rc<glium::Display>) -> Renderer {
        let program = glium::Program::from_source(
            display.as_ref(),
            VERTEX_SHADER_SRC,
            FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();

        let quad_vertex_data = vec![
            Vertex {
                position: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0],
            },
        ];

        let vertex_buffer = glium::VertexBuffer::new(display.as_ref(), &quad_vertex_data).unwrap();

        let quad_index_data = vec![0, 1, 2, 1, 2, 3];

        let index_buffer = glium::IndexBuffer::new(
            display.as_ref(),
            glium::index::PrimitiveType::TrianglesList,
            &quad_index_data,
        )
        .unwrap();

        // create texture
        let pixel_data = compute_red_blue_checker_texture(IMAGE_SIZE);
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &pixel_data,
            (IMAGE_SIZE as u32, IMAGE_SIZE as u32),
        );
        let texture = glium::texture::Texture2d::new(display.as_ref(), image).unwrap();

        Renderer {
            program: program,
            fullscreen_quad_vertex_buffer: vertex_buffer,
            fullscreen_quad_index_buffer: index_buffer,
            texture: texture,
        }
    }

    pub fn render_test<S: glium::Surface>(&self, buffer: &mut S) {
        // clear the screen to green
        buffer.clear_color_and_depth((0.0, 1.0, 0.0, 1.0), 1.0);

        let parameters = glium::DrawParameters {
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
            depth: glium::Depth {
                test: glium::DepthTest::Overwrite,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let texture_sampler = self
            .texture
            .sampled()
            .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
            .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

        let uniforms = uniform! {
            checker_texture: texture_sampler,
        };

        buffer
            .draw(
                &self.fullscreen_quad_vertex_buffer,
                &self.fullscreen_quad_index_buffer,
                &self.program,
                &uniforms,
                &parameters,
            )
            .unwrap();
    }
}

fn compute_red_blue_checker_texture(size: usize) -> Vec<u8> {
    // compute red and blue checkerboard pattern
    let mut pixel_data: Vec<u8> = Vec::with_capacity(4 * size * size);
    for y in 0..size {
        for x in 0..size {
            let (r, g, b) = if (2 * x < size) ^ (2 * y < size) {
                (255, 0, 0)
            } else {
                (0, 0, 255)
            };
            pixel_data.push(r);
            pixel_data.push(g);
            pixel_data.push(b);
            pixel_data.push(0);
        }
    }
    pixel_data
}

const VERTEX_SHADER_SRC: &str = r#"
#version 140

in vec2 position;

out vec2 v_texcoord;

void main() {
    gl_Position = vec4(mix(vec2(-1.0, -1.0), vec2(1.0, 1.0), position), 0.0, 1.0);
    v_texcoord = position;
}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
#version 140

uniform sampler2D checker_texture;

in vec2 v_texcoord;

out vec4 color;

void main() {
    color = texture(checker_texture, v_texcoord);
}
"#;
