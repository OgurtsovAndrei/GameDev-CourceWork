use rand::prelude::SliceRandom;
use rand::{Rng, thread_rng};
use crate::space_ships::{SpaceShip, SpaceShipCharacteristics};
use crate::world::player::Player;

pub(in crate::world::actions::move_menu) fn move_army_to_planet(attack_player: Player, defense_player: Player, mut attack_player_army: Vec<SpaceShip>, mut defense_player_army: Vec<SpaceShip>) -> (Player, Vec<SpaceShip>) {
    if attack_player.id == defense_player.id {
        attack_player_army.append(&mut defense_player_army);
        (attack_player, attack_player_army)
    } else {
        perform_fight(attack_player, defense_player, attack_player_army, defense_player_army)
    }
}

pub(in crate::world::actions::move_menu) fn perform_fight(attack_player: Player, defense_player: Player, mut attack_player_army: Vec<SpaceShip>, mut defense_player_army: Vec<SpaceShip>) -> (Player, Vec<SpaceShip>) {
    let mut rng = thread_rng();
    attack_player_army.shuffle(&mut rng);
    defense_player_army.shuffle(&mut rng);

    assert!(!attack_player_army.is_empty() || !defense_player_army.is_empty());

    while !attack_player_army.is_empty() && !defense_player_army.is_empty() {
        let attack_ship = attack_player_army.pop().unwrap();
        let defense_ship = defense_player_army.pop().unwrap();
        let attack_dice = rng.gen_range(0..10);
        let defense_dice = rng.gen_range(0..10);
        let coeff_attack = SpaceShipCharacteristics::get_by_spaceship_type(attack_ship.ship_type).power;
        let coeff_defense = SpaceShipCharacteristics::get_by_spaceship_type(defense_ship.ship_type).power;
        if attack_dice * coeff_attack > defense_dice * coeff_defense {
            attack_player_army.push(attack_ship);
        } else if attack_dice * coeff_attack == defense_dice * coeff_defense {
            attack_player_army.push(attack_ship);
            defense_player_army.push(defense_ship);
        } else {
            defense_player_army.push(defense_ship);
        }
    }
    if attack_player_army.is_empty() {
        (defense_player, defense_player_army)
    } else {
        (attack_player, attack_player_army)
    }
}