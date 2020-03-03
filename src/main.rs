pub mod constants;
pub mod tom;

use luminance::context::GraphicsContext;
use luminance::pipeline::{PipelineState, Viewport};
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, TessBuilder, TessSliceIndex};
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowEvent, WindowOpt};
use tom::{Tom, VertexSemantics};

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

    let mut tom = Tom::new();
    let viewports = gen_viewports();

    let mut run = true;
    while run {
        for event in surface.poll_events() {
            if let WindowEvent::Close = event {
                run = false;
            }
        }
        let back_buffer = surface.back_buffer().unwrap();

        tom.update();

        let mut pipeline_state = PipelineState::default();
        surface
            .pipeline_builder()
            .pipeline(&back_buffer, &pipeline_state, |_, _| {});

        for (i, viewport) in viewports.iter().enumerate() {
            pipeline_state = pipeline_state.set_viewport(*viewport);
            pipeline_state = pipeline_state.enable_clear_color(false);

            if i == 0 {
                tom.draw();
            } else if i == 1 {
                // lowman.draw()
            }
        }

        surface.swap_buffers();
    }
}
