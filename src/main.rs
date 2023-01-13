#[macro_use]
extern crate glium;

mod audio;

use glium::glutin::WindowEvent::*;
use glium::{glutin, Surface};

const BUF_LEN: usize = 1024;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let audiosession    = audio::run().unwrap();

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window          = glium::glutin::WindowBuilder::new();
    let context         = glium::glutin::ContextBuilder::new();
    let display         = glium::Display::new(window, context, &events_loop).unwrap();

    let hidpi_factor    = display.gl_window().window().get_hidpi_factor();

    let shape = vec![
        Vertex { position: [-1., -1.] },
        Vertex { position: [1., 1.] },
        Vertex { position: [-1., 1.] },
        Vertex { position: [-1., -1.] },
        Vertex { position: [1., -1.] },
        Vertex { position: [1., 1.] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices       = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = include_str!("fragment.glsl");

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src,
        None,
    ).unwrap();

    let matrix = [
        [0.1, 0.0, 0.0, 0.0],
        [0.0, 0.1, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-0.5, -0.5, 0.0, 1.0],
    ];

    let gpu_buffer = glium::uniforms::UniformBuffer::<[[f32; 4]; BUF_LEN / 4]>::empty_persistent(&display).unwrap();
    let mut buffer = [ [0.0f32; 4]; BUF_LEN / 4];
    let mut mouse  = [0.0f32; 2];
    let mut closed = false;

    while !closed {
        let mut target      = display.draw();
        let (width, height) = target.get_dimensions();

        target.clear_color(0.0, 0.0, 1.0, 1.0);

        {
            let dequeue = audiosession.buffer.lock().unwrap();

            if let Some(xdata) = dequeue.back() {
                for i in 0..BUF_LEN {
                    buffer[i >> 2][i & 3] = xdata[i];
                }
            }
        }

        gpu_buffer.write(&buffer);
        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniform !{
                windowSize: [width as f32, height as f32],
                Buffer: &gpu_buffer,
                mouse: mouse
            },
            &Default::default(),
        ).unwrap();

        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                CloseRequested => closed = true,
                CursorMoved { position, .. } => {
                    mouse = [
                        hidpi_factor as f32 * position.x as f32 / width as f32,
                        1f32 - hidpi_factor as f32 * position.y as f32 / height as f32
                    ];
                }
                _ => (),
            },
            _ => ()
        });
    }
}
