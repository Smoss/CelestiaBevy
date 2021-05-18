use bevy::{
    prelude::*,
    // window::WindowMode,
    pbr
};
use planet;
use kinetics;
use camera_control;

fn planets_loaded (
    mut state: ResMut<State<kinetics::AppState>>,
) {
    state.set(kinetics::AppState::InSim).unwrap();
}

fn setup(
   mut commands : Commands 
) {
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(1.0, 20.0, 10.0),
        light: pbr::Light{
            intensity: 500.0,
            range: 200.0,
            ..Default::default()
        },
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(5.0, 200.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(kinetics::GravityPlugin)
        .add_plugin(planet::PlanetPlugin)
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::on_enter(planet::AppState::InSim)
                .with_system(planets_loaded.system())
        )
        .run();
}
