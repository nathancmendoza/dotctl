# CLI

## Invocation

`dotter` shall be invocable from any command line or script. It will use the name `dotter` as its executable entry point and will provided a set of verbs to provide granular control of its execution. The verbs are covered in the following section. The table below describes some **global options** that can be specified. They must be provided *before* the verb and take priority over the values specified in the active `dotter` configuration in use.

| Option | Description |
|:------:| ----------- |
| `--log-level=[error,warn,info,debug]` | Control verbosity of logging to stdout |
| `--ignore-missing` | Choose to continue if a `file not found` error occurs |
| `--ignore-warnings` | Choose to continue if a warning is emitted by `dotter` |
| `--already-exists=[overwrite,confirm,fail]` | Control action taken if path exists when encountered by `dotter`|

## Verbs

### `use`

### `install`

This is the verb used to specify that `dotter` should install a named configuration. It expects a single argument, which is the named configuration to install. 

| Option | Description |
|:------:| ----------- |
| `--link-only=[INTEGER,STRING]` | Establish only a specific link. Either by order (as specified in configuration) or by name. Absence links up entire configuration |
| `--capture-hooks` | Capture stdout and stderr output of hooks run.
| `--hook-error=[fail,warn]` | Control action taken when a hook fails |
| `--verify-install` | After install process is completed, check status of configuration is "installed" |

### `remove`

This is the verb used to specify that `dotter` should remove a named configuration (if it is already installed). It expects a single argument, which is the named configuration to remove.

| Option | Description |
|:------:| ----------- |
| `--prun-only=[INTEGER,STRING]` | Remove only a specific link. Either by order (as specified in configuration) or by name. Absence prunes entire configuration |
| `--capture-hooks` | Capture stdout and stderr output of hooks run.
| `--hook-error=[fail,warn]` | Control action taken when a hook fails |
| `--verify-removal` | After removal process is completed, check status of configuration is "not installed" |

### `status`

This is the verb used to specify that `dotter` should generate a report of the current state of configurations. No argument is required, but a named configuration can be specified to narrow the report to a particular configuration instead of all of them.

| Option | Description |
|:------:| ----------- |
| `--link-state` | Retrieve the state of the configurations (installed/not installed) |
| `--last-used` | Retrieve timestamp of the last interaction (install/remove/status check) |
| `--all` | Retrieve all status fields |

### `info`

This is the verb used to get help about a named configuration that `dotter` recognized. It is an alternative method of reading `dotter`'s own configuration file. No argument is required, but a named configuration can be specified to narrow the report to a particular configuration instead of all of them.

| Option | Description |
|:------:| ----------- |
| `--availability` | Retrieve information on configuration availability on systems |
| `--links` | Retrieve information on links used by a configuration |
| `--hooks` | Retrive information on hooks used by a configuration |
| `--all` | Retrieve all information fields |
