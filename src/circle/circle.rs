use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Circle {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub radius: f32,
    pub border: f32,
    pub border_color: [f32; 4],
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            radius: 50.0,
            border: 0.0,
            border_color: [1.0; 4],
        }
    }
}

impl Circle {
    pub fn attributes() -> [wgpu::VertexAttribute; 5] {
        vertex_attr_array!(1 => Float32x2, 2 => Float32x4, 3 => Float32, 4 => Float32, 5 => Float32x4)
    }

    pub fn layout(attributes: &[wgpu::VertexAttribute]) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Circle>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes,
        }
    }
}
