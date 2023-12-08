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
| `name` | A user assigned name for the configuration separate from its given configuration identifier | Strings (any) | None (Optional) |
| `description` | A user description for the configuration | Strings (any) | None (Optional) |
| `status` | A statement of preparedness for this configuration | String: `["stable", "deprecated", "testing"]` | `"stable"` |
| `repository` | Absolute path to the location of the configuration repository on disk | String (path-like) | None, required field |

Linkage information is provided in the `on` table of a named configuration. Keys in this table are the system names for which the configuration is supported on. To better understand the structure, the configuration for a system can read as `{config} -> on -> {system}`. The value of this chained key specifies how to setup the configuration on *only* the named system. Absence of a system's name as a key indicates that the configuration is not supported on that system. 

The individual links are laid out on the `links` table under a specific system. This allows the link and prune process to be specified for particular system and specialized for configurations that offer cross platform support.

| Link information | Description | Allowed Values | Default |
| ---------------- | ----------- | -------------- | ------- |
| `source` | Path to the actual configuration file. If relative, will be appended to the `repository` preferenc | String (path-like) | None, field is required |
| `target` | Path to the configuration file's desired location | String (path-like) | None, field is required |
| `mode` | How to create the connection between `source` and `target` | Strings: `["link", "copy"]` | `link` |

The hooks section contains any pre-link and post link commands to run. It also hosts the pre-prune and post prune commands to run. The link hooks and prune hooks are separate keys in this section whose values are mappings as described in the following table. This section is optional and in its absence, no action is taken before or after linking or pruning. At this time, only the `run` directive is supported, which can specify shell commands to run as a subprocess of `dotter` at their respective steps on the link or prune process. Other directive support may become available later on.

| Hook information | Description | Allowed Values |
| ---------------- | ----------- | -------------- |
| `preinstall` | Commands to run before setting up a configuration | Strings that are shell commands |
| `postinstall` | Commands to run after setting up a configuration | Strings that are shell commands |
| `preremove` | Commands to run before deactivating a configuration | Strings that are shell commands |
| `postremove` | Commands to run after deactivating a configuration | Strings that are shell commands |

## Example configurations

### JSON

```JSON
{
    "preferences": {
        "repository": "~/.dotfiles",
        "log_level": "info",
        "already_exists": "overwrite"
    },
    "configurations": {
        "git": {
            "name": "Git VCS",
            "description": "commits and other stuff",
            "status": "stable",
            "on": {
                "linux": {
                    "links": {
                        "config": {
                            "source": "git/config.toml",
                            "target": "~/.gitconfig",
                            "mode": "link"
                        }
                    }
                },
                "darwin": {
                    "links": {
                        "config": {
                            "source": "git/config.toml",
                            "target": "~/.gitconfig",
                            "mode": "link"
                        }
                    }
                },
                "nt": {
                    "links": {
                        "config": {
                            "source": "git/config.toml",
                            "target": "~/.gitconfig",
                            "mode": "link"
                        }
                    }
                }
            }
        },
        "ssh": {
            "name": "Secure Shell",
            "description": "Secure Shell user configuration",
            "status": "stable",
            "on": {
                "linux": {
                    "links": {
                        "config": {
                            "source": "~/Dropbox/appconfs/ssh/config",
                            "target": "~/.ssh/config",
                            "mode": "copy"
                        },
                        "known_hosts": {
                            "source": "~/Dropbox/appconfs/ssh/known_hosts",
                            "target": "~/.ssh/known_hosts",
                            "mode": "copy"
                        }
                    },
                    "preinstall": {
                        "run": [
                            "mkdir -p ~/.ssh"
                        ]
                    },
                    "postinstall": {
                        "run": [
                            "ls -R ~/.ssh"
                        ]
                    }
                },
                "darwin": {
                    "links": {
                        "config": {
                            "source": "~/Dropbox/appconfs/ssh/config",
                            "target": "~/.ssh/config",
                            "mode": "copy"
                        },
                        "known_hosts": {
                            "source": "~/Dropbox/appconfs/ssh/known_hosts",
                            "target": "~/.ssh/known_hosts",
                            "mode": "copy"
                        }
                    },
                    "preinstall": {
                        "run": [
                            "mkdir -p ~/.ssh"
                        ]
                    },
                    "postinstall": {
                        "run": [
                            "ls -R ~/.ssh"
                        ]
                    }
                },
                "nt": {
                    "links": {
                        "config": {
                            "source": "~/Dropbox/appconfs/ssh/config",
                            "target": "~/.ssh/config",
                            "mode": "copy"
                        },
                        "known_hosts": {
                            "source": "~/Dropbox/appconfs/ssh/known_hosts",
                            "target": "~/.ssh/known_hosts",
                            "mode": "copy"
                        }
                    },
                    "preinstall": {
                        "run": [
                            "mkdir -p ~/.ssh"
                        ]
                    },
                    "postinstall": {
                        "run": [
                            "dir ~/.ssh"
                        ]
                    }
                }
            }
        },
        "bash": {
            "name": "Shell configuration",
            "status": "stable",
            "on": {
                "linux": {
                    "config": {
                        "source": "bash/config.sh",
                        "target": "~/.bashrc",
                        "mode": "link"
                    },
                    "aliases": {
                        "source": "bash/alias.sh",
                        "target": "~/.aliases",
                        "mode": "link"
                    },
                    "functions": {
                        "source": "bash/functions.sh",
                        "target": "~/.functions",
                        "mode": "link"
                    }
                }
            }
        },
        "nvim": {
            "name": "Neovim",
            "status": "stable",
            "on": {
                "linux": {
                    "links": {
                        "init": {
                            "source": "nvim/init.lua",
                            "target": "~/.config/nvim/init.lua",
                            "mode": "link"
                        },
                        "lua_dir": {
                            "source": "nvim/lua",
                            "target": "~/.config/nvim/lua",
                            "mode": "copy"
                        }
                    },
                    "preinstall": {
                        "run": [
                            "mkdir -p ~/.config/nvim"
                        ]
                    },
                    "postremove": {
                        "run": [
                            "rm -rf ~/.local/share/nvim",
                            "rm -rf ~/.local/state/nvim"
                        ]
                    }
                },
                "darwin": {
                    "links": {
                        "init": {
                            "source": "nvim/init.lua",
                            "target": "~/.config/nvim/init.lua",
                            "mode": "link"
                        },
                        "lua_dir": {
                            "source": "nvim/lua",
                            "target": "~/.config/nvim/lua",
                            "mode": "copy"
                        }
                    },
                    "preinstall": {
                        "run": [
                            "mkdir -p ~/.config/nvim"
                        ]
                    },
                    "postremove": {
                        "run": [
                            "rm -rf ~/.local/share/nvim",
                            "rm -rf ~/.local/state/nvim"
                        ]
                    }
                },
                "nt": {
                    "links": {
                        "init": {
                            "source": "nvim/init.lua",
                            "target": "~/AppData/Local/nvim/init.lua",
                            "mode": "link"
                        },
                        "lua_dir": {
                            "source": "nvim/lua",
                            "target": "~/AppData/Local/nvim/lua",
                            "mode": "copy"
                        }
                    },
                    "preinstall": {
                        "run": [
                            "mkdir -p ~/AppData/Local/nvim"
                        ]
                    },
                    "postremove": {
                        "run": [
                            "rd /s ~/AppData/Local/nvim-data"
                        ]
                    }
                }
            }
        },
        "vim": {
            "name": "Vim text editor",
            "stable": "deprecated",
            "on": {
                "linux": {
                    "links": {
                        "init": {
                            "source": "vim/config.vim",
                            "target": "~/.vimrc",
                            "mode": "copy"
                        }
                    },
                    "postremove": {
                        "run": [
                            "rm -rf ~/.vim"
                        ]
                    }
                }
            }
        }
    }
}
```

### YAML

```YAML
preferences:
  repository: ~/.dotfiles
  log_level: info
  already_exists: overwrite
configurations:
  git:
    name: Git VCS
    description: commits and other stuff
    status: stable
    'on':
      linux:
        links:
          config:
            source: git/config.toml
            target: ~/.gitconfig
            mode: link
      darwin:
        links:
          config:
            source: git/config.toml
            target: ~/.gitconfig
            mode: link
      nt:
        links:
          config:
            source: git/config.toml
            target: ~/.gitconfig
            mode: link
  ssh:
    name: Secure Shell
    description: Secure Shell user configuration
    status: stable
    'on':
      linux:
        links:
          config:
            source: ~/Dropbox/appconfs/ssh/config
            target: ~/.ssh/config
            mode: copy
          known_hosts:
            source: ~/Dropbox/appconfs/ssh/known_hosts
            target: ~/.ssh/known_hosts
            mode: copy
        preinstall:
          run:
            - mkdir -p ~/.ssh
        postinstall:
          run:
            - ls -R ~/.ssh
      darwin:
        links:
          config:
            source: ~/Dropbox/appconfs/ssh/config
            target: ~/.ssh/config
            mode: copy
          known_hosts:
            source: ~/Dropbox/appconfs/ssh/known_hosts
            target: ~/.ssh/known_hosts
            mode: copy
        preinstall:
          run:
            - mkdir -p ~/.ssh
        postinstall:
          run:
            - ls -R ~/.ssh
      nt:
        links:
          config:
            source: ~/Dropbox/appconfs/ssh/config
            target: ~/.ssh/config
            mode: copy
          known_hosts:
            source: ~/Dropbox/appconfs/ssh/known_hosts
            target: ~/.ssh/known_hosts
            mode: copy
        preinstall:
          run:
            - mkdir -p ~/.ssh
        postinstall:
          run:
            - dir ~/.ssh
  bash:
    name: Shell configuration
    status: stable
    'on':
      linux:
        config:
          source: bash/config.sh
          target: ~/.bashrc
          mode: link
        aliases:
          source: bash/alias.sh
          target: ~/.aliases
          mode: link
        functions:
          source: bash/functions.sh
          target: ~/.functions
          mode: link
  nvim:
    name: Neovim
    status: stable
    'on':
      linux:
        links:
          init:
            source: nvim/init.lua
            target: ~/.config/nvim/init.lua
            mode: link
          lua_dir:
            source: nvim/lua
            target: ~/.config/nvim/lua
            mode: copy
        preinstall:
          run:
            - mkdir -p ~/.config/nvim
        postremove:
          run:
            - rm -rf ~/.local/share/nvim
            - rm -rf ~/.local/state/nvim
      darwin:
        links:
          init:
            source: nvim/init.lua
            target: ~/.config/nvim/init.lua
            mode: link
          lua_dir:
            source: nvim/lua
            target: ~/.config/nvim/lua
            mode: copy
        preinstall:
          run:
            - mkdir -p ~/.config/nvim
        postremove:
          run:
            - rm -rf ~/.local/share/nvim
            - rm -rf ~/.local/state/nvim
      nt:
        links:
          init:
            source: nvim/init.lua
            target: ~/AppData/Local/nvim/init.lua
            mode: link
          lua_dir:
            source: nvim/lua
            target: ~/AppData/Local/nvim/lua
            mode: copy
        preinstall:
          run:
            - mkdir -p ~/AppData/Local/nvim
        postremove:
          run:
            - rd /s ~/AppData/Local/nvim-data
  vim:
    name: Vim text editor
    stable: deprecated
    'on':
      linux:
        links:
          init:
            source: vim/config.vim
            target: ~/.vimrc
            mode: copy
        postremove:
          run:
            - rm -rf ~/.vim

```
