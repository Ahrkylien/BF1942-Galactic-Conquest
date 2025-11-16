# Galactic Conquest
Galactic Conquest is a full conversion Star Wars themed modification for Battlefield 1942.

In it you will travel to iconic destinations of the Galactic Civil War and the Clone Wars!\
There are no Jedi Powers in the mod, and it was conceived and originally designed in the early era of modding with full permission from LucasArts.

## Dependencies
To pack and build the mod you will need the python dependencies listed in the submodule.
At the moment of writing that is:
- [`python 3.10`](python.org/downloads/windows) (or higher)
- [python-lzo](pypi.org/project/python-lzo)

## Run via the debugger
This repository contains all the (extracted) files and history of the Mod.

In order to run the mod directly from these extracted files you need to place the debugger (`BF1942_r.exe`) and the main mod into the src folder.
Then run `build/pack lexicon and menu files.bat` to translate the lexicon and menu files to a format that the game understands.
Now you can open debugger.

## Build
To build the mod run `build/make.bat`.
