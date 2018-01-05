use piston_window::{polygon, math, Transformed, Graphics};

use transform::Transform;

pub struct Polygon {
    pub color: [f32; 4],
    pub polygon: &'static [[f64; 2]],
}

impl Polygon {
    pub fn render<G>(&self, transform: &Transform, graphics: &mut G)
        where G: Graphics {
        polygon(
            self.color,
            self.polygon,
            math::identity()
                .trans(transform.position.x, transform.position.y)
                .scale(transform.scale.x, transform.scale.y)
                .rot_rad(transform.rotation.0),
            graphics,
        );
    }
}
