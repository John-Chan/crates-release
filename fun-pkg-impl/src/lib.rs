pub mod v1 {
    use fun_pkg_api::traits::Pkg;
    use fun_pkg_api::Version;

    pub struct FunPkg {
        name: String,
        version: Version,
    }

    impl FunPkg {
        pub fn new(name: String, version: Version) -> Self {
            FunPkg { name, version }
        }
    }

    impl Pkg for FunPkg {
        fn pkg_name(&self) -> String {
            self.name.clone()
        }

        fn pkg_version(&self) -> Version {
            self.version.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::v1::FunPkg;
    use fun_pkg_api::traits::Pkg;
    use fun_pkg_api::Version;

    #[test]
    fn test_pkg_name() {
        let pkg = FunPkg::new("fun-pkg".to_string(), Version::default());
        assert_eq!(pkg.pkg_name(), "fun-pkg");
    }

    #[test]
    fn test_pkg_version() {
        let pkg = FunPkg::new("fun-pkg".to_string(), Version::new(1, 0, 0));
        assert_eq!(pkg.pkg_version().to_string(), "1.0.0");
    }
}
