use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use leptos_bevy_canvas::prelude::*;
use thaw::{ConfigProvider, Slider};
use thaw_utils::Model;

use crate::pages::waveform::{
    plugin::create_waveform, request::WaveformModification, system::Waveform,
};

#[component]
pub fn WaveformEditor() -> impl IntoView {
    view! {
        <main class="w-screen h-screen">
            <BevyWaveform dev_tools=true/>
        </main>
    }
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
                        create_waveform(modification_receiver)
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
