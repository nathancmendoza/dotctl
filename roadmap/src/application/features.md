# Features

## Linking

`dotter` shall provide an interface for linking up a specified configuration. If no configuration is specified, all available configuration are linked up. Configurations should be identified uniquely by a name. A named configuration will specify how to link up a particular configuration, including any specific instruction that should be executed before (pre-install commands) or after (post install commands) linking up the configuration.

### Link modes

`dotter` shall provide two different linking modes to activate a named configuration.

1) Link: create symlinks between the required location and the actual configuration location.
2) Copy: create copies of the actual configuration and place the copied configuration in the required location

The linking mode used during activation will determine how the configuration will be pruned. The locations to be used during the process will be specified in `dotter`'s own configuration on a per-configuration basis. If a configuration required more than 1 artifact to be linked or copied, that can be specified as well.

### Pre-install commands

These are shell commands that need to be executed prior to the link or copies being placed to activate a configuration. A likely scenario of this is specifying the existence of a required directory, such as `~/.config/nvim` for neovim. When activating a neovim configuration, `dotter` will ensure the configuration directory used by neovim exists. If it does not, `dotter` shall create it automatically before performing the link stage for neovim.

### Post install commands

These are shell commands that need to be executed after the link or copies being placed to activate a configuration. A likely scenario of this is running setup commands for the named configuration. For example, activating a configuration for bash would require you to run `source ~/.bashrc` in order to see the new configuration take affect.

## Pruning

`dotter` shall provide an interface for pruning an active configuration from its required location. If the configuration is symlinked, the relevant symlinks are removed. If the configuration is copied, the relevant files and directories are removed. In order to remove a configuration, it must be identified by a unique name *and* it should already be active at the time the pruning takes place. The pruning of an inactive or unrecognized configuration is **not** allowed. A named configuration will also specify how to prune a particular configuration, including any specific instruction that should be executed before (pre-cleanup commands) or after (post cleanup commands) pruning the configuration

### Prune modes

### Pre-cleanup commands

### Post cleanup commands

## System initialization

`dotter` shall provide an interface for bootstrapping a new system. This interface should only required knowing where `dotter`'s own configuration file is located. Any additional information that is needed is specified in said configuration file. After execution of the initialization command, `dotter` shall function as normal.

## Configuration status checks

`dotter` shall provide an interface to determine the status of a named configuration. A status of a configuration is two-fold. The first step is specified within a configuration block. It can take the values of `["STABLE", "DEPRECATED", "TESTING"]`. This status value refers to the state of the configuration within the "dotfile" repository. It is a statement of preparedness rather than an actual status that can be determined. A configuration that is `STABLE` will not have any issues linking or pruning. However, attempting such actions with `DEPRECATED` or `TESTING` configurations will issue a warning before proceeding. The second step of a status check is determined by its actual status of having been activated or not. This should be determined simply by the existence of the named configuration's links or copies. `dotter` shall not provide any sort of verification for individual configurations.
