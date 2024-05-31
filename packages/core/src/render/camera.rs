use std::{f32::EPSILON, time::Duration};

use crate::input::GameCommand;

#[derive(Debug)]
pub struct Camera {
    position: glam::Vec3,
    yaw: f32,
    pitch: f32,

    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,

    front: glam::Vec3,
    right: glam::Vec3,
    up: glam::Vec3,
    forward: glam::Vec3,

    view_mat: glam::Mat4,
    projection_mat: glam::Mat4,
    view_projection_mat: glam::Mat4,
}

impl Camera {
    pub fn new(position: glam::Vec3, fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let yaw: f32 = 0.0;
        let pitch: f32 = 0.0;

        let view_mat = glam::Mat4::look_at_rh(position, position + glam::Vec3::Z, glam::Vec3::Y);
        let projection_mat = glam::Mat4::perspective_rh(fov, aspect, near, far);
        let view_projection_mat = projection_mat * view_mat;

        let front = glam::Vec3::new(
            yaw.to_radians().cos() * pitch.to_radians().cos(),
            pitch.to_radians().sin(),
            yaw.to_radians().sin() * pitch.to_radians().cos(),
        )
        .normalize();
        let right = front.cross(glam::Vec3::Y).normalize();
        let up = right.cross(front).normalize();
        let forward = glam::Vec3::Y.cross(right).normalize();

        Self {
            position,
            yaw,
            pitch,
            fov,
            aspect,
            near,
            far,
            front,
            right,
            up,
            forward,
            view_mat,
            projection_mat,
            view_projection_mat,
        }
    }

    pub fn tick(&mut self, delta_time: Duration, game_command: u32, cursor_delta: glam::Vec2) {
        // println!("{:?} {:?}", self.position, delta_time.as_secs_f32());
        let velocity = 1.0 * delta_time.as_secs_f32();
        if game_command & GameCommand::Forward as u32 != 0 {
            self.position += self.forward * velocity;
        }
        if game_command & GameCommand::Backward as u32 != 0 {
            self.position -= self.forward * velocity;
        }
        if game_command & GameCommand::Left as u32 != 0 {
            self.position -= self.right * velocity;
        }
        if game_command & GameCommand::Right as u32 != 0 {
            self.position += self.right * velocity;
        }
        if game_command & GameCommand::Up as u32 != 0 {
            self.position += glam::Vec3::Y * velocity;
        }
        if game_command & GameCommand::Down as u32 != 0 {
            self.position -= glam::Vec3::Y * velocity;
        }

        let mouse_sensitivity = 0.3;
        if cursor_delta.x.abs() > EPSILON || cursor_delta.y.abs() > EPSILON {
            self.yaw += cursor_delta.x * mouse_sensitivity;
            self.pitch += -1.0 * cursor_delta.y * mouse_sensitivity;
            self.pitch = self.pitch.clamp(-89.0, 89.0);
        }
        self.update_view_mat();
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        self.aspect = ratio;
        self.projection_mat =
            glam::Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far);
        self.view_projection_mat = self.projection_mat * self.view_mat;
    }

    pub fn set_position(&mut self, position: glam::Vec3) {
        self.position = position;
        self.update_view_mat();
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.update_view_mat();
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
        self.update_view_mat();
    }

    pub fn update_view_mat(&mut self) {
        let front = glam::Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();
        let right = front.cross(glam::Vec3::Y).normalize();
        let up = right.cross(front).normalize();
        let forward = glam::Vec3::Y.cross(right).normalize();

        self.front = front;
        self.right = right;
        self.up = up;
        self.forward = forward;

        self.view_mat = glam::Mat4::look_at_rh(self.position, self.position + front, up);
        self.view_projection_mat = self.projection_mat * self.view_mat;
    }

    pub fn view_projection_mat(&self) -> glam::Mat4 {
        self.view_projection_mat
    }
}
