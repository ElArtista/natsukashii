//
// camera.rs
//

use glam::{Mat4, Quat, Vec2, Vec3};
use std::f32::consts::PI;

/// Camera
///
/// Temporal properties use world units/sec as a unit system
#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
    pub angles: Vec3,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub acceleration: f32,
    pub max_velocity: f32,
    pub angular_acceleration: f32,
    pub max_angular_velocity: f32,
}

#[derive(Copy, Clone, Debug)]
pub enum CameraMoveDirection {
    Forward,
    Left,
    Backward,
    Right,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            acceleration: 50.0,
            angular_acceleration: 6.0 * PI,
            max_velocity: 5.0,
            max_angular_velocity: 2.0 * PI,
            ..Default::default()
        }
    }

    pub fn move_to(&mut self, directions: &[CameraMoveDirection], dt: f32) {
        let rotation = self.rotation;
        let v = Mat4::from_quat(rotation);
        let forward = v.z_axis.truncate();
        let strafe = v.x_axis.truncate();

        let (mut dx, mut dz) = (0.0, 0.0);
        for d in directions {
            match d {
                CameraMoveDirection::Forward => dz += 1.0,
                CameraMoveDirection::Left => dx -= 1.0,
                CameraMoveDirection::Backward => dz -= 1.0,
                CameraMoveDirection::Right => dx += 1.0,
            }
        }

        dx *= self.acceleration * dt;
        dz *= self.acceleration * dt;
        self.velocity += (dz * forward) + (dx * strafe);
        self.velocity = self.velocity.clamp_length(0.0, self.max_velocity);
    }

    pub fn look(&mut self, offset: Vec2, dt: f32) {
        // Assume that offset of 1024px is around pi/4 rads
        let dp = (PI / 4.0) * (offset / 1024.0);
        self.angular_velocity.x += self.angular_acceleration * dp.y * dt;
        self.angular_velocity.y += self.angular_acceleration * dp.x * dt;
        self.angular_velocity = self
            .angular_velocity
            .clamp_length(0.0, self.max_angular_velocity);
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn update(&mut self, dt: f32) {
        // Movement
        self.position += self.velocity * dt;
        self.velocity *= (1e-4 as f32).powf(dt);

        // Rotation
        self.angles += self.angular_velocity;
        let qp = Quat::from_rotation_x(self.angles.x);
        let qy = Quat::from_rotation_y(self.angles.y);
        self.rotation = (qy * qp).normalize();
        self.angular_velocity *= (1e-4 as f32).powf(dt);
    }

    pub fn matrix(&self) -> Mat4 {
        let t = Mat4::from_translation(-self.position);
        let r = Mat4::from_quat(self.rotation.conjugate());
        r * t
    }
}
