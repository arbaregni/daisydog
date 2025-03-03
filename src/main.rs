use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Wanted: Public Menace" }</h1>
            <center>
                <img src="assets/dog.jpg" align="BOTTOM"/>
            </center>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
