# General:
- [ ] Document as much as possible.
- [ ] Test everything.
- [ ] Add panic branch for anything that returns Option.
- [ ] Prefer pattern matching over if / else if / else if... as much as possible.

# Components to add:

# Systems to update:
- [ ] More organic camera zoom. Only adjusting FOV looks quite odd.
- [ ] Cleanup adjust_for_latitude as much as possible.
- [ ] Check for incompatible tiles and adjust weights in generate_map.

# Systems to add:

# Plugins to update:

# Map Generation
- [ ] Scaffold::bias_tile, Make well defined!
- [ ] Make algorithms as modular as possible. This allows for mixing algorithms easily.
    - [ ] Fun API for generating hexagonal maps.
- [ ] Neighbors for tiles on the left and right sides of the map should "wrap".
- [ ] Can we use string slices instead of String for domain keys?
    - Maybe just use the enums!
- [ ] Cleanup map generation algo upon satisfactory completion.
- [ ] Maybe one day make latitude a gradient instead of 5 distinct regions.
