// Data structures and functions that help dotter manage user application configurations

pub mod links {
    extern crate dirs;
    use std::path::{PathBuf, Path};
    use std::error::Error;
    use std::fmt;

    use dirs::home_dir;

    trait LinkResolver {

        fn resolve<P: AsRef<Path>>(&self, to_resolve: P) -> Result<PathBuf, LinkResolutionError>;

        fn can_resolve<P: AsRef<Path>>(&self, to_resolve: P) -> bool;
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum LinkMode {
        Link,
        Copy,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct LinkSpec<P: AsRef<Path>> {
        source: P,
        target: P,
        link_mode: LinkMode
    }

    #[derive(Debug, Clone)]
    pub enum LinkResolutionError {
        NoHomeDirectoryFound,
        NoHomeDirectoryToResolve
    }

    struct ResolveWithHomeDirectory<'a> {
        home_dir: Option<PathBuf>,
        home_prefix: &'a str
    }

    struct ResolveWithParentDirectory<D: AsRef<Path>> {
        parent: D
    }

    struct ResolveWithRootDirectory{}

    impl ResolveWithRootDirectory {
        pub fn new() -> Self {
            ResolveWithRootDirectory{}
        }
    }

    impl LinkResolver for ResolveWithRootDirectory {
        fn resolve<P: AsRef<Path>>(&self, to_resolve: P) -> Result<PathBuf, LinkResolutionError> {
            Ok(to_resolve.as_ref().to_path_buf())
        }

        fn can_resolve<P: AsRef<Path>>(&self, to_resolve: P) -> bool {
            to_resolve.as_ref().to_path_buf().is_absolute()
        }

    }

    impl<D: AsRef<Path>> ResolveWithParentDirectory<D> {
        pub fn new(root: D) -> Self {
            ResolveWithParentDirectory { parent: root }
        }
    }

    impl<D: AsRef<Path>> LinkResolver for ResolveWithParentDirectory<D> {
        fn resolve<P: AsRef<Path>>(&self, to_resolve: P) -> Result<PathBuf, LinkResolutionError> {
            Ok(self.parent.as_ref().to_path_buf().join(to_resolve.as_ref().to_path_buf()))
        }

        fn can_resolve<P: AsRef<Path>>(&self, to_resolve: P) -> bool {
            let suffix = to_resolve.as_ref().to_path_buf();
            suffix.is_relative() && !suffix.starts_with(&self.parent)
        }
    }

    impl<'a> ResolveWithHomeDirectory<'a>{
        pub fn new() -> Self {
            ResolveWithHomeDirectory {home_dir: home_dir(), home_prefix: "~/"}
        }
    }

    impl<'a> LinkResolver for ResolveWithHomeDirectory<'a> {
        fn resolve<P: AsRef<Path>>(&self, to_resolve: P) -> Result<PathBuf, LinkResolutionError> {
            match &self.home_dir {
                None => Err(LinkResolutionError::NoHomeDirectoryFound),
                Some(h) => {
                    match to_resolve.as_ref().to_path_buf().strip_prefix(self.home_prefix) {
                        Err(_) => Err(LinkResolutionError::NoHomeDirectoryToResolve),
                        Ok(suffix) => {
                            Ok(h.join(suffix))
                        }
                    }
                }
            }
        }

        fn can_resolve<P: AsRef<Path>>(&self, to_resolve: P) -> bool {
            let suffix = to_resolve.as_ref().to_path_buf();
            suffix.starts_with(self.home_prefix)
        }
    }

    impl fmt::Display for LinkResolutionError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                LinkResolutionError::NoHomeDirectoryFound => write!(f, "The `~` symbol could not be resolved to a path"),
                LinkResolutionError::NoHomeDirectoryToResolve => write!(f, "Expected to resolve `~` prefix but was not found")
            }
        }
    }

    impl Error for LinkResolutionError {}


    impl<P: AsRef<Path>> LinkSpec<P> {
        pub fn new(source_path: P, target_path: P, mode: LinkMode) -> Self {
            LinkSpec {
                source: source_path,
                target: target_path,
                link_mode: mode
            }
        }
        
        pub fn get_canonical_source(&self, parent: Option<P>) -> Result<PathBuf, LinkResolutionError> {
            unimplemented!() 
        }

        pub fn get_canonical_target(&self) -> Result<PathBuf, LinkResolutionError> {
            unimplemented!()
        }

        pub fn get_link_strategy(&self) -> &LinkMode {
            &self.link_mode
        }

        fn resolve_link_path(&self, to_resolve: P) -> Result<PathBuf, LinkResolutionError> {
            
        }

    }
}

#[cfg(test)]
mod links_test {

}
