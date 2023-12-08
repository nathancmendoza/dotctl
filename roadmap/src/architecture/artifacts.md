# Artifacts

## Expectations

This project must produce a software artifact that meets the following criteria

- Deployment Environment: the expected environment is end-user machines, but can also fit into scripts and other automation tools
- Ease of Distribution: the artifact should be able to function on the appropriate OS as-is; external dependencies should be minimal
- Platform Independence: since `dotter` supports dotfiles on any system, it should also work on multiple platforms

With these criteria is mind, the *only* artifact generated executable should be a standalone executable. 

## Helpful tools

### Languages

Language **must** have a toolchain that supports compilation to standalone executables

- C/C++: viable, but involve manual memory management
- Rust: just as performant as C, but stricter rules to help manage memory
- Go: similar tools written with this languages

### Libraries

- A library for parsing JSON/YAML files
- A library to build a CLI
- A library to abstract filesystem operations across systems
