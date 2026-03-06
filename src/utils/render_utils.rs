use wgpu::{BufferAddress, VertexBufferLayout};

#[must_use]
pub const fn desc(attr: &'_ [wgpu::VertexAttribute], size: BufferAddress) -> VertexBufferLayout<'_> {
    VertexBufferLayout {
        array_stride: size,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: attr,
    }
}
