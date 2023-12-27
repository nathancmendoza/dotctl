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

    impl<'a, D: AsRef<Path>> ResolveWithParentDirectory<'a, D> {
        pub fn new(root: &'a Option<D>) -> Self {
            ResolveWithParentDirectory { parent: root }
        }
    }

    impl<'a, D: AsRef<Path>> LinkResolver for ResolveWithParentDirectory<'a, D> {
        fn resolve<P: AsRef<Path>>(&self, to_resolve: P) -> Result<PathBuf, LinkResolutionError> {
            match &self.parent {
                Some(p) => Ok(p.as_ref().to_path_buf().join(to_resolve.as_ref())),
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

        fn find_source_resolutions(&self, to_resolve: &Path) -> Option<LinkResolutionIssue> {
            
            if to_resolve.starts_with(HOME_PREFIX) {
                return Some(LinkResolutionIssue::HasTilde);
            }

            if to_resolve.is_relative() {
                return Some(LinkResolutionIssue::IsRelative);
            }

            None
        }

        fn find_target_resolutions(&self, to_resolve: &Path) -> Option<LinkResolutionIssue> {

            if to_resolve.starts_with(HOME_PREFIX) {
                return Some(LinkResolutionIssue::HasTilde);
            }

            if to_resolve.is_relative() {
                return Some(LinkResolutionIssue::IsRelative);
            }

            None
        }
    }
}

#[cfg(test)]
mod links_test {
    use dirs::home_dir;

    use super::links::{LinkSpec, LinkMode};

    #[test]
    fn test_link_specs_are_comparable() {
        let s = "~/appconfs/app.conf";
        let t = "~/.appconf";

        let spec1 = LinkSpec::new(s, t, LinkMode::Link);
        let spec2 = LinkSpec::new(s, t, LinkMode::Link);

        assert_eq!(spec1, spec2);
    }

    #[test]
    fn test_link_specs_differ_when_paths_do_not_match() {
        let s = "~/appconfs/app.conf";
        let t = "~/.appconf";

        let spec1 = LinkSpec::new(s, t, LinkMode::Link);
        let spec2 = LinkSpec::new(s, s, LinkMode::Link);

        assert_ne!(spec1, spec2);
    }

    #[test]
    fn test_link_specs_differ_when_link_modes_do_not_match() {
        let s = "~/appconfs/app.conf";
        let t = "~/.appconf";

        let spec1 = LinkSpec::new(s, t, LinkMode::Link);
        let spec2 = LinkSpec::new(s, t, LinkMode::Copy);

        assert_ne!(spec1, spec2);
    }

    #[test]
    fn test_link_mode_is_publicly_visible() {
        let s = "~/appconfs/app.conf";
        let t = "~/.appconf";

        let spec1 = LinkSpec::new(s, t, LinkMode::Link);
        let spec2 = LinkSpec::new(s, t, LinkMode::Copy);

        assert_eq!(spec1.get_link_strategy(), &LinkMode::Link);
        assert_eq!(spec2.get_link_strategy(), &LinkMode::Copy);
        assert_ne!(spec1.get_link_strategy(), spec2.get_link_strategy());
    }

    #[test]
    fn test_source_resolution_of_absolute_path() {
        let s = "/etc/app/app.conf";
        let t = "";

        let spec = LinkSpec::new(s, t, LinkMode::Link);
        let res = spec.get_resolved_source(None);

        assert!(res.is_ok());
        assert_eq!(s, res.unwrap().as_os_str());
    }

    #[test]
    fn test_source_resolution_with_tilde() {
        let s = "~/appconfs/app.conf";
        let t = "";
        let home = home_dir().unwrap();

        let spec = LinkSpec::new(s, t, LinkMode::Link);
        let res = spec.get_resolved_source(None);

        assert!(res.is_ok());
        assert!(res.unwrap().starts_with(home));
    }

    #[test]
    fn test_source_resolution_with_parent() {
        let s = "appconfs/app.conf";
        let t = "";

        let spec = LinkSpec::new(s, t, LinkMode::Link);
        let res = spec.get_resolved_source(Some("/var/etc/"));

        assert!(res.is_ok());
        assert_eq!(res.unwrap().as_os_str(), "/var/etc/appconfs/app.conf");
    }

    #[test]
    fn test_source_resolution_with_parent_that_introduces_tilde() {
        let s = "appconfs/app.conf";
        let t = "";
        let home = home_dir().unwrap();

        let spec = LinkSpec::new(s, t, LinkMode::Link);
        let res = spec.get_resolved_source(Some("~/.config"));

        assert!(res.is_ok());
        assert!(res.unwrap().starts_with(home));
    }

    #[test]
    fn test_source_resolution_without_parent_fails() {
        let s = "appconfs/app.conf";
        let t = "";

        let spec = LinkSpec::new(s, t, LinkMode::Copy);
        let res = spec.get_resolved_source(None);

        assert!(res.is_err());
    }

    #[test]
    fn test_target_resolution_of_absolute_path() {
        let s = "/etc/app/app.conf";
        let t = "";

        let spec = LinkSpec::new(t, s, LinkMode::Link);
        let res = spec.get_resolved_target();

        assert!(res.is_ok());
        assert_eq!(s, res.unwrap().as_os_str());
    }

    #[test]
    fn test_target_resolution_of_relative_path_fails() {
        let s = "appconfs/app.conf";
        let t = "";

        let spec = LinkSpec::new(t, s, LinkMode::Copy);
        let res = spec.get_resolved_target();

        assert!(res.is_err());
    }

    #[test]
    fn test_target_resolution_with_tilde() {
        let s = "~/appconfs/app.conf";
        let t = "";
        let home = home_dir().unwrap();

        let spec = LinkSpec::new(t, s, LinkMode::Link);
        let res = spec.get_resolved_target();

        assert!(res.is_ok());
        assert!(res.unwrap().starts_with(home));

    }
}
