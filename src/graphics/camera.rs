use cgmath::{
    self,
    Ortho,
    PerspectiveFov,
};
use maths::{Matrix4f, Point3f, Vector2f, Vector2u, Vector3f};

//Orthographic camera

pub struct Camera {
    ///Position of the camera in world space.
    pub position: Point3f,
    ///Camera's rotation quaternion.
    pub direction: Vector3f,
    ///Near plane distance.
    pub near: f32,
    ///Far plane distance.
    pub far: f32,

    ///Size if orthographic, FOV if perspective.
    size: Vector2f,

    ///Whether the camera is perspective or orthographic.
    pub perspective: bool,
}

impl Camera {
    pub fn from_width(position: Point3f, direction: Vector3f, perspective: bool, near: f32, far: f32, width: f32, screen_size: Vector2u) -> Self {
        let ratio = screen_size.y as f32 / screen_size.x as f32;
        let height = ratio * width;

        Self {
            position,
            direction,
            near,
            far,
            size: Vector2f::new(width, height),
            perspective
        }
    }

    pub fn from_height(position: Point3f, direction: Vector3f, perspective: bool, near: f32, far: f32, height: f32, screen_size: Vector2u) -> Self {
        let ratio = screen_size.x as f32 / screen_size.y as f32;
        let width = ratio * height;

        Self {
            position,
            direction,
            near,
            far,
            size: Vector2f::new(width, height),
            perspective
        }
    }

    pub fn look_at(&mut self, target: Point3f) {
        self.direction = target - self.position;
    }

    pub fn matrix(&self) -> Matrix4f {
        self.proj_matrix() * self.view_matrix()
    }

    fn proj_matrix(&self) -> Matrix4f {
        match self.perspective {
            true => PerspectiveFov {
                fovy: cgmath::Deg(self.size.y).into(),
                near: self.near,
                far: self.far,
                aspect: self.size.x / self.size.y,
            }.into(),

            false => Ortho {
                left: -self.size.x / 2.0,
                right: self.size.x / 2.0,
                bottom: -self.size.y / 2.0,
                top: self.size.y / 2.0,
                near: self.near,
                far: self.far,
            }.into()
        }
    }

    fn view_matrix(&self) -> Matrix4f {
        Matrix4f::look_at_dir(
            self.position,
            self.direction,
            Vector3f::new(0.0, 1.0, 0.0),
        )
    }
}
