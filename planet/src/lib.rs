use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
    pbr
};
use serde::Deserialize;
use kinetics::*;
use std::iter::FromIterator;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    InSim,
}


#[derive(Default)]
struct PlanetHandles {
    handles: Vec<(HandleUntyped, bool)>,
}

#[derive(Deserialize, TypeUuid, Debug, Clone)]
#[uuid = "1d6488f2-4b1b-4c48-be27-88e64dddf684"]
pub struct Planet {
    mass: f32,
    radius: f32,
    color: (f32, f32, f32),
    velocity: (f32, f32, f32),
    location: (f32, f32, f32),
}

impl Planet {
    pub fn insert_bundle(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let sphere_handle = meshes.add(Mesh::from(shape::Icosphere { 
            radius: self.radius,
            subdivisions: 16,
        }));
        let (r, g, b) = self.color;
        let sphere_material_handle = materials.add(StandardMaterial {
            base_color: Color::rgb(r, g, b),
            ..Default::default()
        });
        let (vx, vy, vz) = self.velocity;
        let (x, y, z) = self.location;
        commands
            .spawn()
            .insert_bundle(
                PbrBundle {
                    mesh: sphere_handle.clone(),
                    material: sphere_material_handle.clone(),
                    transform: Transform::from_xyz(x, y, z),
                    ..Default::default()
                }
            ).insert_bundle((
                Mass(self.mass),
                Velocity(Vec3::new(vx, vy, vz)),
                Gravity,
                Massive,
            ));
    }
}



#[derive(Default)]
pub struct PlanetAssetLoader;

impl AssetLoader for PlanetAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<Planet>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["planet"]
    }
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
    mut commands: Commands,
    mut planet_handles: ResMut<PlanetHandles>,
    planets_assets: Res<Assets<Planet>>,
    mut state: ResMut<State<AppState>>,
) {
    for (planet_handle, loaded_bool) in planet_handles.handles.iter_mut() {
        if !*loaded_bool {
            if let Some(planet) = planets_assets.get(&(*planet_handle)) {
                *loaded_bool = true;
                commands.spawn().insert(planet.clone());
            }
        }
    }
    if Vec::from_iter(
        planet_handles.handles.iter().filter(|(_, loaded)| { !loaded })
    ).len() == 0  {
        state.set(AppState::InSim).unwrap();
    }
}

pub struct FinishedLoadingEvent;

fn loaded(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planets: Query<&Planet>,
) {
    // Add all the planets
    for planet in planets.iter() {
        planet.insert_bundle(&mut commands, &mut meshes, &mut materials);
    }
}

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    // this is where we set up our plugin
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(AppState::Loading)
            .init_resource::<PlanetHandles>()
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(setup_loading.system()))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading.system()))
            .add_system_set(SystemSet::on_enter(AppState::InSim).with_system(loaded.system()))
            .add_asset::<Planet>()
            .add_event::<FinishedLoadingEvent>()
            .init_asset_loader::<PlanetAssetLoader>();
    }
}
