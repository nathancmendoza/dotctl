// Data structures and functions that help dotter manage user application configurations

pub mod links {
    extern crate dirs;
    use std::path::{PathBuf, Path};
    use std::error::Error;
    use std::fmt;

    use dirs::home_dir;

    pub const HOME_PREFIX: &str = "~/";

    fn expand_user<P: AsRef<Path>>(path: P) -> Result<PathBuf, LinkResolutionError> {
        match path.as_ref().to_path_buf().strip_prefix(HOME_PREFIX) {
            Ok(p) => {
                match home_dir() {
                    Some(home) => Ok(home.to_path_buf().join(p)),
                    None => Err(LinkResolutionError::NoHomeDirectoryFound)
                }
            },
            Err(_) => Err(LinkResolutionError::NoHomeDirectoryToResolve)
        }
    }

    fn expand_with_parent<P: AsRef<Path>>(parent: P, path: &PathBuf) -> PathBuf {
        parent.as_ref().to_path_buf().join(path).to_path_buf()
    }

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

    #[derive(Debug, Clone)]
    pub enum LinkResolutionError {
        NoHomeDirectoryFound,
        NoHomeDirectoryToResolve,
        NoParentToResolveRelativePath
    }

    impl fmt::Display for LinkResolutionError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                LinkResolutionError::NoHomeDirectoryFound => write!(f, "The `~` symbol could not be resolved to a path"),
                LinkResolutionError::NoHomeDirectoryToResolve => write!(f, "Expected to resolve `~` prefix but was not found"),
                LinkResolutionError::NoParentToResolveRelativePath => write!(f, "Cannot resolve relative path without a parent")
            }
        }
    }

    impl Error for LinkResolutionError {}

    impl LinkSpec {
        pub fn new<P: AsRef<Path>>(source_path: P, target_path: P, mode: LinkMode) -> Self {
            LinkSpec {
                source: source_path.as_ref().to_path_buf(),
                target: target_path.as_ref().to_path_buf(),
                link_mode: mode
            }
        }
        
        pub fn get_canonical_source<P: AsRef<Path>>(&self, parent: Option<P>) -> Result<PathBuf, LinkResolutionError> {
            if self.source.is_absolute() {
                return Ok(expand_with_parent("/", &self.source));
            }

            if self.source.starts_with(HOME_PREFIX) {
                expand_user(&self.source)
            }
            else {
                match parent {
                    Some(root) => {
                        if root.as_ref().starts_with(HOME_PREFIX) {
                            return expand_user(expand_with_parent(root, &self.source))
                        }
                        Ok(expand_with_parent(root, &self.source))
                    },
                    None => Err(LinkResolutionError::NoParentToResolveRelativePath)
                }
            } 
        }

        pub fn get_canonical_target(&self) -> Result<PathBuf, LinkResolutionError> {
            if self.target.is_absolute() {
                Ok(expand_with_parent("/", &self.target))
            }
            else if self.target.starts_with(HOME_PREFIX) {
                expand_user(&self.target)
            }
            else {
                Err(LinkResolutionError::NoParentToResolveRelativePath)
            }
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

        assert_eq!(expected.join(s.strip_prefix(HOME_PREFIX).unwrap()).as_os_str(), true_source.as_os_str());
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

    #[test]
    fn resolve_target_with_tilde() {
        let s = "~/an/app.conf";
        let t = "~/an/app/config";
        let expected = dirs::home_dir().unwrap();
        let spec = LinkSpec::new(s, t, LinkMode::Link);

        let true_target = match spec.get_canonical_target() {
            Ok(path) => path,
            Err(_) => panic!("Target path not retrieved")
        };

        assert_eq!(expected.join(t.strip_prefix(HOME_PREFIX).unwrap()).as_os_str(), true_target.as_os_str());
    }

    #[test]
    fn resolve_target_that_is_absolute() {
        let s = "~/an/app.conf";
        let t = "/etc/an/app/config";
        let spec = LinkSpec::new(s, t, LinkMode::Link);

        let true_target = match spec.get_canonical_target() {
            Ok(path) => path,
            Err(_) => panic!("Target path not retrieved")
        };

        assert_eq!(t, true_target.as_os_str());
    }
}
