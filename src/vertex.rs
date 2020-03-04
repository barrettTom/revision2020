use luminance_derive::{Semantics, Vertex};

#[derive(Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    pub position: VertexPosition,
    #[vertex(normalized = "true")]
    pub color: VertexRGB,
}

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
}
