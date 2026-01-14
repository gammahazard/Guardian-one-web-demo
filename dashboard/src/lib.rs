// what: main leptos application entry point with tab-based navigation
// why: provides story-driven walkthrough of wasi 0.2 security thesis
// relations: parent of tabs/*.rs, mounts to index.html

use leptos::*;

mod tabs;

use tabs::{problem::Problem, hardware::Hardware, demo::Demo, proof::Proof};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Problem,
    Hardware,
    Demo,
    Proof,
}

#[component]
pub fn App() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal(Tab::Problem);

    view! {
        <div class="app">
            <header class="header">
                <h1>"Reliability Triad Console"</h1>
                <p class="subtitle">"Industrial Edge Security Demonstration"</p>
            </header>

            <nav class="tabs">
                <button
                    class=move || if active_tab.get() == Tab::Problem { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set(Tab::Problem)
                >
                    "The Problem"
                </button>
                <button
                    class=move || if active_tab.get() == Tab::Hardware { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set(Tab::Hardware)
                >
                    "The Hardware"
                </button>
                <button
                    class=move || if active_tab.get() == Tab::Demo { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set(Tab::Demo)
                >
                    "The Demo"
                </button>
                <button
                    class=move || if active_tab.get() == Tab::Proof { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set(Tab::Proof)
                >
                    "The Proof"
                </button>
            </nav>

            <main class="content">
                {move || match active_tab.get() {
                    Tab::Problem => view! { <Problem /> }.into_view(),
                    Tab::Hardware => view! { <Hardware /> }.into_view(),
                    Tab::Demo => view! { <Demo /> }.into_view(),
                    Tab::Proof => view! { <Proof /> }.into_view(),
                }}
            </main>

            <footer class="footer">
                <p>"Part of the Reliability Triad Portfolio"</p>
            </footer>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
