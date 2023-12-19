use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Id(pub u32);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
