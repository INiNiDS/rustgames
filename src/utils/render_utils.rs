use wgpu::{BufferAddress, VertexBufferLayout};

/// Helper function to create a `VertexBufferLayout` with the given attributes, stride, and step mode.
#[must_use]
pub const fn desc(
    attr: &'_ [wgpu::VertexAttribute],
    size: BufferAddress,
    step_mode: wgpu::VertexStepMode,
) -> VertexBufferLayout<'_> {
    VertexBufferLayout {
        array_stride: size,
        step_mode,
        attributes: attr,
    }
}
