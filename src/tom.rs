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

fn relative(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

#[derive(Default)]
pub struct Tom {
    border: Vec<Vec<Vertex>>,
    wave: Vec<Vertex>,
    last_x: usize,
    waveform: Vec<i16>,
    tessalations: Vec<Tess>,
}

impl Tom {
    pub fn new(waveform: Vec<i16>) -> Tom {
        Tom {
            waveform,
            border: gen_border(),
            wave: Vec::new(),
            last_x: 0,
            tessalations: Vec::new(),
        }
    }

    pub fn update<T: GraphicsContext>(&mut self, mut surface: T) -> T {
        self.tessalations.clear();

        self.update_wave();

        for vertices in self.border.iter() {
            self.tessalations.push(
                TessBuilder::new(&mut surface)
                    .add_vertices(vertices)
                    .set_mode(Mode::TriangleFan)
                    .build()
                    .unwrap(),
            );
        }

        self.tessalations.push(
            TessBuilder::new(&mut surface)
                .add_vertices(&self.wave)
                .set_mode(Mode::LineStrip)
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

    fn update_wave(&mut self) {
        self.wave.clear();

        let start = self.last_x;
        let end = start + 3000;

        let mut positions = Vec::new();
        for x in start..end {
            positions.push(VertexPosition::new([x as f32, self.waveform[x] as f32]));
        }

        for position in positions {
            let color = match position[1].abs() as u32 {
                0..=50 => VertexRGB::new(constants::C64_BLUE),
                51..=125 => VertexRGB::new(constants::C64_LIGHT_BLUE),
                126..=225 => VertexRGB::new(constants::C64_CYAN),
                _ => VertexRGB::new(constants::C64_GREEN),
            };

            self.wave.push(Vertex { position, color });
        }

        let max_y = *self.waveform.iter().max().unwrap() as f32;
        let min_y = *self.waveform.iter().min().unwrap() as f32;
        for vertex in self.wave.iter_mut() {
            vertex.position[0] = relative(vertex.position[0], start as f32, end as f32, -0.9, 0.9);
            vertex.position[1] = relative(vertex.position[1], max_y, min_y, -0.9, 0.9);
        }

        self.wave.reverse();

        self.last_x += constants::SAMPLE_RATE / 60;
    }
}
