/*
    Project Hex
    Copyright (C) 2024 Clevermeld™ LLC

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

use bevy::prelude::*;
use rand::seq::SliceRandom;

const NUM_NAMES: usize = 100;

#[derive(Resource)]
pub struct CityNames<'a> {
    names: [&'a str; NUM_NAMES],
}

impl CityNames<'static> {
    pub fn new() -> Self {
        CityNames {
            names: [
                "Ravenscar",
                "Elderglen",
                "Frosthaven",
                "Shadowfen",
                "Gloomridge",
                "Emberfield",
                "Thornvale",
                "Cinderbrook",
                "Ashenwood",
                "Mistvale",
                "Silverhold",
                "Windermere",
                "Brightspire",
                "Crescentmoor",
                "Duskwood",
                "Ironforge",
                "Moonlit Hollow",
                "Glimmerstone",
                "Thistledown",
                "Fablewood",
                "Cindervale",
                "Wraithmere",
                "Goldenhollow",
                "Starwatch",
                "Emberwatch",
                "Hollowvale",
                "Mossy Glen",
                "Ravenwood",
                "Frostglen",
                "Crimsonwood",
                "Whispering Hollow",
                "Ebonvale",
                "Thornridge",
                "Dawnwatch",
                "Cobalt Hollow",
                "Shadowspire",
                "Mirthvale",
                "Frostfire Keep",
                "Elysium Grove",
                "Cinderhollow",
                "Brightvale",
                "Wolfsbane",
                "Glenhaven",
                "Emberstone",
                "Hearthvale",
                "Silverbrook",
                "Moonshadow",
                "Ravensreach",
                "Thornwood",
                "Galehaven",
                "Frostholm",
                "Dewhaven",
                "Twilight Hollow",
                "Ironvale",
                "Starfall",
                "Elderwood",
                "Cindermoor",
                "Whispering Pines",
                "Gloomhaven",
                "Emberglade",
                "Crescent Hollow",
                "Misty Hollow",
                "Wraithwood",
                "Frostvale",
                "Brightwood",
                "Hollowbrook",
                "Cinderkeep",
                "Ashenvale",
                "Ravenstone",
                "Shadowbrook",
                "Emberfall",
                "Thornspire",
                "Glenwood",
                "Wolfswood",
                "Crimsonvale",
                "Moonstone Keep",
                "Fablehaven",
                "Ironwood",
                "Elysian Vale",
                "Duskvale",
                "Starhaven",
                "Frosthaven",
                "Cinderhaven",
                "Thornmoor",
                "Whispering Glade",
                "Glimmerforge",
                "Silverlake",
                "Dewdrop Hollow",
                "Ebonwood",
                "Brightstone",
                "Ravenswood",
                "Cinderforge",
                "Whispering Spire",
                "Emberglint Hollow",
                "Thornshadow Keep",
                "Celestial Wyrmwood",
                "Frostfire Citadel",
                "Moonlit Vale of Echoes",
                "Starlit Bastion of Dreams",
                "Crimson Veil Enclave",
            ],
        }
    }

    pub fn get_random_name(&self) -> String {
        self.names
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}
