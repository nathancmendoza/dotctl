// Data structures and functions that help dotter manage user application configurations

pub mod links {
    use std::path::PathBuf;

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
        pub fn new(source_path: PathBuf, target_path: PathBuf, mode: LinkMode) -> Self {
            LinkSpec {
                source: source_path,
                target: target_path,
                link_mode: mode
            }
        }
    }
}

pub mod hooks {

    use std::vec::Vec;
    use std::process::Command;

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

    use std::path::Path;
    use crate::links::{LinkSpec, LinkMode};

    #[test]
    fn link_spec_from_path() {
        let s = Path::new("an/app.conf").to_path_buf();
        let t = Path::new("~/app/config").to_path_buf();


        let spec1 = LinkSpec::new(s.clone(), t.clone(), LinkMode::Link);
        let spec2 = LinkSpec::new(s.clone(), t.clone(), LinkMode::Link);

        assert_eq!(spec1, spec2);
    }

    #[test]
    fn link_spec_differs_by_mode() {
        let s = Path::new("an/app.conf").to_path_buf();
        let t = Path::new("~/app/config").to_path_buf();


        let spec1 = LinkSpec::new(s.clone(), t.clone(), LinkMode::Link);
        let spec2 = LinkSpec::new(s.clone(), t.clone(), LinkMode::Copy);

        assert_ne!(spec1, spec2);
    }
}
