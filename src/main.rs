use leptos::*;

#[component]
fn header() -> impl IntoView {
    view! {
        <div class="header">
            <h1>"LodgeMaster Social Page"</h1>
            <hr></hr>
        </div>
    }
}

#[component]
fn calendar() -> impl IntoView {
    view! {
        <h2>"Calendar"</h2>
        <table>
            <tr>
                <th>"Sunday"</th>
                <th>"Monday"</th>
                <th>"Tuesday"</th>
                <th>"Wednesday"</th>
                <th>"Thursday"</th>
                <th>"Friday"</th>
                <th>"Saturday"</th>
            </tr>
            {(0..5).into_iter().map(|_| view! {
                <tr>
                {(0..7).into_iter().map(|_| view! {
                    <td>
                    </td>
                }).collect::<Vec<_>>()}
                </tr>
            }).collect::<Vec<_>>()}
        </table>

    }
}

#[component]
fn points() -> impl IntoView {
    view! {
    <h2>Points</h2>
    <p>Attend more events for more points in your lodge!</p>

    }
}

#[component]
fn app() -> impl IntoView {
    view! {
        <Header/>
        <div class="name">
            <h1> "Luke Bushell" </h1>
            <p>"Gabriel Honor"</p>
        </div>

        <Calendar/>
        <Points/>
        <div class="side-by-side">

        <div class="upcoming">
        <h2>"Upcoming Events"</h2>
        </div>

        </div>

    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
