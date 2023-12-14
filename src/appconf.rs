// Data structures and functions that help dotter manage user application configurations

pub mod links {
    extern crate dirs;
    use std::path::{PathBuf, Path};

    const HOME_PREFIX: &str = "~";

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

        pub fn get_canonical_source(&self) -> Result<PathBuf, &str> {
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

pub mod hooks {

    use std::vec::Vec;
    //use std::process::Command;

    #[derive(Debug, PartialEq, Eq)]
    pub enum HookTime {
        Preinstall,
        Postinstall,
        Preremove,
        Postremove
    }

    pub struct HookProcess {
        commands: Vec<String>,
        runtime: HookTime
    }

    impl HookProcess {
        pub fn new(cmds: Vec<String>, when: HookTime) -> Self {
            HookProcess {
                commands: cmds,
                runtime: when
            }
        }

        pub fn run(&self) {
            for cmd in self.commands.iter() {
                self.exec(&cmd);
            }
        }

        fn exec(&self, command: &String) {
            unimplemented!()
        }
    }
}


#[cfg(test)]
mod links_test {

    use crate::links::{LinkSpec, LinkMode};

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
}
