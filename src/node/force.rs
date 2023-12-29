use std::{
    fmt,
    iter::Sum,
    ops::{Add, AddAssign, Mul, Neg},
};

use cgmath::{self, InnerSpace, Zero};

use super::Position;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Force(pub cgmath::Vector3<f32>);

impl fmt::Display for Force {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.0.x, self.0.y, self.0.z)
    }
}

impl Sum for Force {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let summed_vector = iter.map(|f| f.0).sum();
        Self(summed_vector)
    }
}

impl Add for Force {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Force {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Neg for Force {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Mul<f32> for Force {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from(self.0.map(|n| n * rhs))
    }
}

impl From<(f32, f32, f32)> for Force {
    fn from(value: (f32, f32, f32)) -> Self {
        let (x, y, z) = value;
        Self::from(cgmath::Vector3 { x, y, z })
    }
}

impl From<cgmath::Vector3<f32>> for Force {
    fn from(value: cgmath::Vector3<f32>) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for Force {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let force_string = value.trim_matches('"').split(',');
        let positions: Vec<Result<f32, _>> =
            force_string.map(|s| s.trim().parse::<f32>()).collect();
        if positions.len() != 3 {
            return Err("Force must have 3 values".to_string());
        }
        let x = match &positions[0] {
            Ok(number) => *number,
            Err(_) => {
                return Err("Force vector x must be an f32".to_string());
            }
        };
        let y = match &positions[1] {
            Ok(number) => *number,
            Err(_) => {
                return Err("Force vector y must be an f32".to_string());
            }
        };
        let z = match &positions[2] {
            Ok(number) => *number,
            Err(_) => {
                return Err("Force vector z must be an f32".to_string());
            }
        };
        Ok(Self(cgmath::Vector3 { x, y, z }))
    }
}

impl Into<(f32, f32, f32)> for Force {
    fn into(self) -> (f32, f32, f32) {
        self.0.into()
    }
}

impl Into<cgmath::Vector3<f32>> for Force {
    fn into(self) -> cgmath::Vector3<f32> {
        self.0.into()
    }
}

impl PartialOrd for Force {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.magnitude().partial_cmp(&other.magnitude())
    }
}

impl Force {
    const FORCE_RADIUS: u32 = 5;
    pub fn zero() -> Self {
        Self(cgmath::Vector3::zero())
    }

    pub fn calculate_incoming_force(
        node: &super::Node,
        others: &Vec<&super::Node>,
        default_gravitational_constant: &f32,
    ) -> Self {
        let resultant_force = others
            .iter()
            .map(|other| {
                Self::calculate_incoming_force_from_node(
                    node,
                    other,
                    default_gravitational_constant,
                )
            })
            .sum();
        resultant_force
    }

    fn calculate_incoming_force_from_node(
        node: &super::Node,
        other: &super::Node,
        default_gravitational_constant: &f32,
    ) -> Self {
        let displacement = Position::displacement(&node.position, &other.position);
        let magnitude_distance = displacement.magnitude();
        if magnitude_distance > Self::FORCE_RADIUS as f32 {
            return Self::zero();
        }
        // Avoid divide by zero errors
        if displacement == cgmath::Vector3::zero() {
            return Self::zero();
        }

        // Newtons law of universal gravitation
        // https://en.wikipedia.org/wiki/Newton%27s_law_of_universal_gravitation
        let g = match other.gravitational_constant_override {
            Some(gravitational_constant) => gravitational_constant,
            None => *default_gravitational_constant,
        };
        let m1 = node.mass;
        let m2 = other.mass;
        let r = magnitude_distance;
        let force = g * (m1 * m2 / r.powf(2.0));
        let force = force * displacement.normalize();
        Force(force)
    }

    pub fn magnitude(&self) -> f32 {
        self.0.magnitude()
    }
}

#[cfg(test)]
mod a_force {
    use cgmath::Zero;
    use rstest::rstest;

    use crate::node::{Force, Id, Node, Position};

    #[test]
    fn can_be_displayed() {
        let force = Force(cgmath::Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let expected_display_string =
            format!("x: {}, y: {}, z: {}", force.0.x, force.0.y, force.0.z);

        assert_eq!(expected_display_string, force.to_string())
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0), (1.0, 0.0, 1.0), (2.0, 3.0, 1.0), (3.0, 3.0, 2.0))]
    fn can_be_summed(
        #[case] force_1: (f32, f32, f32),
        #[case] force_2: (f32, f32, f32),
        #[case] force_3: (f32, f32, f32),
        #[case] expected_sum: (f32, f32, f32),
    ) {
        let (x, y, z) = force_1;
        let force_1 = Force(cgmath::Vector3 { x, y, z });
        let (x, y, z) = force_2;
        let force_2 = Force(cgmath::Vector3 { x, y, z });
        let (x, y, z) = force_3;
        let force_3 = Force(cgmath::Vector3 { x, y, z });
        let (x, y, z) = expected_sum;
        let expected_sum = Force(cgmath::Vector3 { x, y, z });
        let iter = vec![force_1, force_2, force_3].into_iter();
        assert_eq!(expected_sum, iter.sum());
    }

    #[rstest]
    #[case((0.0, 0.5, 0.0), (1.0, 0.0, -1.0), (1.0, 0.5, -1.0))]
    fn can_be_added(
        #[case] force_1: (f32, f32, f32),
        #[case] force_2: (f32, f32, f32),
        #[case] expected_force: (f32, f32, f32),
    ) {
        let (x, y, z) = force_1;
        let force_1 = Force(cgmath::Vector3 { x, y, z });
        let (x, y, z) = force_2;
        let force_2 = Force(cgmath::Vector3 { x, y, z });
        let (x, y, z) = expected_force;
        let expected_sum = Force(cgmath::Vector3 { x, y, z });
        assert_eq!(expected_sum, force_1 + force_2);
    }

    #[rstest]
    #[case((0.0, 0.5, 0.0), (1.0, 0.0, -1.0), (1.0, 0.5, -1.0))]
    fn can_be_add_assigned(
        #[case] force_1: (f32, f32, f32),
        #[case] force_2: (f32, f32, f32),
        #[case] expected_force: (f32, f32, f32),
    ) {
        let (x, y, z) = force_1;
        let mut force_1 = Force(cgmath::Vector3 { x, y, z });
        let (x, y, z) = force_2;
        let force_2 = Force(cgmath::Vector3 { x, y, z });
        let (x, y, z) = expected_force;
        let expected_sum = Force(cgmath::Vector3 { x, y, z });
        force_1 += force_2;
        assert_eq!(expected_sum, force_1);
    }

    #[test]
    fn zero_force() {
        assert_eq!(cgmath::Vector3::zero(), Force::zero().0)
    }

    #[test]
    fn can_calculate_incoming_force() {
        let node_a = Node::new(
            Id(1),
            Position(cgmath::Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
        );
        let node_b = Node::new(
            Id(2),
            Position(cgmath::Point3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }),
        );
        let gravitational_constant = -1.0;

        let expected_force_on_node_a = Force(cgmath::Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        });
        let force_on_node_a = super::Force::calculate_incoming_force(
            &node_a,
            &vec![&node_b],
            &gravitational_constant,
        );

        let expected_force_on_node_b = Force(cgmath::Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        });
        let force_on_node_b = super::Force::calculate_incoming_force(
            &node_b,
            &vec![&node_a],
            &gravitational_constant,
        );

        assert_eq!(expected_force_on_node_a, force_on_node_a);
        assert_eq!(expected_force_on_node_b, force_on_node_b);
        let forces_are_opposite = force_on_node_a == -force_on_node_b;
        assert!(forces_are_opposite);
    }

    #[test]
    fn can_be_multiplied() {
        let force = Force(cgmath::Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let factor = 2.0;
        let expected_force = Force(cgmath::Vector3 {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        });
        assert_eq!(expected_force, force * factor)
    }

    #[test]
    fn can_create_from_tuple() {
        let tuple = (1.0, 2.0, 3.0);
        let expected_force = Force(cgmath::Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        assert_eq!(expected_force, Force::from(tuple))
    }

    #[test]
    fn can_create_from_vector() {
        let vector = cgmath::Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let expected_force = Force(cgmath::Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        assert_eq!(expected_force, Force::from(vector))
    }

    #[test]
    fn can_turn_into_tuple() {
        let force = Force(cgmath::Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let expected_tuple = (1.0, 2.0, 3.0);
        assert_eq!(expected_tuple, force.into())
    }

    #[test]
    fn can_turn_into_vector() {
        let force = Force(cgmath::Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let expected_vector = cgmath::Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(expected_vector, force.into())
    }

    #[test]
    fn incoming_force_is_zero_if_distance_greater_than_force_radius() {
        let node_a = Node::new(
            Id(1),
            Position(cgmath::Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
        );
        let node_b = Node::new(
            Id(2),
            Position(cgmath::Point3 {
                x: Force::FORCE_RADIUS as f32 + 1.0,
                y: 0.0,
                z: 0.0,
            }),
        );
        let gravitational_constant = 1.0;

        let expected_force_on_node_a = Force(cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let force_on_node_a = super::Force::calculate_incoming_force(
            &node_a,
            &vec![&node_b],
            &gravitational_constant,
        );

        let expected_force_on_node_b = Force(cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let force_on_node_b = super::Force::calculate_incoming_force(
            &node_b,
            &vec![&node_a],
            &gravitational_constant,
        );

        assert_eq!(expected_force_on_node_a, force_on_node_a);
        assert_eq!(expected_force_on_node_b, force_on_node_b);
    }

    #[test]
    fn incoming_force_is_scaled_by_nodes_gravitational_constant() {
        let mut node_a = Node::new(
            Id(1),
            Position(cgmath::Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
        );
        node_a.gravitational_constant_override = Some(-2.0);
        let node_b = Node::new(
            Id(2),
            Position(cgmath::Point3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }),
        );
        let gravitational_constant = -1.0;

        // Constant only affects other nodes
        let expected_force_on_node_a = Force(cgmath::Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        });
        let force_on_node_a = super::Force::calculate_incoming_force(
            &node_a,
            &vec![&node_b],
            &gravitational_constant,
        );

        // Force is scaled by constant of other node
        let expected_force_on_node_b = Force(cgmath::Vector3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        });
        let force_on_node_b = super::Force::calculate_incoming_force(
            &node_b,
            &vec![&node_a],
            &gravitational_constant,
        );

        assert_eq!(expected_force_on_node_a, force_on_node_a);
        assert_eq!(expected_force_on_node_b, force_on_node_b);
    }

    /// Avoids issues with divide by zero
    #[test]
    fn incoming_force_is_zero_if_distance_is_zero() {
        let position = Position(cgmath::Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let node_a = Node::new(Id(1), position.clone());
        let node_b = Node::new(Id(2), position.clone());
        let gravitational_constant = 1.0;

        let expected_force_on_node_a = Force(cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let force_on_node_a = super::Force::calculate_incoming_force(
            &node_a,
            &vec![&node_b],
            &gravitational_constant,
        );

        let expected_force_on_node_b = Force(cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let force_on_node_b = super::Force::calculate_incoming_force(
            &node_b,
            &vec![&node_a],
            &gravitational_constant,
        );

        assert_eq!(expected_force_on_node_a, force_on_node_a);
        assert_eq!(expected_force_on_node_b, force_on_node_b);
    }

    #[rstest]
    #[case((1.0, 0.0, 0.0), 1.0)]
    #[case((-1.0, 0.0, 0.0), 1.0)]
    #[case((3.0, 0.0, 4.0), 5.0)]
    fn has_magnitude(#[case] force: (f32, f32, f32), #[case] expected_magnitude: f32) {
        let force = Force::from(force);
        assert_eq!(expected_magnitude, force.magnitude())
    }

    #[test]
    fn can_be_compared_based_on_magnitude() {
        let lesser_force = Force(cgmath::Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        });
        let greater_force = Force(cgmath::Vector3 {
            x: -2.0,
            y: -1.0,
            z: -1.0,
        });
        assert!(greater_force > lesser_force)
    }

    #[rstest]
    #[case("1.0,2.0,3.0", Force(cgmath::Vector3 { x: 1.0, y: 2.0, z: 3.0 }))]
    #[case("1,2,3", Force(cgmath::Vector3 { x: 1.0, y: 2.0, z: 3.0 }))]
    #[case("-1.0,2.0,3.0", Force(cgmath::Vector3 { x: -1.0, y: 2.0, z: 3.0 }))]
    #[case("\"1.0,2.0,3.0\"", Force(cgmath::Vector3 { x: 1.0, y: 2.0, z: 3.0 }))]
    #[case("\"-1.0,2.0,3.0\"", Force(cgmath::Vector3 { x: -1.0, y: 2.0, z: 3.0 }))]
    #[case("\"1.0, 2.0, 3.0\"", Force(cgmath::Vector3 { x: 1.0, y: 2.0, z: 3.0 }))]
    #[case("\"-1.0, 2.0, 3.0\"", Force(cgmath::Vector3 { x: -1.0, y: 2.0, z: 3.0 }))]
    fn can_be_created_from_a_valid_string(#[case] value: String, #[case] expected_force: Force) {
        let force = match Force::try_from(value.clone()) {
            Ok(force) => force,
            Err(err) => panic!("Failed converting string {} to Force - {}", value, err),
        };
        assert_eq!(expected_force, force)
    }

    #[rstest]
    #[case("1.0,2.0", "Force must have 3 values")]
    #[case("x,2.0,3.0", "Force vector x must be an f32")]
    #[case("1.0,y,3.0", "Force vector y must be an f32")]
    #[case("1.0,2.0,z", "Force vector z must be an f32")]
    fn cant_be_created_from_an_invalid_string(
        #[case] value: String,
        #[case] expected_error_message: String,
    ) {
        let error_message = match Force::try_from(value.clone()) {
            Ok(force) => panic!(
                "Unexpectedly succeeded converting string {} to Force {}",
                value, force
            ),
            Err(err) => err,
        };
        assert_eq!(expected_error_message, error_message)
    }
}
