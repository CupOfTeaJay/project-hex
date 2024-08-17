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

use crate::plugins::ui::rnd::bundles::buttons::EconomyButton;
use crate::plugins::ui::rnd::bundles::buttons::MilitaryButton;
use crate::plugins::ui::rnd::bundles::buttons::TechnologyButton;
use crate::plugins::ui::rnd::bundles::nodes::RndLandingRoot;
use crate::plugins::ui::rnd::bundles::texts::EconomyButtonText;
use crate::plugins::ui::rnd::bundles::texts::MilitaryButtonText;
use crate::plugins::ui::rnd::bundles::texts::TechnologyButtonText;

pub fn construct_rnd_landing(mut commands: Commands) {
    // Root node.
    commands
        .spawn(RndLandingRoot::new())
        .with_children(|rnd_landing_root| {
            // "Economy" button.
            rnd_landing_root
                .spawn(EconomyButton::new())
                .with_children(|economy_button| {
                    economy_button.spawn(EconomyButtonText::new());
                });
        })
        .with_children(|rnd_landing_root| {
            // "Military" button.
            rnd_landing_root
                .spawn(MilitaryButton::new())
                .with_children(|military_button| {
                    military_button.spawn(MilitaryButtonText::new());
                });
        })
        .with_children(|rnd_landing_root| {
            // "Technology" button.
            rnd_landing_root
                .spawn(TechnologyButton::new())
                .with_children(|technology_button| {
                    technology_button.spawn(TechnologyButtonText::new());
                });
        });
}
