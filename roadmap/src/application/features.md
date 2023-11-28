# Features

## Linking

`dotter` shall provide an interface for linking up a specified configuration. If no configuration is specified, all available configuration are linked up. Configurations should be identified uniquely by a name. A named configuration will specify how to link up a particular configuration, including any specific instruction that should be executed before (pre-install commands) or after (post install commands) linking up the configuration.

### Link modes

### Pre-install commands

### Post install commands

## Pruning

`dotter` shall provide an interface for pruning an active configuration from its required location. If the configuration is symlinked, the relevant symlinks are removed. If the configuration is copied, the relevant files and directories are removed. In order to remove a configuration, it must be identified by a unique name *and* it should already be active at the time the pruning takes place. The pruning of an inactive or unrecognized configuration is **not** allowed. A named configuration will also specify how to prune a particular configuration, including any specific instruction that should be executed before (pre-cleanup commands) or after (post cleanup commands) pruning the configuration

### Prune modes

### Pre-cleanup commands

### Post cleanup commands

## System initialization

`dotter` shall provide an interface for bootstrapping a new system. This interface should only required knowing where `dotter`'s own configuration file is located. Any additional information that is needed is specified in said configuration file. After execution of the initialization command, `dotter` shall function as normal.

## Configuration status checks

`dotter` shall provide an interface to determine the status of a named configuration. A status of a configuration is two-fold. The first step is specified within a configuration block. It can take the values of `["STABLE", "DEPRECATED", "TESTING"]`. This status value refers to the state of the configuration within the "dotfile" repository. It is a statement of preparedness rather than an actual status that can be determined. A configuration that is `STABLE` will not have any issues linking or pruning. However, attempting such actions with `DEPRECATED` or `TESTING` configurations will issue a warning before proceeding. The second step of a status check is determined by its actual status of having been activated or not. This should be determined simply by the existence of the named configuration's links or copies. `dotter` shall not provide any sort of verification for individual configurations.
