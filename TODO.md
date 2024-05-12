# Map Generation:
- [ ] Apply "gradient-trick" to noise layers.
- [ ] Implement icecaps / glaciers.
- [ ] Implement WFC post noise-generation.
- [ ] Bias terrain types based on latitude (gradient).

# Data Driven Design:
- [ ] Define JSON format for map and a parser that can generate a map off of this, so people can use external map scripts.
    - [ ] Allow game to export any generated map into this JSON format.

# Camera:
- [ ] Restrict the player's view such that they can't see the ENTIRE map at once, and can infinitely scroll the map East <-> West.