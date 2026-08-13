use leptos::prelude::*;
use leptos::logging::log;

mod components;
mod types;


use components::header::header::Header;
use components::list::list::List;
use components::form::form::FormTask;

use crate::types::todo::todo_type::Todo;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (output_list, set_output_list) = signal(Vec::<Todo>::new());

    // Imprime no console sempre que output_list mudar no Pai
    Effect::new(move |_| {
        log!("Pai (App) recebeu: {:?}", output_list.get());
    });

    view! { 
        <div class="container">
            <section class="area">
                <Header />
                <FormTask task_list=output_list emitted_updated_list=set_output_list/>
                <List emit_list=set_output_list prop_list=output_list/>
            </section>	
        </div> 
    }

}