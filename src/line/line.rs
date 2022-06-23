use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Line {
    pub position1: [f32; 2],
    pub position2: [f32; 2],
    pub color: [f32; 4],
    pub thiccness: f32,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            position1: [0.0, 0.0],
            position2: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            thiccness: 100.0,
        }
    }
}

impl Line {
    pub fn attributes() -> [wgpu::VertexAttribute; 4] {
        vertex_attr_array!(1 => Float32x2, 2 => Float32x2, 3 => Float32x4, 4 => Float32)
    }

    pub fn layout(attributes: &[wgpu::VertexAttribute]) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Line>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes,
        }
    }
}
