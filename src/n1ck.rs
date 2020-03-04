use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, Tess, TessBuilder, TessSliceIndex};
use luminance::texture::Dim2;

use crate::vertex::{Vertex, VertexPosition, VertexRGB, VertexSemantics};

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
        if vertex.color[1] < 255 {
            vertex.color[1] += 1;
        }
    }
}

#[derive(Default)]
pub struct N1ck {
    vertices: Vec<Vertex>,
    tessalations: Vec<Tess>,
}

impl N1ck {
    pub fn new() -> N1ck {
        let vertices = gen_vertices();
        let tessalations = Vec::new();

        N1ck {
            vertices,
            tessalations,
        }
    }

    pub fn update<T: GraphicsContext>(&mut self, mut surface: T) -> T {
        alter_vertices(&mut self.vertices);

        self.tessalations.clear();
        self.tessalations.push(
            TessBuilder::new(&mut surface)
                .add_vertices(&self.vertices)
                .set_mode(Mode::Triangle)
                .build()
                .unwrap(),
        );

        surface
    }

    pub fn draw<T: GraphicsContext>(
        &self,
        mut surface: T,
        back_buffer: &Framebuffer<Dim2, (), ()>,
        program: &Program<VertexSemantics, (), ()>,
        pipeline_state: &PipelineState,
    ) -> T {
        surface.pipeline_builder().pipeline(
            &back_buffer,
            &pipeline_state,
            |_pipeline, mut shd_gate| {
                shd_gate.shade(&program, |_, mut rdr_gate| {
                    rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                        for tessalation in self.tessalations.iter() {
                            tess_gate.render(tessalation.slice(..));
                        }
                    });
                });
            },
        );

        surface
    }
}
