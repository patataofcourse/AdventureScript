# Getting started

## The folder structure
```none
.
├── info.toml
├── modcfg
│   └── (config files for modules go here)
├── save
│   └── (saves go here)
└── script
    ├── start.as2
    └── (scripts go here)
```

`start.as2` will be the entry point for the game, that is, the script which will be read first when running the game for the first time.

The `save` directory will only appear in the directory if the save mode is set to local/portable - otherwise saves will be in your AppData/.config directory.

For information regarding the info.toml file, check the section regarding said [configuration file](cfgfile.md).