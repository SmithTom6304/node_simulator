use std::{fmt, ops::Add};

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

impl From<(f32, f32, f32)> for Position {
    fn from(value: (f32, f32, f32)) -> Self {
        let (x, y, z) = value;
        Self::from(cgmath::Point3 { x, y, z })
    }
}

impl From<cgmath::Point3<f32>> for Position {
    fn from(value: cgmath::Point3<f32>) -> Self {
        Self(value)
    }
}

impl From<&cgmath::Point3<f32>> for Position {
    fn from(value: &cgmath::Point3<f32>) -> Self {
        Self(*value)
    }
}

impl TryFrom<String> for Position {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let pos_string = value.trim_matches('"').split(',');
        let positions: Vec<Result<f32, _>> = pos_string.map(|s| s.parse::<f32>()).collect();
        if positions.len() != 3 {
            return Err("Position must have 3 values".to_string());
        }
        let x = match &positions[0] {
            Ok(number) => *number,
            Err(_) => {
                return Err("Position x must be an f32".to_string());
            }
        };
        let y = match &positions[1] {
            Ok(number) => *number,
            Err(_) => {
                return Err("Position y must be an f32".to_string());
            }
        };
        let z = match &positions[2] {
            Ok(number) => *number,
            Err(_) => {
                return Err("Position z must be an f32".to_string());
            }
        };
        Ok(Self(cgmath::Point3 { x, y, z }))
    }
}

impl Into<(f32, f32, f32)> for Position {
    fn into(self) -> (f32, f32, f32) {
        self.0.into()
    }
}

impl Into<cgmath::Point3<f32>> for Position {
    fn into(self) -> cgmath::Point3<f32> {
        self.0.into()
    }
}

impl Into<cgmath::Point3<f32>> for &Position {
    fn into(self) -> cgmath::Point3<f32> {
        self.0.into()
    }
}

impl Add<cgmath::Vector3<f32>> for Position {
    type Output = Self;

    fn add(self, rhs: cgmath::Vector3<f32>) -> Self::Output {
        let point_3 = Into::<cgmath::Point3<f32>>::into(self) + rhs;
        Self::from(point_3)
    }
}

impl Add<super::Force> for Position {
    type Output = Self;

    fn add(self, rhs: super::Force) -> Self::Output {
        let point_3 =
            Into::<cgmath::Point3<f32>>::into(self) + Into::<cgmath::Vector3<f32>>::into(rhs);
        Self::from(point_3)
    }
}

impl Position {
    pub fn distance_to(&self, other: &Position) -> cgmath::Vector3<f32> {
        (self.0 - other.0).map(|n| n.abs())
    }

    pub fn displacement(&self, other: &Position) -> cgmath::Vector3<f32> {
        let other_point = Into::<cgmath::Point3<f32>>::into(other);
        let self_point = Into::<cgmath::Point3<f32>>::into(self);
        other_point - self_point
    }
}

#[cfg(test)]
mod a_position {
    use crate::node::{Force, Position};
    use rstest::rstest;

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

    #[test]
    fn can_be_displayed() {
        let position = Position(cgmath::Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let expected_display_string = format!(
            "x: {}, y: {}, z: {}",
            position.0.x, position.0.y, position.0.z
        );

        assert_eq!(expected_display_string, position.to_string())
    }

    #[test]
    fn can_create_from_tuple() {
        let tuple = (1.0, 2.0, 3.0);
        let expected_position = Position(cgmath::Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        assert_eq!(expected_position, Position::from(tuple))
    }

    #[test]
    fn can_create_from_point() {
        let point = cgmath::Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let expected_position = Position(cgmath::Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        assert_eq!(expected_position, Position::from(point))
    }

    #[test]
    fn can_turn_into_tuple() {
        let position = Position(cgmath::Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let expected_tuple = (1.0, 2.0, 3.0);
        assert_eq!(expected_tuple, position.into())
    }

    #[test]
    fn can_turn_into_point() {
        let position = Position(cgmath::Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let expected_point = cgmath::Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(expected_point, position.into())
    }

    #[test]
    fn can_add_a_vector() {
        let position = Position(cgmath::Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let vector_to_add = cgmath::Vector3 {
            x: 1.0,
            y: 3.0,
            z: 5.0,
        };
        let expected_result = Position(cgmath::Point3 {
            x: 2.0,
            y: 5.0,
            z: 8.0,
        });
        assert_eq!(expected_result, position + vector_to_add)
    }

    #[test]
    fn can_add_a_force() {
        let position = Position(cgmath::Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let force_to_add = Force(cgmath::Vector3 {
            x: 1.0,
            y: 3.0,
            z: 5.0,
        });
        let expected_result = Position(cgmath::Point3 {
            x: 2.0,
            y: 5.0,
            z: 8.0,
        });
        assert_eq!(expected_result, position + force_to_add)
    }
}
