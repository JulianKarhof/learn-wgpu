use winit::dpi::PhysicalPosition;

pub struct Camera {
    pub width: f32,
    pub height: f32,
    pub offset: PhysicalPosition<f32>,
    pub zoom: f32,
    pub weight: Weight,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Weight {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub kjell: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let top = self.offset.y;
        let bottom = self.offset.y + self.height;
        let left = self.offset.x;
        let right = self.offset.x + self.width;

        let proj = cgmath::ortho(
            left + ((right - left) * self.weight.left) * self.zoom,
            right - ((right - left) * self.weight.right) * self.zoom,
            bottom - ((bottom - top) * self.weight.kjell) * self.zoom,
            top + ((bottom - top) * self.weight.top) * self.zoom,
            2.0,
            0.0,
        );

        return proj;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
