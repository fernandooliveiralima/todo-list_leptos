use leptos::prelude::*;
use crate::types::todo::todo_type::Todo;

#[component]
pub fn ListItem(todo: Todo, on_toggle: impl Fn(i32) + 'static) -> impl IntoView {
    let (id, _set_id) = signal(todo.id);
    
    let change_input_task = move |_|{
      on_toggle(id.get());
    };

    view! {
        <li>
            <input 
                type="checkbox" 
                prop:checked= todo.done 
                on:change= change_input_task 
            />
            <label class=("task_done", move || todo.done)>
                {todo.description}
            </label>
        </li>
    }
}