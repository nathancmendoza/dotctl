# Purpose

"Dotfiles" are files on Unix-like operating systems (such as Linux and macOS) that are named with a leading dot (.) character, which makes them hidden by default in directory listings. These files often store configuration settings for various programs or applications. The term "dotfiles" is derived from the convention of using a dot as the prefix for these hidden configuration files.

The most common place to find a "dotfile" is the a user's home directory. This is, however, dependent on the program for which the "dotfile" is for. Some programs place them in other dedicated directorys, such as `~/.config` or in a custom location that is particular to a specific program. The nature of "dotfile" placement invites tooling to help address these inconsistency and avoid errors in placement.

For those that treasure their configurations, methods for preserving, sharing, and installing "dotfiles" are available. Most of these are crude solutions to somethings that has to be done once when setting up a new system. However, when slight tweaks or adjustments need to be made to a live system, it can be an unpleasant and error-prone process. `dotter` aims to make this a pain-free and interactive process

## Organization

The `dotter` application is not intended to help one organize their "dotfiles". Instead, it's intended role is to work alongside a collection of "dotfiles" that is already organized in some fashion.

### "Dotfiles" organization

`dotter` shall be agnostic to the organization structure of a user's "dotfiles" repository. This means that a user can organize their "dotfiles" however they choose, and `dotter` should still be able to assist them. Navigating a arbitrary organized repository needs to be specifed in `dotter`'s own configuration file. 

At a bare minimum, `dotter` expects a user to have their "dotfiles" in a centralized location. For anyone that treasures their configurations will likely already have this requirement met. However, it would be nice if `dotter` were to help user who do not yet have such a way of organizing their dotfiles.

While `dotter` is intended to assist with "dotfile" repositories at a system user level, there is a potential use case to have multiple repositories for a single user. At this time, however, there is no plan to support this.

### Configuration

The configuration for `dotter` itself serves 2 purposes

1) Provide a map to navigating a user's "dotfiles" repository
2) Specify defaults for `dotter` actions such as logging level and linking mode 

`dotter`'s configuration should also be considered a "dotfile" and can be stored with other "dotfiles" in the same repository for which in manages. This means some sort of bootstrapping process for `dotter` itself will be required. While this isn't too different for what would need to occur for installing "dotfiles" on a new system, `dotter` shall provide a well-defined process for bootstrapping itself

The syntax for `dotter`'s configurations and supported formats are discussed in the [Interaction](../interaction/config.md) section of this roadmap. 

## Flexibility

`dotter` should provide the flexibility to work with the following

- Arbitrarily organized "dotfiles" repositories
- Hotswapping of individual program configurations

### Repository structure

- Organized by application/program

```
├── git
│   └── config.toml
├── ssh
│   ├── config
├── vim
│   └── config.vim
└── zsh
    ├── aliases.zsh
    ├── config.zsh
    └── functions.zsh
```

- Organized by system type

```
├── any
│   ├── git
│   │   └── config.toml
│   └── ssh
│       └── config
├── nt
│   └── powershell
│       └── config.json
└── unix
    ├── bash
    │   └── config.sh
    ├── vim
    │   └── config.vim
    └── zsh
        └── config.zsh
```

The snippets above are a couple of example ways that a "dotfiles" repository might be organized. `dotter` should aim not to impose any particular structure, but instead will use its own configuration to navigate a user's "dotfiles" repository. 

## Automation

### Mundane tasks

`dotter` aims to provide a quick and consistent way to address the following "dotfiles" tasks

- Activating a program's configurations (symlink or copy the configuration to its required location)
- Deactivating a program's configurations (prune the symlink or remove the copied configuration file(s))
- Swapping out a program's configurations (for user's that have separate configurations for the same program/application)
- Checking the status of a program's configuration (active/inactive?)

### Bootstrapping a new system

`dotter` shall provide a well-defined process for initializing a new system

1) An initialization command where the location of the "dotfile" repository is specified and `dotter`'s configuration is discovered within
2) `dotter` links up its (via copy) configuration to be used on future runs
3) `dotter` can be used as normal

### Unattended

`dotter` tasks should be done with or without being attended do. This makes it suitable for use in other scripts or automatic deployment of computing instances. The need for superuser privileges should not be required as most use cases of "dotfiles" stay within a user's home directory. 
