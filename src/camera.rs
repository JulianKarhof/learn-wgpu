use cgmath::{vec2, vec4, Matrix4, SquareMatrix, Vector2};
use winit::dpi::PhysicalSize;

pub struct Camera {
    pub mouse_pos: Vector2<f32>,
    pub limits: Limits<f32>,
    pub zoom: f32,
    pub offset: Vector2<f32>,
}

pub struct Limits<T> {
    pub left: T,
    pub right: T,
    pub bottom: T,
    pub top: T,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let projection = cgmath::ortho(
            self.limits.left,
            self.limits.right,
            self.limits.bottom,
            self.limits.top,
            2.0,
            0.0,
        );

        return projection;
    }

    pub fn get_absolute_mouse_pos(
        &self,
        projection: Matrix4<f32>,
        window_size: PhysicalSize<u32>,
    ) -> Vector2<f32> {
        // Invert view projection matrix
        let inverted = projection.invert().unwrap();
        // Prepare mouse position vector
        let mouse = vec4(
            2.0 * self.mouse_pos.x / window_size.width as f32 - 1.0,
            -(2.0 * self.mouse_pos.y / window_size.height as f32 - 1.0),
            1.0,
            1.0,
        );

        // Multiply inverted matrix with screen space mouse position to get world spaces coordinates
        let worldspace_mouse_pos = inverted * mouse;

        return vec2(worldspace_mouse_pos.x, worldspace_mouse_pos.y);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }

    pub fn get_absolute_mouse_pos(
        &self,
        camera: &Camera,
        window_size: PhysicalSize<u32>,
    ) -> Vector2<f32> {
        return camera.get_absolute_mouse_pos(self.view_proj.into(), window_size);
    }
}
