# Configuration

## Formats

### Requirements

- Syntax should be easy to write and read by a human user
- Parsing support is widely and reliable available
- Syntax must be capable to express complex key value mappings

### Keys and values

The configuration shall be separated into 2 main parts: a preferences section and a named configuration section. The preferences section stores the user's preferred values for global options. These options apply to no particular action or configuration that `dotter` works with. The named configuration section is a collection of names that map to the information required to setup or tear down the named configuration. All keys, regardless of configuration syntax in use, should follow the snake case naming semantics. Values are dependent on what information the key represents. `dotter` will be able to validate its own configuration for invalid keys or improper values.

#### Preferences

| Key | Description | Allowed Values | Default |
| --- | ----------- | -------------- | ------- |
| `log_level` | The verbosity of logging output during runtime | Strings: `["error", "warn", "info", "debug"]` | `"info"` |
| `ignore_missing` | How to respond to a missing file or directory.  | Boolean | `False` |
| `ignore_warnings` | How to respond to a warning issued by `dotter` | Boolean | `False` |
| `already_exists` | How to respond to an name clash when using `dotter` | String: `["overwrite", "confirm", "fail"]` | `"confirm"` |

> More preference option may be added if needed

#### Configurations

A configuration represents a application's necessary configuration files that need to be linked or pruned. Each configuration is divided up into 3 parts 

1) General package information
2) Linkage information
3) Configuration hooks

| Package information | Description | Allowed Values | Default |
| ------------------- | ----------- | -------------- | ------- |
| `config_name` | A user assigned name for the configuration separate from its given configuration identifier | Strings (any) | Modified version of the configuration identifier |
| `status` | A statement of preparedness for this configuration | String: `["stable", "deprecated", "testing"]` | `"stable"` |

The linkage information works on a per-path basis. Multiple links can be part of a single configuration. In effect, the value of the `links` key is a list of the following key-value mappings

| Link information | Description | Allowed Values | Default |
| ---------------- | ----------- | -------------- | ------- |
| `source` | Path to the actual configuration file | String (path-like) | None, field is required |
| `target` | Path to the configuration file's desired location | String (path-like) | None, field is required |
| `link_mode` | How to create the connection between `source` and `target` | Strings: `["link", "copy"]` | `link` |

The hooks section contains any pre-link and post link commands to run. It also hosts the pre-prune and post prune commands to run. The link hooks and prune hooks are separate keys in this section whose values are mappings as described in the following table. This section is optional and in its absence, no action is taken before or after linking or pruning.

| Hook information | Description | Allowed Values |
| ---------------- | ----------- | -------------- |
| `pre_setup` | Commands to run before setting up a configuration | Strings that are shell commands |
| `post_setup` | Commands to run after setting up a configuration | Strings that are shell commands |
| `pre_teardown` | Commands to run before deactivating a configuration | Strings that are shell commands |
| `post_teardown` | Commands to run after deactivating a configuration | Strings that are shell commands |

## Example configurations

### JSON

### TOML

### YAML
