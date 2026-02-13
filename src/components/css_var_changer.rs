use leptos::prelude::*;
use leptos_use::use_css_var;

#[component]
pub fn CssVarChanger() -> impl IntoView {
    let (spacing_sm, set_spacing_sm) = use_css_var("--spacing-sm");
    let (spacing_md, set_spacing_md) = use_css_var("--spacing-md");
    let (spacing_lg, set_spacing_lg) = use_css_var("--spacing-lg");
    let (spacing_xl, set_spacing_xl) = use_css_var("--spacing-xl");
    let (spacing_xxl, set_spacing_xxl) = use_css_var("--spacing-xxl");

    view! {
        // TODO: Styling
        <div style="display: flex; flex-direction: column; gap: 0.5rem; margin-top: 1rem;">
            <h3>"CSS Variable Playground"</h3>
            <label>
                "Spacing (small)"
                <input
                    type="range"
                    id="spacing_sm"
                    name="spacing_sm"
                    min="0"
                    max="20"
                    on:input:target=move |ev| {
                        set_spacing_sm.set(ev.target().value() + "px");
                    }
                    prop:value=move || { spacing_sm.get().trim_end_matches("px").to_string() }
                />
            </label>
            <label>
                "Spacing (medium)"
                <input
                    type="range"
                    id="spacing_md"
                    name="spacing_md"
                    min="0"
                    max="20"
                    on:input:target=move |ev| {
                        set_spacing_md.set(ev.target().value() + "px");
                    }
                    prop:value=move || { spacing_md.get().trim_end_matches("px").to_string() }
                />
            </label>
            <label>
                "Spacing (large)"
                <input
                    type="range"
                    id="spacing_lg"
                    name="spacing_lg"
                    min="0"
                    max="20"
                    on:input:target=move |ev| {
                        set_spacing_lg.set(ev.target().value() + "px");
                    }
                    prop:value=move || { spacing_lg.get().trim_end_matches("px").to_string() }
                />
            </label>
            <label>
                "Spacing (extra large)"
                <input
                    type="range"
                    id="spacing_xl"
                    name="spacing_xl"
                    min="0"
                    max="20"
                    on:input:target=move |ev| {
                        set_spacing_xl.set(ev.target().value() + "px");
                    }
                    prop:value=move || { spacing_xl.get().trim_end_matches("px").to_string() }
                />
            </label>
            <label>
                "Spacing (extra extralarge)"
                <input
                    type="range"
                    id="spacing_xxl"
                    name="spacing_xxl"
                    min="0"
                    max="64"
                    on:input:target=move |ev| {
                        set_spacing_xxl.set(ev.target().value() + "px");
                    }
                    prop:value=move || { spacing_xxl.get().trim_end_matches("px").to_string() }
                />
            </label>
        </div>
    }
}
