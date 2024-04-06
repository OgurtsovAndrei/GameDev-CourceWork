use bevy::utils::HashMap;

fn create_setup_field_map_for_r3() -> HashMap<usize, usize> {
    let mut game_map: HashMap<usize, usize> = HashMap::new();
    let size = 36usize;


    game_map.insert(0, 2);
    game_map.insert(1, 5);
    game_map.insert(2, 5);
    game_map.insert(3, 1);
    game_map.insert(4, 5);
    game_map.insert(5, 5);
    game_map.insert(6, 0);
    game_map.insert(7, 0);
    game_map.insert(8, 5);
    game_map.insert(9, 5);
    game_map.insert(10, 3);
    game_map.insert(11, 5);
    game_map.insert(12, 5);
    game_map.insert(13, 0);
    game_map.insert(14, 5);
    game_map.insert(15, 2);
    game_map.insert(16, 5);
    game_map.insert(17, 5);
    game_map.insert(18, 4);

    for resource in game_map.clone() {
        if resource.0 != size / 2 {}
        game_map.insert(size - resource.0, resource.1);
    }

    return game_map;
}

fn create_setup_field_map_for_r2() -> HashMap<usize, usize> {
    let mut game_map: HashMap<usize, usize> = HashMap::new();
    let size = 18usize;


    game_map.insert(0, 5);
    game_map.insert(1, 0);
    game_map.insert(2, 1);
    game_map.insert(3, 3);
    game_map.insert(4, 5);
    game_map.insert(5, 5);
    game_map.insert(6, 0);
    game_map.insert(7, 5);
    game_map.insert(8, 5);
    game_map.insert(9, 4);

    for resource in game_map.clone() {
        if resource.0 != size / 2 {}
        game_map.insert(size - resource.0, resource.1);
    }

    return game_map;
}

pub fn create_setup_field_map_for_radius(radius: u32) -> HashMap<usize, usize> {
    match radius.clone() {
        2 => { return create_setup_field_map_for_r2(); }
        3 => { return create_setup_field_map_for_r3(); }
        _ => { panic!("Radius <{}> is not supported", radius) }
    }
}
