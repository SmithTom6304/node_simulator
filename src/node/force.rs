use std::{
    iter::Sum,
    ops::{Add, AddAssign, Neg},
};

use cgmath::{self, InnerSpace, Zero};

use super::Position;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Force(pub cgmath::Vector3<f32>);

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
        let force = -g * (m1 * m2 / r.powf(2.0)); // Negate to push away
        let force = force * displacement.normalize();
        Force(force)
    }
}

#[cfg(test)]
mod a_force {
    use cgmath::Zero;
    use rstest::rstest;

    use crate::node::{Force, Id, Node, Position};

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
        let gravitational_constant = 1.0;

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
}
