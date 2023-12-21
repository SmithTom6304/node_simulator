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

impl Position {
    pub fn distance_to(&self, other: &Position) -> cgmath::Vector3<f32> {
        (self.0 - other.0).map(|n| n.abs())
    }
}

#[cfg(test)]
mod a_position {
    use rstest::rstest;

    use crate::node::Position;

    #[test]
    fn default_is_0_0_0() {
        assert_eq!(
            cgmath::Point3::<f32>::new(0.0, 0.0, 0.0),
            Position::default().0
        )
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 0.0, 0.0))]
    #[case((0.0, 0.0, 0.0), (-1.0, 0.0, 0.0), (1.0, 0.0, 0.0))]
    #[case((1.0, 0.0, 0.0), (0.0, 0.0, 0.0), (1.0, 0.0, 0.0))]
    #[case((-1.0, 0.0, 0.0), (0.0, 0.0, 0.0), (1.0, 0.0, 0.0))]
    #[case((1.0, 0.0, 0.0), (-1.0, 0.0, 0.0), (2.0, 0.0, 0.0))]
    #[case((-1.0, 0.0, 0.0), (1.0, 0.0, 0.0), (2.0, 0.0, 0.0))]
    fn can_calculate_distance_to_another_position(
        #[case] position: (f32, f32, f32),
        #[case] other: (f32, f32, f32),
        #[case] expected_distance: (f32, f32, f32),
    ) {
        let (x, y, z) = position;
        let position = Position(cgmath::Point3 { x, y, z });
        let (x, y, z) = other;
        let other = Position(cgmath::Point3 { x, y, z });
        let (x, y, z) = expected_distance;
        let expected_distance = cgmath::Vector3::<f32>::new(x, y, z);
        assert_eq!(position.distance_to(&other), expected_distance);
    }
}
