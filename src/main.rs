use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_xpbd_2d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use cb_asteroids::{
    assets::AssetsPlugin,
    controls::ControlsPlugin,
    laser_meteor_collision,
    meteors::MeteorPlugin,
    movement::MovementPlugin,
    settings::SettingsPlugin,
    start_game,
    ui::{
        choose_ship::ChooseShipPlugin,
        pause::{Pausable, PausePlugin},
        UiPlugin,
    },
    GameState,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.1)))
        .insert_resource(AssetMetaCheck::Never) // https://github.com/bevyengine/bevy/issues/10157
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Asteroids!".into(),
                    ..default()
                }),
                ..default()
            }),
            SettingsPlugin,
            ControlsPlugin,
            AssetsPlugin,
            UiPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            MeteorPlugin,
            MovementPlugin,
            ChooseShipPlugin,
            PausePlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Playing), start_game)
        .add_systems(
            Update,
            laser_meteor_collision
                .run_if(in_state(GameState::Playing))
                .run_if(resource_equals(Pausable::NotPaused)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
