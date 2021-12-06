# The info.toml configuration file
Every AdventureScript game needs a configuration file to specify some properties of the game - its name, its current version, what modules it uses, etc. This configuration file follows the TOML format.

Here is an example of what an info.toml file looks like:

```toml
name = "Some test game"
description = "Hello"
version = "0.1.1"
icon = "assets/icon.png"

[[module]]
name = "achievements"
file = "achievements.toml
```

[TODO: Add explanation / proper specifications]