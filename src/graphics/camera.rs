use maths::Matrix4f;

pub trait Camera {
    fn projection_matrix(&self) -> Matrix4f;
}

pub struct OrthographicCamera {
    pub screen_width: u32,
    pub screen_height: u32,

}

impl Camera for OrthographicCamera {
    fn projection_matrix(&self) -> Matrix4f {
        Matrix4f::identity()
    }
}
