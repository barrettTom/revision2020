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

pub struct Tom {
    vertices: Vec<Vertex>,
    //tessalation: Tess,
}

impl Tom {
    pub fn new() -> Tom {
        let mut vertices = gen_vertices();
        //let tessalation = TessBuilder::new(&mut surface)
        //    .add_vertices(&vertices)
        //    .set_mode(Mode::Triangle)
        //    .build()
        //    .unwrap();
        Tom {
            vertices,
            //    tessalation,
        }
    }

    pub fn update(&mut self) {
        alter_vertices(&mut self.vertices);
        //let tessalation = TessBuilder::new(&mut surface)
        //    .add_vertices(&self.vertices)
        //    .set_mode(Mode::Triangle)
        //    .build()
        //    .unwrap();
    }

    pub fn draw(&self) {
        /*
        surface.pipeline_builder().pipeline(
            &back_buffer,
            &pipeline_state,
            |_pipeline, mut shd_gate| {
                shd_gate.shade(&program, |_, mut rdr_gate| {
                    rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                        //tess_gate.render(triangle.slice(..));
                    });
                });
            },
        );
        */
    }
}
