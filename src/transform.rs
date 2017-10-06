use cgmath::{Point2, Rad, Vector2};

pub struct Transform {
    pub position: Point2<f64>,
    pub rotation: Rad<f64>,
    pub scale: Vector2<f64>,
}
