use bevy::prelude::*;

use crate::ui::hud::components::*;
use crate::ui::hud::styles::*;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_resource_hud(&mut commands, &asset_server);
    build_space_ship_hud(&mut commands, &asset_server);
}

pub fn build_resource_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: get_hud_style(),
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            // LHS
            parent
                .spawn(NodeBundle {
                    style: get_lhs_style(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // // Star Image
                    // parent.spawn(ImageBundle {
                    //     style: get_image_style(),
                    //     image: asset_server.load("sprites/star.png").into(),
                    //     ..default()
                    // });
                    // Score Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Resources and influence:\n",
                                        get_text_style(&asset_server),
                                    ),
                                    TextSection::new(
                                        "",
                                        get_text_style(&asset_server),
                                    )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ScoreText {},
                    ));
                });
        })
        .id();
    hud_entity
}

pub fn build_space_ship_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: get_rhs_hud_style(),
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            // RHS
            parent
                .spawn(NodeBundle {
                    style: get_rhs_style(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            style: Style {
                                justify_self: JustifySelf::End,
                                align_self: AlignSelf::End,
                                ..default()
                            },
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Spaceships in Hex:\n",
                                        get_text_style(&asset_server),
                                    ),
                                    TextSection::new(
                                        "",
                                        get_text_style(&asset_server),
                                    )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        SpaceShipsText {},
                    ));
                });
        })
        .id();
    hud_entity
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
