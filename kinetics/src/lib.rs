use bevy::{
    prelude::*,
    tasks::prelude::*
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Paused,
    InSim,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum MyLabels {
    Accelerate,
}

pub struct Mass(pub f32);
pub struct Velocity(pub Vec3);
pub struct Gravity;
pub struct Massive;
const GRAV_CONSTANT: f32 = 10.0;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    // this is where we set up our plugin
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(AppState::Paused)
            .add_system_set(
                SystemSet::on_update(AppState::InSim)
                    .with_system(
                        acceleration_calculation.system()
                            .label(MyLabels::Accelerate)
                    )
                    .with_system(
                        move_objects.system()
                            .after(MyLabels::Accelerate)
                    )
            );
    }
}

fn acceleration_calculation(
    sources: Query<(&Transform, &Mass), With<Gravity>>,
    mut affected: Query<(&Transform, &mut Velocity), With<Massive>>,
    pool: Res<ComputeTaskPool>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    affected.par_for_each_mut(
        &pool,
        32,
         |(target, mut velocity)|{
            let target_loc = target.translation;
            for (source, Mass(mass)) in 
                sources.iter().filter(|(transform, _)| {*transform != target})
                {
                    let source_loc = source.translation;
                    let dist_squared = source_loc.distance_squared(target_loc);
                    let unit = (source_loc - target_loc) / dist_squared.sqrt();
                    let force =
                        (mass * GRAV_CONSTANT / dist_squared) 
                        * delta
                        * unit;
                    velocity.0 += force;
                }
        }
    );
}

fn move_objects(
    mut affected: Query<(&mut Transform, &Velocity)>,
    pool: Res<ComputeTaskPool>,
    time: Res<Time>
) {
    // println!("move");
    let delta = time.delta_seconds();
    affected.par_for_each_mut(
        &pool,
        32,
         |(mut target, velocity)|{
             let delta_l = velocity.0 * delta;
             target.translation += delta_l;
        }
    );
}