
See `git tag -n` for a list of the biggest features in each tag.
Or `git tag -n  | grep -E "^[^.]+\.[^.]+\.0"` for the big features only.

# Gameplay

- [/] UI
  - [/] top bar with "move, build, see networks, see production, see goals, help"
  - [x] game finished GUI
  - [x] tooltip when hovering over build buttons, explaining cost and purpose
  - [x] timer to grade game runs
- [/] automated resource production
  - [/] show machine status (producing X units per time unit, consuming, animated in the map, etc.)
  - [/] show global production
- [/] terraforming mechanics
  - [ ] cleaning water
  - [x] cleaning air
  - [ ] cleaning soil
- [/] water simulation capable of flooding your base
  - [x] communicating vessels
  - [/] floodable floor
- [ ] putting life
  - [x] put plants
  - [ ] put animals
  - [ ] put other life ??
- [/] Story dialogs

# Nice to haves

- [x] compile for linux, wasm, windows and mac
- UI
  - [x] highlight and count cells when clicking a queued task
  - [x] count cells while selecting (I want to build X tiles)
  - [x] bigger robot icon for the queue
  - [x] loading screen
  - [x] minimal friction to explain that solar panels can not be build underground
  - [x] remove from selection
  - [ ] SPACE accepts current pop up
- mechanics
  - [x] build dumpster that creates columns of ~stairs to dump~ dug rock
  - [ ] allow building rock in the air and make it fall

# Bugs

- [ ] touchpad scroll is too sensitive
- [-] tile transparency also makes floor transparent. Solved: remove floors
- [ ] dirt can be converted to rock. is this wrong?
- [ ] macroquad forces all windows except 1 to be inactive and darker
- [ ] egui draws textures with the wrong alpha. I suspect egui can use the textures loaded
      by macroquad but interprets the alpha channel wrong. Like, assuming it is premultiplied
      when it's not. I might be very wrong on this.
- [ ] after clicking some transformation, the map gets a cell selected, while
      in the macroquad UI the cell that got the transformation keeps the selection.
- [x] removing several cells that may split the network creates a broken state.
      Fixed by not allowing splitting the network, but still applies if natural disasters destroy machines.
- [ ] flooded machines still work
- [ ] storage machines work without power
- 
