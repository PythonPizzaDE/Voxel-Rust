use cgmath::prelude::*;

use crate::ogl;

fn euler_to_vector(pitch: f32, yaw: f32) -> cgmath::Vector3<f32> {
    let mut vector = cgmath::vec3(0f32, 0f32, 0f32);
    const HALF: f32 = std::f32::consts::PI / 180f32;
    vector.x = (yaw * HALF).cos() * (pitch * HALF).cos();
    vector.y = -((pitch * HALF).sin());
    vector.z = (yaw * HALF).sin() * (pitch * HALF).cos();
    vector.normalize()
}

pub struct Camera {
    pub speed: f32,
    pub mouse_sensitivity: f32,

    fov: cgmath::Rad<f32>,
    pitch: f32,
    yaw: f32,
    last_mouse_x: f32,
    last_mouse_y: f32,

    position: cgmath::Point3<f32>,
    front: cgmath::Vector3<f32>,
    up: cgmath::Vector3<f32>,

    view_matrix: cgmath::Matrix4<f32>,
    projection_matrix: cgmath::Matrix4<f32>,

}

impl Camera {
    pub fn new(position: cgmath::Point3<f32>, pitch: f32, yaw: f32, up: cgmath::Vector3<f32>, speed: f32, mouse_sensitivity: f32, fov: f32) -> Self {
        let front = euler_to_vector(pitch, yaw);

        Camera {
            speed: speed,
            mouse_sensitivity: mouse_sensitivity,
            fov: cgmath::Rad((fov / 180f32) * std::f32::consts::PI),
            pitch: pitch,
            yaw: yaw,
            last_mouse_x: (1280 / 2) as f32,
            last_mouse_y: (720 / 2) as f32,
            position: position,
            front: front,
            up: up,
            view_matrix: cgmath::Matrix4::from_scale(1f32),
            projection_matrix: cgmath::Matrix4::from_scale(1f32),
        }
    }

    pub fn update(&mut self, shader: &mut ogl::shader::Shader, window: &glfw::Window, delta: f32, mouse_x: f32, mouse_y: f32) {
        if [glfw::Action::Press, glfw::Action::Repeat].contains(&window.get_key(glfw::Key::W)) {
            self.position += self.front * self.speed * delta;
        }

        if [glfw::Action::Press, glfw::Action::Repeat].contains(&window.get_key(glfw::Key::S)) {
            self.position -= self.front * self.speed * delta;
        }

        if [glfw::Action::Press, glfw::Action::Repeat].contains(&window.get_key(glfw::Key::A)) {
            self.position -= self.front.cross(self.up).normalize() * self.speed * delta;
        }

        if [glfw::Action::Press, glfw::Action::Repeat].contains(&window.get_key(glfw::Key::D)) {
            self.position += self.front.cross(self.up).normalize() * self.speed * delta;
        }

        if [glfw::Action::Press, glfw::Action::Repeat].contains(&window.get_key(glfw::Key::Space)) {
            self.position += self.up * self.speed * delta;
        }

        if [glfw::Action::Press, glfw::Action::Repeat].contains(&window.get_key(glfw::Key::LeftShift)) {
            self.position -= self.up * self.speed * delta;
        }

        self.update_mouse(mouse_x, mouse_y);

        let look_at_point = self.position + self.front;
        self.view_matrix = cgmath::Matrix4::look_at_rh(self.position, look_at_point, self.up);
        self.projection_matrix = cgmath::perspective(self.fov, 1280f32 / 720f32, 0.1f32, 100f32);
        // self.projection_matrix = cgmath::ortho(0f32, 1280f32, 0f32, 720f32, 0.1f32, 100f32);

        shader.set_matrix4_uniform("view", &self.view_matrix);
        shader.set_matrix4_uniform("projection", &self.projection_matrix);
    }

    fn update_mouse(&mut self, mouse_x: f32, mouse_y: f32) {
        let x_offset = mouse_x - self.last_mouse_x;
        let y_offset = mouse_y - self.last_mouse_y;


        self.last_mouse_x = mouse_x;
        self.last_mouse_y = mouse_y;

        self.yaw += x_offset * self.mouse_sensitivity;
        self.pitch += y_offset * self.mouse_sensitivity;


        if self.pitch > 89f32 {
            self.pitch = 89f32;
        } else if self.pitch < -89f32 {
            self.pitch = -89f32;
        }

        self.front = euler_to_vector(self.pitch, self.yaw);
        println!("pitch={} | yaw={}", self.pitch, self.yaw);
    }
}
