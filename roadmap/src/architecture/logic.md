# Logic

## Configuration

### Placement

- Configuration: `~/.dotter/config`
- State information : `~/.dotter/state/*`
  - Installation information stored as `INSTALL.json`

### Validation

Configurations are allowed to be written in [yaml](https://en.wikipedia.org/wiki/YAML) or [json](https://en.wikipedia.org/wiki/JSON), with the former format being preferred. `dotter` will try to parse the configuration file as yaml, then as json. If both parse attempts fail, the configuration is declared to be invalid.

Validation should take place *anytime* the configuration is read to ensure an up-to-date parsing is valid. Once the configuration consists of valid grammar of the available specifications, the keys and values are audited. The audit of keys are values ensures the following

- Valid context use of keys
- Valid types associated with keys

If the context audit fails, the configuration is declared to be invalid. Fixes must be applied before continued use.

## Workflows

The following is a collection of flowcharts that show the execution logic of `dotter`'s commands.

### Setup

```mermaid
flowchart TD
  A[Initialize Command]
  B{Config Path Exists?}
  C[Exit Failure]
  D[Exit Success]
  E{Config Is Valid?}
  F[Place Configuration]
  G{State Directory Exists?}
  H[Remove State Directory]
  A -->|Get Path Argument| B
  B -->|No| C
  B -->|Yes| E
  E -->|No| C
  E -->|Yes| F
  F -->|Clear Previous State| G
  G -->|Yes| H
  G -->|No| D
  H --> D
```

### Install

```mermaid
flowchart TD
  A[Install Command]
  B{Config Is Valid?}
  C[Exit Failure]
  D[Exit Success]
  E{Config Is Named?}
  F{Config Supported?}
  G[Link Configuration]
  A -->|Read Config| B
  B -->|No| C
  B -->|Find Config Name| E
  E -->|No| C
  E -->|Determine Current System| F
  F -->|No| C
  F -->|Run Preinstall Hooks| G
  G -->|Run Postinstall Hooks| D
```

### Uninstall

```mermaid
flowchart TD
  A[Remove Command]
  B{Config Is Valid?}
  C[Exit Failure]
  D[Exit Success]
  E{Config Is Named?}
  F{Config Is Installed?}
  G[Unlink Configuration]
  A -->|Read Config| B
  B -->|No| C
  B -->|Find Config Name| E
  E -->|No| C
  E -->|Yes| F
  F -->|No| C
  F -->|Run Preremove Hooks| G
  G -->|Run Postremove Hooks| D
```

### Status checks

```mermaid
flowchart TD
  A[Status Command]
  B{Config Is Valid?}
  C[Exit Failure]
  D[Exit Success]
  E{Config Is Named?}
  F[Check Links Exist]
  G[Check Install Receipt]
  A -->|Read Config| B
  B -->|No| C
  B -->|Find Config Name| E
  E -->|Yes| F
  E -->|No| C
  F --> G
  G --> D
```

### Information viewing

```mermaid
flowchart TD
  A[Info Command]
  B{Config Is Valid?}
  C[Exit Failure]
  D[Exit Success]
  E{Config Is Named?}
  F[Format Info For ALL Configs]
  G[Format Info For SELECTED Config]
  A -->|Read Config| B
  B -->|No| C
  B -->|Yes| E
  E -->|No| F
  E -->|Yes| G
  F --> D
  G --> D
```
