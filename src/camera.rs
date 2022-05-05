use winit::dpi::PhysicalPosition;

pub struct Camera {
    pub width: f32,
    pub height: f32,
    pub offset: PhysicalPosition<f32>,
    pub zoom: f32,
    pub weight: Weight,
}

pub struct Weight {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub kjell: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let proj = cgmath::ortho(
            0.0 * self.zoom * self.weight.left + self.offset.x * self.zoom,
            self.width * self.zoom * self.weight.right + self.offset.x * self.zoom,
            self.height * self.zoom * self.weight.top + self.offset.y * self.zoom,
            0.0 * self.zoom * self.weight.kjell + self.offset.y * self.zoom,
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
