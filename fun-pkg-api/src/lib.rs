use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Default)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub mod traits {
    use crate::Version;

    pub trait Pkg {
        ///  get package name
        fn pkg_name(&self) -> String;

        /// get package version
        fn pkg_version(&self) -> Version;
    }
}
