use bevy::prelude::*;

use crate::pages::waveform::system::Waveform;

#[derive(Event)]
pub struct WaveformModification {
    pub amplitude: f32,
    pub wavelength: f32,
    pub omega: f32,
}

pub fn handle_modification_request(
    mut event_reader: EventReader<WaveformModification>,
    mut waveform_settings: ResMut<Waveform>,
) {
    for event in event_reader.read() {
        *waveform_settings = Waveform {
            amplitude: event.amplitude,
            wavelength: event.wavelength,
            omega: event.omega,
        }
    }
}
