use leptos::prelude::*;
use leptos::logging::log;
use leptos::ev::{SubmitEvent};

use crate::Todo;

#[component]
pub fn FormTask(task_list: ReadSignal<Vec<Todo>>, emitted_updated_list: WriteSignal<Vec<Todo>>) -> impl IntoView {
    let (description_task, set_description_task) = signal("".to_string());
    let (base_id, set_base_id) = signal(0);

    let generate_id = move || -> i32 {
        let mut id = set_base_id.write();
        *id += 1;
        *id
    };

    let validation_form = move || -> bool {
        let text = description_task.get();
        text.is_empty() || text.len() < 5
    };

    let submit_form = move |event: SubmitEvent|{
        event.prevent_default();
        let mut task_list_copy = task_list.get();
        if validation_form() {return};
        
        let new_task = Todo{
            id: generate_id(),
            description: description_task.get(),
            done: false
        };
        task_list_copy.push(new_task);
        log!("task_list_copy: {:?}", task_list_copy);
        emitted_updated_list.set(task_list_copy);
        set_description_task.set("".to_string());
    };

    // Exemplo: logando no console sempre que a lista recebida no Form mudar
    Effect::new(move |_| {
        log!("FormTask recebeu a lista atualizada: {:?}", task_list.get());
        log!("description_task: {}", description_task.get());
        
    });
    
    view! {
        <div class="form_container">
          <form on:submit= submit_form>
           <input 
            type="text" 
            bind:value= (description_task, set_description_task)
            placeholder="Nova Tarefa" />
            <button type="submit" >"Adicionar"</button>
            </form>
            
            <Show when=move || validation_form() fallback=move || view! { /* mostra nada se for válido */ }>
                <div class="validation_message">
                    "O campo é obrigatório, e deve conter pelo menos 5 caracteres"
                </div>
            </Show>
        </div>
    }
}