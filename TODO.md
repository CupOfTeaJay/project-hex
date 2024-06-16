# On Deck:
- [ ] Implement A* pathfinding.
- [ ] Implement 'analog' (smooth) movement.
- [ ] Implement traversability by domain (Sea, Air, Land).
- [ ] Implement movement points.

# Map Generation:
- [ ] Restrict terrains by latitude. Perhaps adjust gaussian in such a way, i.e. don't hard code by regions.
- [ ] Apply "gradient-trick" to noise layers.
- [ ] Implement icecaps / glaciers.
- [ ] Convolutions
    - Refactor.
    - make convolution radius adjustable.
        - Should break up little "pockets" of terrain.
    - when equal # of counts happens for multiple terrains, randomize pick based on map seed.
        - Should break up little "pockets" of terrain.
- [ ] Add another layer to heightmap for highlands.
- [ ] Error check map paremeter inputs.

# Movement:
- [ ] Try to disjoint queries in move_thing.rs

# Multiplayer:
- Allow for peer-to-peer multiplayer.
- Allow for server multiplayer.

# Data Driven Design:
- [ ] Define JSON format for map and a parser that can generate a map off of this, so people can use external map scripts.
    - [ ] Allow game to export any generated map into this JSON format.

# Camera:
- [ ] Restrict the player's view such that they can't see the ENTIRE map at once, and can infinitely scroll the map East <-> West.

# Selection:
- [ ] Can PickSelection state be bubbled up?
