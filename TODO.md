# Map Generation:
- [ ] Restrict terrains by latitude. Perhaps adjust gaussian in such a way, i.e. don't hard code by regions.
- [ ] Apply "gradient-trick" to noise layers.
- [ ] Implement icecaps / glaciers.
- [ ] Make convolution radius adjustable. Refactor.
- [ ] Add another layer to heightmap for highlands.
- [ ] Maybe replace all (i32, i32, i32) tuples with hexpos.

# Multiplayer:
- Allow for peer-to-peer multiplayer.
- Allow for server multiplayer.

# Data Driven Design:
- [ ] Define JSON format for map and a parser that can generate a map off of this, so people can use external map scripts.
    - [ ] Allow game to export any generated map into this JSON format.

# Camera:
- [ ] Restrict the player's view such that they can't see the ENTIRE map at once, and can infinitely scroll the map East <-> West.

# Optimizations:
- Limit all queries as much as possible. Maybe add "flag" components to entities that share similar components.