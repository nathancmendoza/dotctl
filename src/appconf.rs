// Data structures and functions that help dotter manage user application configurations

pub mod links {
    extern crate dirs;
    use std::path::{PathBuf, Path};
    use std::error::Error;
    use std::fmt;

    use dirs::home_dir;

    const HOME_PREFIX: &str = "~/";

    fn use_resolver<P: AsRef<Path>>(to_resolve: P, resolver: impl LinkResolver) -> Result<PathBuf, LinkResolutionError> {
        resolver.resolve(to_resolve)
    }



    trait LinkResolver {

        fn resolve<P: AsRef<Path>>(&self, to_resolve: P) -> Result<PathBuf, LinkResolutionError>;

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
        NoHomeDirectoryToResolve,
        ResolverNotApplicable
    }

    #[derive(Debug, Clone)]
    enum LinkResolutionIssue {
        HasTilde,
        IsRelative,
    }

    struct ResolveWithHomeDirectory<'a> {
        home_dir: Option<PathBuf>,
        home_prefix: &'a str
    }

    struct ResolveWithParentDirectory<'a, D: AsRef<Path>> {
        parent: &'a Option<D>
    }

//    struct ResolveWithRootDirectory{}
//
//    impl ResolveWithRootDirectory {
//        pub fn new() -> Self {
//            ResolveWithRootDirectory{}
//        }
//    }
//
//    impl LinkResolver for ResolveWithRootDirectory {
//        fn resolve<P: AsRef<Path>>(&self, to_resolve: P) -> Result<PathBuf, LinkResolutionError> {
//            Ok(to_resolve.as_ref().to_path_buf())
//        }
//
//        fn can_resolve<P: AsRef<Path>>(&self, to_resolve: P) -> bool {
//            to_resolve.as_ref().to_path_buf().is_absolute()
//        }
//
//    }

    impl<'a, D: AsRef<Path>> ResolveWithParentDirectory<'a, D> {
        pub fn new(root: &'a Option<D>) -> Self {
            ResolveWithParentDirectory { parent: root }
        }
    }

    impl<'a, D: AsRef<Path>> LinkResolver for ResolveWithParentDirectory<'a, D> {
        fn resolve<P: AsRef<Path>>(&self, to_resolve: P) -> Result<PathBuf, LinkResolutionError> {
            match &self.parent {
                Some(p) => Ok(p.as_ref().to_path_buf().join(to_resolve.as_ref().to_path_buf())),
                None => Err(LinkResolutionError::ResolverNotApplicable)
            }
        }
    }

    impl<'a> ResolveWithHomeDirectory<'a>{
        pub fn new() -> Self {
            ResolveWithHomeDirectory {home_dir: home_dir(), home_prefix: HOME_PREFIX}
        }
    }

    impl<'a> LinkResolver for ResolveWithHomeDirectory<'a> {
        fn resolve<P: AsRef<Path>>(&self, to_resolve: P) -> Result<PathBuf, LinkResolutionError> {
            match &self.home_dir {
                None => Err(LinkResolutionError::NoHomeDirectoryFound),
                Some(h) => {
                    match to_resolve.as_ref().to_path_buf().strip_prefix(self.home_prefix) {
                        Err(_) => Err(LinkResolutionError::NoHomeDirectoryToResolve),
                        Ok(suffix) => Ok(h.join(suffix))
                    }
                }
            }
        }
    }

    impl fmt::Display for LinkResolutionError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                LinkResolutionError::NoHomeDirectoryFound => write!(f, "The `~` symbol could not be resolved to a path"),
                LinkResolutionError::NoHomeDirectoryToResolve => write!(f, "Expected to resolve `~` prefix but was not found"),
                LinkResolutionError::ResolverNotApplicable => write!(f, "The chosen resolver cannot be applied")
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
        
        pub fn get_resolved_source(&self, parent: Option<P>) -> Result<PathBuf, LinkResolutionError> {
            let mut true_source = self.source.as_ref().to_path_buf();
            while let Some(issue) = self.find_source_resolutions(&true_source) {
                let resolved_source = match issue {
                    LinkResolutionIssue::HasTilde => use_resolver(true_source, ResolveWithHomeDirectory::new()),
                    LinkResolutionIssue::IsRelative => use_resolver(true_source, ResolveWithParentDirectory::new(&parent))
                }?;
                true_source = resolved_source;
            }
            Ok(true_source)
        }

        pub fn get_resolved_target(&self) -> Result<PathBuf, LinkResolutionError> {
            let mut true_target = self.target.as_ref().to_path_buf();
            while let Some(issue) = self.find_target_resolutions(&true_target) {
                let resolved_target = match issue {
                    LinkResolutionIssue::HasTilde => use_resolver(true_target, ResolveWithHomeDirectory::new()),
                    LinkResolutionIssue::IsRelative => {
                        let no_parent: Option<&str> = None;
                        use_resolver(true_target, ResolveWithParentDirectory::new(&no_parent))
                    }
                }?;
                true_target = resolved_target;
            }
            Ok(true_target)
        }

        pub fn get_link_strategy(&self) -> &LinkMode {
            &self.link_mode
        }

        fn find_source_resolutions(&self, to_resolve: &PathBuf) -> Option<LinkResolutionIssue> {
            
            if to_resolve.starts_with(HOME_PREFIX) {
                return Some(LinkResolutionIssue::HasTilde);
            }

            if to_resolve.is_relative() {
                return Some(LinkResolutionIssue::IsRelative);
            }

            None
        }

        fn find_target_resolutions(&self, to_resolve: &PathBuf) -> Option<LinkResolutionIssue> {

            if to_resolve.starts_with(HOME_PREFIX) {
                return Some(LinkResolutionIssue::HasTilde);
            }

            None
        }
    }
}

#[cfg(test)]
mod links_test {

}
