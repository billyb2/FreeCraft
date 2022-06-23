use glam::{Vec3, Vec3A, Mat4};
use wgpu::SurfaceConfiguration;

pub struct Camera {
    pos: Vec3A,
    target: Vec3A,
    up_axis: Vec3A,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,

}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        // Moves world to be at pos and rot of cam
        let view = Mat4::look_at_rh(self.pos.into(), self.target.into(), self.up_axis.into());
        // Adds depth by transforming vertices in a way that makes them smamller or larger
        // depending on distance
        let proj  = Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);

        proj * view

    }
    
    pub fn new() -> Self {
        Self {
            pos: (800.0, 801.0, 52.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up_axis: Vec3A::Y,
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }

    }

    pub fn update_aspect(&mut self, config: &SurfaceConfiguration) {
        self.aspect = config.width as f32 / config.height as f32;

    }

    pub fn move_forward(&mut self, speed: f32) {
        let forward = self.target - self.pos;
        let forward_norm = forward.normalize_or_zero();
        // Equivalent of cgmath magnitude

        self.pos += forward_norm * speed;

    }

    pub fn move_backward(&mut self, speed: f32) {
        let forward = self.target - self.pos;
        let forward_norm = forward.normalize();
        // Equivalent of cgmath magnitude

        self.pos -= forward_norm * speed;

    }

    pub fn move_right(&mut self, speed: f32) {
        let forward = self.target - self.pos;
        let forward_norm = forward.normalize_or_zero();
        let forward_mag = forward.dot(forward).sqrt(); 

        let right = forward_norm.cross(self.up_axis);

        //self.pos = self.target - (forward - right * speed).normalize() * forward_mag;
        self.pos.z -= speed;
    }

    pub fn move_left(&mut self, speed: f32) {
        let forward = self.target - self.pos;
        let forward_norm = forward.normalize();
        let forward_mag = forward.dot(forward).sqrt(); 

        let right = forward_norm.cross(self.up_axis);

        self.pos = self.target - (forward + right * speed).normalize() * forward_mag;
    }
    
    pub fn move_up(&mut self, speed: f32) {
        self.pos.y -= speed;

    }

    pub fn move_down(&mut self, speed: f32) {
        self.pos.y += speed;

    }
    
    pub fn pos(&self) -> Vec3 {
        self.pos.into()

    }

}


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [f32; 16],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array(),

        }

    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array();

    }

    pub fn view_proj(&self) -> &[f32; 16] {
        &self.view_proj

    }

}
