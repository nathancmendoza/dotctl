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

## Automation
