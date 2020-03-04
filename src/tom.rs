use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, Tess, TessBuilder, TessSliceIndex};
use luminance::texture::Dim2;

use crate::constants;
use crate::vertex::{Vertex, VertexPosition, VertexRGB, VertexSemantics};

fn gen_rectangle(x1: f32, y1: f32, x2: f32, y2: f32, color: [u8; 3]) -> Vec<Vertex> {
    let mut vertices: Vec<Vertex> = Vec::new();

    vertices.push(Vertex {
        position: VertexPosition::new([x1, y1]),
        color: VertexRGB::new(color),
    });

    vertices.push(Vertex {
        position: VertexPosition::new([x1, y2]),
        color: VertexRGB::new(color),
    });

    vertices.push(Vertex {
        position: VertexPosition::new([x2, y2]),
        color: VertexRGB::new(color),
    });

    vertices.push(Vertex {
        position: VertexPosition::new([x2, y1]),
        color: VertexRGB::new(color),
    });

    vertices
}

fn gen_border() -> Vec<Vec<Vertex>> {
    let mut vertices: Vec<Vec<Vertex>> = Vec::new();

    vertices.push(gen_rectangle(-1.0, -1.0, 1.0, -0.9, constants::C64_RED));

    vertices.push(gen_rectangle(-1.0, -1.0, -0.9, 1.0, constants::C64_GREEN));

    vertices.push(gen_rectangle(-1.0, 1.0, 1.0, 0.9, constants::C64_BLUE));

    vertices.push(gen_rectangle(0.9, 1.0, 1.0, -1.0, constants::C64_VIOLET));

    vertices
}

#[derive(Default)]
pub struct Tom {
    vertices: Vec<Vec<Vertex>>,
    tessalations: Vec<Tess>,
}

impl Tom {
    pub fn new() -> Tom {
        let vertices = gen_border();
        let tessalations = Vec::new();

        Tom {
            vertices,
            tessalations,
        }
    }

    pub fn update<T: GraphicsContext>(&mut self, mut surface: T) -> T {
        self.tessalations.clear();
        for vertices in self.vertices.iter() {
            self.tessalations.push(
                TessBuilder::new(&mut surface)
                    .add_vertices(vertices)
                    .set_mode(Mode::TriangleFan)
                    .build()
                    .unwrap(),
            );
        }

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
