use yew::{function_component, html, Html};
use crate::components::todo::todo_item::TodoItem;
use crate::components::todo::types::Todo;

#[function_component(TodoList)]
pub fn todo_list() -> Html {
  let todo_items= vec![
    Todo {
      id: 1,
      title: "Learn Rust".to_string(),
      completed: false,
    },
    Todo {
      id: 2,
      title: "Learn Yew".to_string(),
      completed: true,
    },
    Todo {
      id: 3,
      title: "Go shopping".to_string(),
      completed: false,
    },
  ];


  html! {
    <ul class="list-group">
      {todo_items.iter().map(|todo| html!{
        <TodoItem title={todo.title.clone()} completed={todo.completed} />
      }).collect::<Html>()}
    </ul>
  }
}