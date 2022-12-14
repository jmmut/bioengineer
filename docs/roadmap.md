
See `git tag -n` for a list of the biggest features in each tag.
Or `git tag -n  | grep -E "^[^.]+\.[^.]+\.0"` for the big features only.

# Gameplay

- [x] basic mechanism to build machines
- [/] UI
  - [ ] top bar with "move, build, see networks, see production, see goals, help"
  - [x] game finished GUI
  - [x] tooltip when hovering over build buttons, explaining cost and purpose
  - [x] timer to grade game runs
- [x] automated resource production
  - [x] show machine status (producing X units per time unit, consuming, etc.)
  - [x] show global production
- [/] terraforming mechanics
  - [ ] cleaning water
  - [x] cleaning air
  - [ ] cleaning soil
- [/] water simulation capable of flooding your base
  - [x] communicating vessels
  - [ ] floodable floor

# Nice to haves

- [ ] highlight cells when clicking a queued task
- [x] bigger robot icon for the queue
- [ ] loading screen
- [ ] minimal friction to explain that solar panels can not be build underground
- [x] compile for linux, wasm, windows and mac
- [ ] remove from selection

# Bugs

- [ ] touchpad scroll is too sensitive
- [ ] tile transparency also makes floor transparent
