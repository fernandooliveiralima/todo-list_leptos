use leptos::prelude::*;
use leptos::logging::log;

use crate::components::list::todo_item::list_item::ListItem;
use crate::types::todo::todo_type::Todo;

#[component]
pub fn List(emit_list: WriteSignal<Vec<Todo>>, prop_list: ReadSignal<Vec<Todo>> ) -> impl IntoView {
    let (list, set_list) = signal(vec![
        /* Todo { id: 1, description: "Learn Rust".to_string(), done: false},
        Todo { id: 2, description: "Build a web app".to_string(), done: false} */
    ]);

    let task_done = move |id: i32|{
        let mut new_list: Vec<Todo> = prop_list.get();

        if let Some(todo) = new_list.iter_mut().find(|task| task.id == id){
            todo.done = !todo.done;
        }
        set_list.set(new_list);
        log!("List: {:?}", list.get());
    };

    Effect::new(move |_|{
        emit_list.set(list.get());
        log!("List -> : {:?}", list.get());
    });

    view! {
        <div>
            <For
                each= move || prop_list.get()
                key=|todo| (todo.id, todo.done)
                children= move |child| view! {
                    <ul>
                        <ListItem todo=child on_toggle= task_done/>
                    </ul>
                }
            />
        </div>
    }
}
