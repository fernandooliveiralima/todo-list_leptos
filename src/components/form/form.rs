use leptos::prelude::*;
use leptos::logging::log;
use crate::Todo;

#[component]
pub fn FormTask(task_list: ReadSignal<Vec<Todo>>) -> impl IntoView {

    // Exemplo: logando no console sempre que a lista recebida no Form mudar
    Effect::new(move |_| {
        log!("FormTask recebeu a lista atualizada: {:?}", task_list.get());
    });
    
    view! {
        <div class="form_container">
          <form>
           <input type="text" placeholder="Nova Tarefa" />
           <button type="submit">"Adicionar"</button>
          </form>
        </div>
    }
}