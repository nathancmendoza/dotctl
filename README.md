# Dotctl

`dotctl` is a CLI tool for managing active program configuration files (AKA "dotfiles"). It works alongside a dedicated directory that centralizes configuration location while allowing them to be linked appropriately

## Usage

### Your "dotfiles" repository

This tool aims to be agnostic of an individual "dotfile" repository. The storage and/or retrieval of the files is not handled by this tool. Whether you use git to manage a literal repository or you have a dedicated place on a cloud server for your "dotfiles", this tool can help with setting up these configurations.

As mentioned before, the layout and structure of the "dotfile" directory does not matter. The only assumption for now is that there is only a single repository housing the "dotfiles". This is intended because the tool should be able to integrate into existing setups without heavily modifying it. There may be some initial work to setup to get things working smoothly, but hopefully such work focused around the `dotctl` application and not anything that already exists.

### Your `dotctl` config

Currently, the only configuration format supported is YAML, but more formats may come later on. The configuration for `dotctl` consists of two parts: an options sections and a list of configurations to work with. If a configuration is not part of that list, `dotctl` won't know about it. 

#### Configuration options

This section assigns values to certain things that `dotctl` may utilize. Currently, the only thing here is the location of the "dotfile" repository that this configuration is part of.

```yaml
options:
  repository: "~/.dotfiles"
```

The value here should be the full path for the directory that houses the "dotfiles". It should be absolute, but support for the `~` shorthand is included for convenience.

#### Configuration apps

Application configurations are objects in a list that `dotctl` will be able to work with. Each configuration needs to specify some information to determine

1) How to set up the configuration
2) If it is allowed/safe to do so on the current system

The instruction to setup a configuration are specified as a list of "links" to create. Links will be created sequentially until the setup process is complete. Safety is determined by the current OS name and the user assigned status of the specific configuration.

```yaml
configuratoins:
  - name: "bash" # This is the name specified on the command line. Should be unique, but not enforced.
    status: Ready | Unused # Ready would indicate configuration is ok to use, while unused would indicate the configuration has been archived
    os: linux # Operating system configuration is allowed to be installed on. Cross platform configuration should make separate entries
    links: # The links to create with this configuration
      source: "config.sh" # Path to *real* file. Can be specified relative to the repository option or as an absolute path
      target: "~/.bashrc" # Path to place the symlink/copy. Should be absolute with `~` shorthand supported
      mode: copy # The linking method to use. Can be copy, hard, or soft
      # Copy: will create a copy of source and place it at target
      # Soft: will create a soft link (or symlink) at target pointing to source
      # Hard: will create a hard link at target pointing to source
    # More links to follow if needed
  # More configurations to follow if needed
```

The configuration is designed to specify *how* to navigate a current "dotfile" repository rather than force a layout or structure onto the user. It will hopefully extend to be able to work with more complicated application configurations

#### Example

A full example of the configuration would look like. Note that names for individual links are optional.

```yaml
options:
  repository: ~/.dotfiles
configurations:
  - name: bash
    status: Ready
    os: linux
    links:
    - name: init
      source: bash/init.sh
      target: ~/.bashrc
      mode: Soft
    - name: aliases
      source: bash/aliases.sh
      target: ~/.aliases
      mode: Hard
    - name: functions
      source: bash/functions.sh
      target: ~/.functions
      mode: Hard
  - name: bash
    status: Unused
    os: macos
    links:
    - name: init
      source: bash/init.sh
      target: ~/.bashrc
      mode: Soft
    - name: aliases
      source: bash/aliases.sh
      target: ~/.aliases
      mode: Hard
    - name: functions
      source: bash/functions.sh
      target: ~/.functions
      mode: Hard
  - name: ssh
    status: Ready
    os: macos
    links:
      - name: config
        source: ssh/config
        target: ~/.ssh/config
        mode: Copy
  - name: vim
    status: Unused
    os: macos
    links:
      - name: config
        source: vim/config.vim
        target: ~/.vimrc
        mode: Soft
  - name: nvim
    status: Ready
    os: macos
    links:
      - name: init
        source: nvim/init.lua
        target: ~/.config/nvim/init.lua
        mode: Copy
      - name: lua dir
        source: nvim/lua/
        target: ~/.config/nvim/lua/
        mode: Copy
```

### Application configuration

Setting up `dotctl` should be a pretty straightforward process

1) Install the `dotctl` binary
2) Place is somewhere on `$PATH` so it can be executed from the command line
3) Specify the configuration file with the `use` directive

```shell
$ dotctl use <PATH-TO-CONFIG-FILE>
```

- The path should point to a file that holds contents comparable to the example above
  - Absolute paths will be found as specified
  - Relative paths will be found with the current working directory as the base path
- The configuration file will be placed at `~/.dotctl` for future use

### Application utilization

To setup an application configuration:

```shell
$ dotctl setup <CONFIG-NAME>
```

To tear down an application configuration:

```shell
$ dotctl teardown <CONFIG-NAME>
```

## Roadmap

- [ ] Hooks to assist with more complex configurations (like `nvim` or `ssh`)
- [ ] Logging for user awareness and debugging assistance
- [ ] Repository exploration like describing existing configurations or listing their current status
- [ ] Action atomicity on a invocation basis (failure should leave the file system as-is before program was invoked)
- [ ] Application configuration groups (to setup things for a work computer vs. a personal computer)
