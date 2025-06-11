use leptos::prelude::*;
use leptos_bevy_canvas::prelude::{event_b2l, BevyCanvas};

use crate::pages::reentry::{notification::CapsuleState, system::run_reentry};

#[component]
pub fn ReentrySimulation() -> impl IntoView {
    let (current_state, bevy_transmitter) = event_b2l::<CapsuleState>();
    let current_velocity = Signal::derive({
        let current_state = current_state.clone();
        move || current_state.get().unwrap_or_default().velocity
    });
    let current_altitude = Signal::derive({
        let current_state = current_state.clone();
        move || current_state.get().unwrap_or_default().altitude
    });

    view! {
        <main class="w-screen h-screen relative">
            <div class="absolute top-4 left-4 flex flex-col">
                <p>{move || format!("Velocity: {:.0} m/s", current_velocity.get())}</p>
                <p>{move || format!("Altitude: {:.0} m", current_altitude.get())}</p>
            </div>

            <BevyCanvas
                init=move || {
                    run_reentry(bevy_transmitter)
                }
            />
        </main>
    }
}
