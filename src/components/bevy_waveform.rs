use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use leptos_bevy_canvas::prelude::*;
use thaw::{ConfigProvider, Slider};
use thaw_utils::Model;

#[derive(Event)]
pub struct WaveformModification {
    pub amplitude: f32,
    pub wavelength: f32,
    pub omega: f32,
}

#[component]
pub fn BevyWaveform(#[prop(optional)] dev_tools: bool) -> impl IntoView {
    let waveform_state = RwSignal::new(Waveform::default());

    let (modification_sender, modification_receiver) = event_l2b::<WaveformModification>();

    Effect::new(move |_| {
        let updated_waveform_request = waveform_state.get();

        modification_sender
            .send(WaveformModification {
                amplitude: updated_waveform_request.amplitude,
                wavelength: updated_waveform_request.wavelength,
                omega: updated_waveform_request.omega,
            })
            .ok();
    });

    let amplitude_model: Model<f64> = (
        Signal::derive(move || waveform_state.get().amplitude as f64),
        SignalSetter::map(move |updated: f64| {
            let mut new_waveform = waveform_state.get_untracked();
            new_waveform.amplitude = updated as f32;
            waveform_state.set(new_waveform);
        }),
    )
        .into();
    let wavelength_model: Model<f64> = (
        Signal::derive(move || waveform_state.get().wavelength as f64),
        SignalSetter::map(move |updated: f64| {
            let mut new_waveform = waveform_state.get_untracked();
            new_waveform.wavelength = updated as f32;
            waveform_state.set(new_waveform);
        }),
    )
        .into();
    let omega_model: Model<f64> = (
        Signal::derive(move || waveform_state.get().omega as f64),
        SignalSetter::map(move |updated: f64| {
            let mut new_waveform = waveform_state.get_untracked();
            new_waveform.omega = updated as f32;
            waveform_state.set(new_waveform);
        }),
    )
        .into();

    view! {
        <main class=move || format!("flex items-center justify-center overflow-hidden absolute inset-0 {}",
            if dev_tools {
                "z-[0]"
            } else {
                "z-[-10]"
            }
        )>
            <div class="w-full h-full">
                <BevyCanvas
                    init=move || {
                        init_bevy_app(modification_receiver)
                    }
                />
            </div>
        </main>

        <Show
            when=move || dev_tools
            fallback=|| view! {<></>}
        >
            <div class="w-[250px] h-[200px] absolute right-4 top-4 flex flex-col items-start justify-start z-[100]">
                <ConfigProvider class="w-full h-full bg-accent rounded-[10px] p-2">
                    <p class="text-xl text-white">"Amplitude"</p>
                    <Slider value=amplitude_model min=0. max=20./>

                    <p class="text-xl text-white">"Wavelength"</p>
                    <Slider value=wavelength_model min=1. max=100./>

                    <p class="text-xl text-white">"Omega"</p>
                    <Slider value=omega_model min=0. max=10./>
                </ConfigProvider>
            </div>
        </Show>
    }
}

#[derive(Component)]
struct Particle {
    position: Vec3,
}

#[derive(Resource, Clone)]
pub struct Waveform {
    pub amplitude: f32,
    pub wavelength: f32,
    pub omega: f32,
}

impl Default for Waveform {
    fn default() -> Self {
        Waveform {
            amplitude: 2.0,
            wavelength: 30.0,
            omega: 0.5,
        }
    }
}

fn init_bevy_app(modification_receiver: BevyEventReceiver<WaveformModification>) -> App {
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

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, -125., 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2000.,
    });
}

/// Calculates the unit radius for evenly distribued points inside a circle
fn radius(index: u32, total_points: u32, boundary_points: u32) -> f32 {
    if index > total_points - boundary_points {
        1.0
    } else {
        (index as f32 - 0.5).sqrt()
            / ((total_points as f32 - boundary_points as f32 + 1.0) / 2.0).sqrt()
    }
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let total_points: u32 = 5_000;
    let distribution: u32 = 1;
    let scale = 80.0;

    let boundary_points = (distribution as f32 * (total_points as f32).sqrt()) as u32;
    let phi = ((5.0_f32).sqrt() + 1.0) / 2.0;
    let golden_angle = std::f32::consts::TAU * (1.0 - 1.0 / phi);

    let mesh = meshes.add(Sphere::default());
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.208, 0.612, 1.),
        ..default()
    });

    for i in 0..total_points {
        let r = radius(i, total_points, boundary_points) * scale;
        let theta = i as f32 * golden_angle;

        let pos = Vec3::new(r * theta.cos(), r * theta.sin(), 0.0);

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(pos).with_scale(Vec3::splat(0.5)),
            Particle { position: pos },
        ));
    }
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

fn animate_sine_wave(
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
