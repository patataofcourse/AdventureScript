# AdventureScript 2
## An engine for text-based games

AdventureScript is an engine and scripting language for text-based games, made with adventure games in mind, which offers a set amount of options from which the player has to choose.

This repository is a WIP for version 2 of AdventureScript, written in Rust instead of Python (which was used for 0.x and 1.x releases).

You can see the changelog [here](Changelog.md).

### Roadmap
* **Alpha prereleases**: The objective of the alpha releases is to get AdventureScript 2 working as fast as possible, speedily implementing the core and any other basic features. Alpha releases are not meant to be used for your games, since there may be breaking changes that force you to alter your script significantly.
    - alpha.1: Get the absolute bare minimum working
    - alpha.2: Multiple script files, storing variables, labels...
    - alpha.3: Saving and restoring, improve error managing, improve args managing
    - alpha.4: Finalize the AdventureScript code
    - alpha.5: CLI application for running AdventureScript games
    - alpha.6: "Title screen", achievements
    - alpha.7: A GUI for AdventureScript, using the IO system in the engine
* **Beta prereleases**: The objective of the beta releases is to patch out any remaining bugs not found on the alpha process. They will likely be few in number, and will not include any new features, although some breaking changes are bound to happen.
* **Stable 2.0 release**: This release will be the first main release for AdventureScript 2. Breaking changes from this point onward will be rare, and in the case of happening will likely only apply to saves. This is where you're recommended to start scripting.
* **Possible bugfix releases**: In the case that any serious bugs are found post-2.0, they will be addressed in bugfix releases prior to 2.1. If development for 2.1 hasn't started, any minor bugs will also be patched out in a minor 2.0.x release.
* **2.1 feature release**: This release will provide modules for quickly managing things like an inventory system and a shop system. Any feature from AdventureScript 1 not present in 2.0 will be added in 2.1 as well.
* **Future releases**: 2.1.0 won't be the end of life for AdventureScript. Both bugfix releases and feature releases will continue for an unspecified period of time. 

### Documentation
* Incomplete documentation for the AdventureScript language can be found at [adventurescript.readthedocs.io](https://adventurescript.readthedocs.io)
* Documentation for the library's public interface will be added over time before the 2.0 release

### Special thanks
* aleok for helping me learn Rust and giving me input on how to improve the code
* Villa/DuelPyro for a lot of input regarding the language and a LOT of support (and hugs)