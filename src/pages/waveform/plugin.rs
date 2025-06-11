use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use leptos_bevy_canvas::prelude::*;

use crate::pages::waveform::{
    animate::animate_sine_wave,
    request::{handle_modification_request, WaveformModification},
    system::{setup_ui, spawn_particles, Waveform},
};

pub fn create_waveform(modification_receiver: BevyEventReceiver<WaveformModification>) -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#bevy_canvas".into()),
            transparent: true,
            decorations: false,
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
    .import_event_from_leptos(modification_receiver)
    .insert_resource(ClearColor(Color::NONE))
    .insert_resource(Waveform::default())
    .add_systems(Startup, (setup_ui, spawn_particles))
    .add_systems(
        Update,
        (handle_modification_request, animate_sine_wave).chain(),
    )
    .add_plugins(PanOrbitCameraPlugin);
    app
}
