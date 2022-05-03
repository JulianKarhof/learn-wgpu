use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Rect {
    pub position: [f32; 2],
    pub rotation: f32,
    pub color: [f32; 4],
    pub size: [f32; 2],
    pub border_radius: [f32; 4],
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            rotation: 0.0,
            color: [1.0, 1.0, 1.0, 1.0],
            size: [100.0; 2],
            border_radius: [0.0; 4],
        }
    }
}

impl Rect {
    pub fn attributes() -> [wgpu::VertexAttribute; 5] {
        vertex_attr_array!(1 => Float32x2, 2 => Float32, 3 => Float32x4, 4 => Float32x2, 5 => Float32x4)
    }

    pub fn layout(attributes: &[wgpu::VertexAttribute]) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Rect>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes,
        }
    }
}
