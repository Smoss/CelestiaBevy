use std::iter::FromIterator;
use bevy::{
    prelude::*,
    pbr,
};
use planet::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_state(AppState::Loading)
        .init_resource::<PlanetHandles>()
        .init_resource::<Planets>()
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(setup_loading.system()))
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading.system()))
        .add_system_set(SystemSet::on_enter(AppState::InSim).with_system(loaded.system()))
        .add_plugins(DefaultPlugins)
        .add_asset::<Planet>()
        .init_asset_loader::<PlanetAssetLoader>()
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Loading,
    InSim,
}


#[derive(Default)]
struct PlanetHandles {
    handles: Vec<(HandleUntyped, bool)>,
}


#[derive(Default)]
struct Planets {
    planets: Vec<Planet>,
}


fn setup_loading(
    mut state: ResMut<PlanetHandles>,
    asset_server: Res<AssetServer>,
) {
    let planet_handles: Vec<HandleUntyped> = asset_server.load_folder("planets").unwrap();
    state.handles = Vec::new();
    for planet_handle in planet_handles.iter() {
        state.handles.push((planet_handle.clone(), false));
    }
}

fn loading(
    mut planet_handles: ResMut<PlanetHandles>,
    planets_assets: Res<Assets<Planet>>,
    mut state: ResMut<State<AppState>>,
    mut planets: ResMut<Planets>,
) {
    for (planet_handle, loaded_bool) in planet_handles.handles.iter_mut() {
        if !*loaded_bool {
            if let Some(planet) = planets_assets.get(&(*planet_handle)) {
                *loaded_bool = true;
                planets.planets.push(planet.clone())
            }
        }
    }
    if Vec::from_iter(
        planet_handles.handles.iter().filter(|(_, loaded)| { !loaded })
    ).len() == 0  {
        state.set(AppState::InSim).unwrap();
    }
}

fn loaded(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planets: Res<Planets>,
) {
    
    // Add the Sun
    for planet in planets.planets.iter() {
        planet.insert_bundle(&mut commands, &mut meshes, &mut materials);
    }
    // sol.insert_bundle(&mut commands, &mut meshes, &mut materials);
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        light: pbr::Light{
            intensity: 500.0,
            range: 200.0,
            ..Default::default()
        },
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(5.0, 100.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}