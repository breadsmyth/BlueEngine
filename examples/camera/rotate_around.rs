/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    prelude::{Engine, ObjectSettings, ShaderSettings},
    primitive_shapes::square,
};

fn main() -> Result<(), blue_engine::error::Error> {
    // Create the engine
    let mut engine = Engine::new()?;

    // create a square
    square(
        // let's give it a name
        "Rotating Square",
        ObjectSettings {
            // and set the size
            // we need it to not cull it's back face so that it's visible on both side
            shader_settings: ShaderSettings {
                cull_mode: None,
                ..Default::default()
            },
            // and have default settings for the rest
            ..Default::default()
        },
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    let radius = 2f32;
    let start = std::time::Instant::now();

    engine.update_loop(move |engine| {
        let camx = start.elapsed().as_secs_f32().sin() * radius;
        let camz = start.elapsed().as_secs_f32().cos() * radius;
        engine.camera.set_position([camx, 0.0, camz]);
    })?;

    Ok(())
}
