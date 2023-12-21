use std::{iter::Sum, ops::AddAssign};

use cgmath::{self, InnerSpace, Zero};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Force(pub cgmath::Vector3<f32>);

impl Sum for Force {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let summed_vector = iter.map(|f| f.0).sum();
        Self(summed_vector)
    }
}

impl AddAssign for Force {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
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
        let distance = node.position.0 - other.position.0;
        let magnitude_distance = distance.magnitude();
        if magnitude_distance > Self::FORCE_RADIUS as f32 {
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
        let force = force * distance.normalize();
        Force(force)
    }
}

#[cfg(test)]
mod a_force {
    use rstest::rstest;

    use crate::node::Force;

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
}
