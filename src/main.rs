use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="header">
            <h1>"LodgeMaster Social Page"</h1>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
