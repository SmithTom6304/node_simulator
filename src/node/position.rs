use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position(pub cgmath::Point3<f32>);

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.0.x, self.0.y, self.0.z)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self(cgmath::Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    }
}
