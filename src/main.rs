use bevy::prelude::*;
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // !!!!IMPORTANT!!!! Add the OverlayPlugin here
        .add_plugin(OverlayPlugin { font_size: 32.0, ..default() })
        .add_startup_system(setup)
        .add_system(screen_print_text)
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn screen_print_text(time: Res<Time>) {
    let current_time = time.elapsed_seconds_f64();
    let at_interval = |t: f64| current_time % t < time.delta_seconds_f64();
    screen_print!(col: Color::CYAN, "GRID MASTER");
    if at_interval(0.1) {
        let last_fps = 1.0 / time.delta_seconds();
        screen_print!(col: Color::CYAN, "fps: {last_fps:.0}");
    }
}
