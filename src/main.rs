pub mod audio;
pub mod constants;
pub mod n1ck;
pub mod tom;
pub mod vertex;

use alto::Source;
use luminance::context::GraphicsContext;
use luminance::pipeline::{PipelineState, Viewport};
use luminance::shader::program::Program;
use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowEvent, WindowOpt};

use n1ck::N1ck;
use tom::Tom;
use vertex::VertexSemantics;

fn gen_viewports() -> Vec<Viewport> {
    let mut viewports = Vec::new();

    viewports.push(Viewport::Specific {
        x: 0,
        y: 0,
        width: constants::WIDTH / 2,
        height: constants::HEIGHT / 2,
    });

    viewports.push(Viewport::Specific {
        x: 0,
        y: constants::HEIGHT / 2,
        width: constants::WIDTH / 2,
        height: constants::HEIGHT / 2,
    });

    viewports.push(Viewport::Specific {
        x: constants::WIDTH / 2,
        y: 0,
        width: constants::WIDTH / 2,
        height: constants::HEIGHT / 2,
    });

    viewports.push(Viewport::Specific {
        x: constants::WIDTH / 2,
        y: constants::HEIGHT / 2,
        width: constants::WIDTH / 2,
        height: constants::HEIGHT / 2,
    });

    viewports
}

fn main() {
    let mut surface = GlfwSurface::new(
        WindowDim::Windowed(constants::WIDTH, constants::HEIGHT),
        "revision2020",
        WindowOpt::default(),
    )
    .unwrap();

    let program: Program<VertexSemantics, (), ()> =
        Program::from_strings(None, include_str!("vs.glsl"), None, include_str!("fs.glsl"))
            .unwrap()
            .ignore_warnings();

    let (mut stream, toms_samples) = audio::init();
    stream.play();

    let mut tom = Tom::new(toms_samples);
    let mut n1ck = N1ck::new();

    let viewports = gen_viewports();

    let mut run = true;
    while run {
        for event in surface.poll_events() {
            if let WindowEvent::Close = event {
                run = false;
            }
        }

        surface = tom.update(surface);
        surface = n1ck.update(surface);

        let back_buffer = surface.back_buffer().unwrap();
        let mut pipeline_state = PipelineState::default();
        surface
            .pipeline_builder()
            .pipeline(&back_buffer, &pipeline_state, |_, _| {});

        for (i, viewport) in viewports.iter().enumerate() {
            pipeline_state = pipeline_state.set_viewport(*viewport);
            pipeline_state = pipeline_state.enable_clear_color(false);

            if i == 0 || i == 3 {
                surface = tom.draw(surface, &back_buffer, &program, &pipeline_state);
            } else if i == 1 || i == 2 {
                surface = n1ck.draw(surface, &back_buffer, &program, &pipeline_state);
            }
        }

        surface.swap_buffers();
    }
}
