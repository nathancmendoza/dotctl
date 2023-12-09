# Modules

## Concerns

| Module | Purpose | Interactions |
|:------:| ------- | ------------ |
| `cli` | Entry point and command processor of `dotter` | `config` |
| `config` | Parsing and validation of `dotter` configurations | `cli`, `filesystem`, `hooks` |
| `filesystem` | Filesystem operations as dictated by application configurations | `config` |
| `hooks` | Hook process management as dictated by application configurations | `config` |

## Details

### `cli`

Implements the command parsing and handlers mentioned in the [CLI](../interaction/cli.md) chapter. Ensures the command verb is valid and the any required arguments and options are set. Also perform error handling for clean exits when things don't go as planned.

- [CLI Builder](https://crates.io/crates/clap)
- [Logging Facilities](https://crates.io/crates/log)

### `config`

Responsible for parsing and extracting useful information from the configuration file. Also provides validation for field usage and date provided to field

- [YAML Parse Support](https://crates.io/crates/serde_yaml)
- [JSON Parse Support](https://crates.io/crates/serde_json)

### `filesystem`

Performs file system operations relevant to the application. This includes creating symlinks, directory, copying files and removing files and directory. Any actions taken are specified in the application configuration

- [Basic FS Actions](https://doc.rust-lang.org/std/fs/index.html)
- [OS Dependent Functionality](https://doc.rust-lang.org/std/os/index.html)
- [Configuration Checks](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html)
- [Path Manipulation](https://doc.rust-lang.org/std/path/struct.Path.html)

### `hooks`

Handles spawning of child process for processing hook commands. Should allow provide capturing of child process output.

- [Command Builder](https://doc.rust-lang.org/std/process/index.html)




