use bevy::{
    prelude::*,
    render,
};

fn move_camera(
    mut cameras: Query<&mut Transform, With<render::camera::Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut camera in cameras.iter_mut() {
        println!("Camera");
        let delta = time.delta_seconds();
        if keyboard_input.pressed(KeyCode::A) {
            camera.translation.x -= 10.0 * delta;
        }
        
        if keyboard_input.pressed(KeyCode::D) {
            camera.translation.x += 10.0 * delta;
        }
        
        if keyboard_input.pressed(KeyCode::W) {
            camera.translation.z -= 10.0 * delta;
        }
        
        if keyboard_input.pressed(KeyCode::S) {
            camera.translation.x += 10.0 * delta;
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    // this is where we set up our plugin
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(move_camera.system());
    }
}