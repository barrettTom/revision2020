pub mod constants;
use luminance::context::GraphicsContext;
use luminance::pipeline::{PipelineState, Viewport};
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, TessBuilder, TessSliceIndex};
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowEvent, WindowOpt};

#[derive(Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    position: VertexPosition,
    #[vertex(normalized = "true")]
    color: VertexRGB,
}

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
}

fn gen_vertices() -> Vec<Vertex> {
    let mut vertices: Vec<Vertex> = Vec::new();

    vertices.push(Vertex {
        position: VertexPosition::new([-0.5, -0.5]),
        color: VertexRGB::new([255, 0, 0]),
    });
    vertices.push(Vertex {
        position: VertexPosition::new([0.5, -0.5]),
        color: VertexRGB::new([0, 255, 0]),
    });
    vertices.push(Vertex {
        position: VertexPosition::new([0.0, 0.5]),
        color: VertexRGB::new([0, 0, 255]),
    });

    vertices
}

fn alter_vertices(vertices: &mut Vec<Vertex>) {
    for vertex in vertices {
        vertex.position[0] += 0.01;
        if vertex.color[1] < 255 {
            vertex.color[1] += 1;
        }
    }
}

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
        y: constants::WIDTH / 2,
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
        "art",
        WindowOpt::default(),
    )
    .unwrap();

    let program: Program<VertexSemantics, (), ()> =
        Program::from_strings(None, include_str!("vs.glsl"), None, include_str!("fs.glsl"))
            .unwrap()
            .ignore_warnings();

    let mut vertices = gen_vertices();
    let viewports = gen_viewports();

    let mut run = true;
    while run {
        for event in surface.poll_events() {
            if let WindowEvent::Close = event {
                run = false;
            }
        }
        let back_buffer = surface.back_buffer().unwrap();
        alter_vertices(&mut vertices);

        let triangle = TessBuilder::new(&mut surface)
            .add_vertices(&vertices)
            .set_mode(Mode::Triangle)
            .build()
            .unwrap();

        let mut pipeline_state = PipelineState::default();
        for viewport in viewports.iter() {
            pipeline_state = pipeline_state.set_viewport(*viewport);

            surface.pipeline_builder().pipeline(
                &back_buffer,
                &pipeline_state,
                |_pipeline, mut shd_gate| {
                    shd_gate.shade(&program, |_, mut rdr_gate| {
                        rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(triangle.slice(..));
                        });
                    });
                },
            );
        }

        surface.swap_buffers();
    }
}
