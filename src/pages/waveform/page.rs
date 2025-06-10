use leptos::prelude::*;

use crate::components::bevy_waveform::BevyWaveform;

#[component]
pub fn WaveformEditor() -> impl IntoView {
    view! {
        <main class="w-screen h-screen">
            <BevyWaveform dev_tools=true/>
        </main>
    }
}
