# Data dependencies

## The configuration file

`dotter` shall be configurable with a single configuration file. This file should specify named configuration that dotter can interact with as a single unit. The naming convention and the format of the configuration file is covered in detail in the [configuration](../interaction/config.md) chapter. 

Upon initialization, the path to the configuration file is specified and it is **copied** to a predictable location. This location is likely to be the user's home directory. The copy is done to support more user control over this configuration file. Changes made to the configuration file will not be reflected in the "live" instance unless `dotter` is reinitialized. Optionally, the user may configure the initialization to create a symlink to `dotter`'s configuration instead of copying. This must be specified explicitly in the initialization command as creating a copy is the default

## The dotfile repository

This is the primary data that `dotter` shall be working with. At this time, there is no strict structure to the repository. There are only a few assumptions made.

1) The repository is a dedicated directory or location on disk. It should house the independent configuration files `dotter` will handle
2) The repository shall have a structure for uniquely identifying configuration. Basic filesystem organization should suffice, including up to a flat layout of configuration files with unique names.
3) All paths in `dotter`'s configuration file refer to the location of this repository (unless absolute, which is preferred). The path to the repository is also specified on `dotter`'s configuration file. It is the user's responsibility to maintain the truthness of location options in the application's configuration.

`dotter` shall support dotfile repositories that are structure-agnostic. Any navigation nuances are dealt with path specification within `dotter`'s configuration file. 

Note that use of the term "repository" is generalized. It simply refers to a collection of application configuration files. Although it does align with the use of a git repository for storing and tracking said configuration files. 
