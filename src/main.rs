use yew::prelude::*;
use components::header::Header;
use components::todo::todo_list::TodoList;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <TodoList />
            </main>
        </> 
    }
}

fn main() {
    yew::start_app::<App>();
}