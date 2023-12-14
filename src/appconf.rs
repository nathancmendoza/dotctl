// Data structures and functions that help dotter manage user application configurations

pub mod links {
    extern crate dirs;
    use std::path::{PathBuf};


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
        pub fn new<P: AsRef<PathBuf>>(source_path: P, target_path: P, mode: LinkMode) -> Self {
            LinkSpec {
                source: source_path.as_ref().to_path_buf(),
                target: target_path.as_ref().to_path_buf(),
                link_mode: mode
            }
        }

        pub fn source<R: AsRef<PathBuf>>(&self, root: Option<R>) -> Result<PathBuf, &str> {
            if self.source.is_absolute() {
                return Ok(self.source.clone())
            }
            if self.source.is_relative() {
                let _ = match root {
                    Some(p) => Ok::<PathBuf, &str>(self.resolve_path_with_root(p)),
                    None => Ok(self.expanduser_on_source())
                };
            }
            Err("Failed to resolve link source path")
        }

        pub fn target(&self) -> Result<PathBuf, &str> {
            unimplemented!()
        }

        fn resolve_path_with_root<R: AsRef<PathBuf>>(&self, root: R) -> PathBuf {
            root.as_ref().to_path_buf().join(self.source.clone())
        }

        fn expanduser_on_source(&self) -> PathBuf {
            const HOME_PREFIX: &str = "~";
            if self.source.starts_with(HOME_PREFIX) {
                let suffix = self.source.strip_prefix(HOME_PREFIX).unwrap();
                match dirs::home_dir() {
                    Some(h) => h.join(suffix),
                    None => PathBuf::from("/").join(suffix)
                };
            }
            self.source.clone()
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
