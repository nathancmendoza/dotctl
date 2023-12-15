// Data structures and functions that help dotter manage user application configurations

pub mod links {
    extern crate dirs;
    use std::path::{PathBuf, Path};

    pub const HOME_PREFIX: &str = "~";

    #[derive(Debug, PartialEq, Eq)]
    pub enum LinkMode {
        Link,
        Copy,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct LinkSpec {
        source: PathBuf,
        target: PathBuf,
        link_mode: LinkMode
    }

    impl LinkSpec {
        pub fn new<P: AsRef<Path>>(source_path: P, target_path: P, mode: LinkMode) -> Self {
            LinkSpec {
                source: source_path.as_ref().to_path_buf(),
                target: target_path.as_ref().to_path_buf(),
                link_mode: mode
            }
        }
        
        pub fn get_canonical_source<P: AsRef<Path>>(&self, parent: Option<P>) -> Result<PathBuf, &str> {
            unimplemented!()
        }

        pub fn get_canonical_target(&self) -> Result<PathBuf, &str> {
            unimplemented!()
        }

        pub fn get_link_strategy(&self) -> &LinkMode {
            &self.link_mode
        }

    }
}

#[cfg(test)]
mod links_test {

    use std::path::PathBuf;

    use crate::links::{LinkSpec, LinkMode, HOME_PREFIX};

    #[test]
    fn link_spec_from_path() {
        let s = "an/app.conf";
        let t = "~/app/config";


        let spec1 = LinkSpec::new(s, t, LinkMode::Link);
        let spec2 = LinkSpec::new(s, t, LinkMode::Link);

        assert_eq!(spec1, spec2);
    }

    #[test]
    fn link_spec_differs_by_mode() {
        let s = "an/app.conf";
        let t = "~/app/config";

        let spec1 = LinkSpec::new(s, t, LinkMode::Link);
        let spec2 = LinkSpec::new(s, t, LinkMode::Copy);

        assert_ne!(spec1, spec2);
    }

    #[test]
    fn link_spec_differs_by_paths() {
        let s = "an/app.conf";
        let t = "~/app/config";

        let spec1 = LinkSpec::new(s, t, LinkMode::Link);
        let spec2 = LinkSpec::new(t, s, LinkMode::Link);

        assert_ne!(spec1, spec2);
    }

    #[test]
    fn link_spec_stratgety_retrieval() {
        let s = "an/app.conf";
        let t = "~/app/config";

        let spec1 = LinkSpec::new(s, t, LinkMode::Link);
        let spec2 = LinkSpec::new(s, t, LinkMode::Copy);

        assert_eq!(spec1.get_link_strategy(), &LinkMode::Link);
        assert_eq!(spec2.get_link_strategy(), &LinkMode::Copy);
        assert_ne!(spec1.get_link_strategy(), spec2.get_link_strategy());

    }

    #[test]
    fn resolve_source_with_tilde() {
        let s = "~/an/app.conf";
        let t = "~/an/app/config";
        let expected = dirs::home_dir().unwrap();
        let spec = LinkSpec::new(s, t, LinkMode::Link);

        let true_source = match spec.get_canonical_source::<&str>(None) {
            Ok(p) => p,
            Err(_) => panic!("Souce path not retrieved")
        };

        expected.join(s.strip_prefix(HOME_PREFIX).unwrap());
        assert_eq!(expected.as_os_str(), true_source.as_os_str());
    }

    #[test]
    fn resolve_source_with_provided_parent() {
       let s = "an/app.conf";
       let t = "~/an/app/config";
       let p = "/etc/app/";

       let spec = LinkSpec::new(s, t, LinkMode::Link);

       let true_source = match spec.get_canonical_source(Some(p)) {
            Ok(path) => path,
            Err(_) => panic!("Source path not retrieved")
       };

       assert_eq!(PathBuf::from(p).join(s).as_os_str(), true_source.as_os_str());
    }

    #[test]
    fn resolve_source_that_is_absolute() {
        let s = "/etc/app.conf";
        let t = "/usr/bin/app";

        let spec = LinkSpec::new(s, t, LinkMode::Link);

        let true_source = match spec.get_canonical_source::<&str>(None) {
            Ok(p) => p,
            Err(_) => panic!("Source path not retrieved")
        };

        assert_eq!(s, true_source.as_os_str());
    }

    #[test]
    fn resolve_source_with_provided_parent_that_introduces_tilde() {
        let s = "an/app.conf";
        let t = "~/an/app/config";
        let p = "~/appconfigs/";

        let spec = LinkSpec::new(s, t, LinkMode::Link);

        let true_source = match spec.get_canonical_source(Some(p)) {
            Ok(path) => path,
            Err(_) => panic!("Source path not retrieved")
        };

        let expected = dirs::home_dir().unwrap();
        

        assert_eq!(expected.join(p.strip_prefix(HOME_PREFIX).unwrap()).join(s).as_os_str(), true_source.as_os_str());

    }
}
