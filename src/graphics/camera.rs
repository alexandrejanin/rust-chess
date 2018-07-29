use maths::{Matrix4f, Vector2u, Vector3f};

pub trait Camera {
    fn matrix(&self) -> Matrix4f;
}

///A camera with an orthographic projection.
pub struct OrthographicCamera {
    ///Position of the camera in world space.
    position: Vector3f,
    ///Size of the camera in world space.
    size: Vector3f,
}

impl OrthographicCamera {
    pub fn from_width(position: Vector3f, width: f32, depth: f32, screen_size: Vector2u) -> Self {
        let ratio = screen_size.y as f32 / screen_size.x as f32;
        Self {
            position,
            size: Vector3f::new(width, ratio * width, depth),
        }
    }

    pub fn from_height(position: Vector3f, height: f32, depth: f32, screen_size: Vector2u) -> Self {
        let ratio = screen_size.x as f32 / screen_size.y as f32;
        Self {
            position,
            size: Vector3f::new(ratio * height, height, depth),
        }
    }

    pub fn resize_keep_width(&mut self, screen_size: Vector2u) {
        let ratio = screen_size.y as f32 / screen_size.x as f32;
        self.size.y = ratio * self.size.x;
    }

    pub fn resize_keep_height(&mut self, screen_size: Vector2u) {
        let ratio = screen_size.x as f32 / screen_size.y as f32;
        self.size.x = ratio * self.size.y;
    }
}

impl Camera for OrthographicCamera {
    fn matrix(&self) -> Matrix4f {
        //Get clipping planes
        let left = self.position.x - self.size.x / 2.0;
        let right = self.position.x + self.size.x / 2.0;

        let bottom = self.position.y - self.size.y / 2.0;
        let top = self.position.y + self.size.y / 2.0;

        let near = self.position.z;
        let far = self.position.z + self.size.z;

        //Initialize matrix
        let mut mat = Matrix4f::identity();

        //Set the projection values
        mat[(0, 0)] = 2.0 / (right - left);
        mat[(1, 1)] = 2.0 / (top - bottom);
        mat[(2, 2)] = -2.0 / (far - near);

        mat[(3, 0)] = -(right + left) / (right - left);
        mat[(3, 1)] = -(top + bottom) / (top - bottom);
        mat[(3, 2)] = -(far + near) / (far - near);

        mat
    }
}
