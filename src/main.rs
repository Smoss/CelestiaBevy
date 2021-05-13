use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
    pbr,
};
use serde::Deserialize;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_asset::<Planet>()
        .init_asset_loader::<PlanetAssetLoader>()
        .add_startup_system(setup.system())
        .run();
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
            println!("{:?}", custom_asset);
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["planet"]
    }
}

pub struct Mass(f32);
pub struct Velocity(Vec3);
pub struct Gravity;


#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "1d6488f2-4b1b-4c48-be27-88e64dddf684"]
pub struct Planet {
    mass: f32,
    radius: f32,
    color: (f32, f32, f32),
    velocity: (f32, f32, f32),
    location: (f32, f32, f32),
}

impl Planet {
    fn insert_bundle(
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
            ));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planets_assets: Res<Assets<Planet>>,
) {
    // let sphere_handle = meshes.add(Mesh::from(shape::Icosphere { 
    //     radius: 2.0,
    //     subdivisions: 16,
    // }));
    // let sphere_material_handle = materials.add(StandardMaterial {
    //     base_color: Color::rgb(1.0, 1.0, 1.0),
    //     ..Default::default()
    // });
    // let sol = Planet{
    //     mass: 100.0,
    //     radius: 20.0,
    //     color: (1.0, 1.0, 1.0),
    //     velocity: (0.0, 0.0, 0.0),
    //     location: (0.0, 0.0, 0.0),
    // };
    let planet_handles: Vec<HandleUntyped> = asset_server.load_folder("planets").unwrap();
    println!("{}", planet_handles.len());
    for planet_handle in planet_handles.iter() {
        println!("{:?}", planet_handle);
        let planet = planets_assets.get(planet_handle).unwrap();
        planet.insert_bundle(&mut commands, &mut meshes, &mut materials);
    }
    // let sol_handle: Handle<Planet> = asset_server.load("planets/Sol.planet");
    // println!("{:?}", planets_assets.get(&sol_handle));
    // Add the Sun
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