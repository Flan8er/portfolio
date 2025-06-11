use leptos::prelude::*;
use leptos_bevy_canvas::prelude::BevyCanvas;
use thaw::{
    Button, ButtonAppearance, ConfigProvider, Dialog, DialogActions, DialogBody, DialogContent,
    DialogSurface, DialogTitle,
};

use crate::pages::galaga::system::render_game;

#[component]
pub fn GalagaGame() -> impl IntoView {
    let controls_open = RwSignal::new(true);

    view! {
        <main class="w-screen h-screen relative">
            <ConfigProvider>
                <Dialog open=controls_open>
                    <DialogSurface>
                        <DialogBody>
                            <DialogTitle>"Controls"</DialogTitle>
                            <DialogContent>
                                <p>"Pitch up: 'S'"</p>
                                <p>"Pitch down: 'W'"</p>
                                <p>"Roll right: 'D'"</p>
                                <p>"Roll left: 'A'"</p>
                                <p>"Forward: 'Left Shift'"</p>
                                <p>"Backward: 'Left Ctrl'"</p>
                                <p>"Pause: 'Esc'"</p>
                            </DialogContent>
                            <DialogActions>
                                <Button
                                    appearance=ButtonAppearance::Primary
                                    on_click=move |_| controls_open.set(false)
                                >"Gotcha!"</Button>
                            </DialogActions>
                        </DialogBody>
                    </DialogSurface>
                </Dialog>
            </ConfigProvider>

            <BevyCanvas
                init=move || {
                    render_game()
                }
            />
        </main>
    }
}
