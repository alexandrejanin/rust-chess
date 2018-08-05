use maths::{Matrix4f, Point3f, Vector3f};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform {
    pub position: Point3f,
    pub scale: Vector3f,
    pub rotation: Vector3f,
}

impl Transform {
    ///Create a transform with base position, scale, and rotation.
    pub fn new() -> Self {
        Self {
            position: Point3f::new(0.0, 0.0, 0.0),
            scale: Vector3f::new(1.0, 1.0, 1.0),
            rotation: Vector3f::new(0.0, 0.0, 0.0),
        }
    }

    ///Create a transform at set position, with base scale and rotation.
    pub fn from_position(position: Point3f) -> Self {
        Self {
            position,
            scale: Vector3f::new(1.0, 1.0, 1.0),
            rotation: Vector3f::new(0.0, 0.0, 0.0),
        }
    }

    ///Create a transform with set scale, with base position and rotation.
    pub fn from_scale(scale: Vector3f) -> Self {
        Self {
            position: Point3f::new(0.0, 0.0, 0.0),
            scale,
            rotation: Vector3f::new(0.0, 0.0, 0.0),
        }
    }

    ///Create a transform with set rotation, with base position and scale.
    pub fn from_rotation(rotation: Vector3f) -> Self {
        Self {
            position: Point3f::new(0.0, 0.0, 0.0),
            scale: Vector3f::new(1.0, 1.0, 1.0),
            rotation,
        }
    }

    ///Creates a matrix that applies selected transforms to a vector.
    pub fn matrix(&self) -> Matrix4f {
        Matrix4f::new(
            self.scale.x, 0., 0., 0.,
            0., self.scale.y, 0., 0.,
            0., 0., self.scale.z, 0.,
            self.position.x, self.position.y, self.position.z, 1.,
        )
    }
}
