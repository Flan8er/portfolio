use bevy::prelude::*;

use crate::pages::waveform::system::{Particle, Waveform};

pub fn animate_sine_wave(
    time: Res<Time>,
    mut query: Query<(&Particle, &mut Transform)>,
    waveform_settings: Res<Waveform>,
) {
    let t = time.elapsed_secs();

    let amplitude = waveform_settings.amplitude; // wave height
    let wavelength = waveform_settings.wavelength; // peak-to-peak distance
    let omega = waveform_settings.omega; // wave propagation speed

    let k = std::f32::consts::TAU / wavelength; // spatial frequency
    for (particle, mut transform) in &mut query {
        let x = particle.position.x;
        let y = particle.position.y;
        let r = (x * x + y * y).sqrt();

        let phase = k * r + omega * t;
        let z = amplitude * phase.sin();

        transform.translation = Vec3::new(x, y, z);
    }
}
