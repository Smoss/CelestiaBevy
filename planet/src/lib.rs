use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;
use kinetics::*;

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