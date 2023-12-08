# Models

## Runtime data

### Application configurations

Application configurations reflect there sections of the [Config](../interaction/config.md) file.

- Name(required): uniquely identifies it in a dotfile repository
- Status(required): ready or not ready to link
- Support Systems(required):
  - Links: Files to symlink or copy
    - Source path
    - Target path
    - Link method
  - Hooks:
    - Preinstall: run before install
    - Postinstall: run after install
    - Preremove: run before removal
    - Postremove: run after removal

### Filesystem actions

- Symlinks: Make the application configuration location point to within the dotfile repository (or somewhere else)
- Files/Directories: Make the dotfile repository data appear in the application configuration location
- Removals: Unlink existing symlinks, files, and directories

## Produced data

### Configuration setup

During the `use` command, the provided path is validated for the following

- Existence
- Configuration 

When both validation checks pass, the contents of the provided path are **copied** to `~/.dotter/config`

### Status receipts

During the `install` command, a breadcrumb JSON document is left specifying the state of installation

```JSON
{
  "dotter_version": "x.x.x",
  "options": {
    "--capture_hooks": true,
    "--link-only": "2",
    "--hook-error": "warn",
    "--verify_install": false
  },
  "date_installed": "2023-12-08 12:07",
}
```
