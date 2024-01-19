# EU4-Patcher
This tool enables features locked behind the ironman system. It calculates all necessary patches dyamically to handle new releases without the need to update this tool. 
Currently, it supports the following feaures:

- **Modded Ironman**: Allows gaining achievements whilst using mods, that change the checksum
- **Ironman Loading**: Enable the load save menu in Ironman and displays ironman saves in there to speed up "birding"
- **Midgame Ironman**:  Turn normal saves into ironman saves by hovering over the load save button in game. This does not add any earnable achievements, so this is just for testing.

To run this, download the latest exe from the releases, or build the tool yourself. Then you can apply patches by running the exe from the commandline (cmd/powershell) like this:
```
eu4-patcher.exe --patch modded-ironman,ironman-loading  --input 'C:\Program Files (x86)\Steam\steamapps\common\Europa Universalis IV\eu4.exe' 
```

If you encounter any unexpected errors, feel free to open an issue
