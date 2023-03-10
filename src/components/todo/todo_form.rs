use yew::{Callback, InputEvent, MouseEvent, Properties, function_component, html, Html, use_state};

#[derive(Properties, PartialEq)]
pub struct TodoFormProps {
  pub on_add: Callback<String>
}

#[function_component(TodoForm)]
pub fn todo_item(props: &TodoFormProps) -> Html {  //イベントが発生したことを伝える。
  let title = use_state(|| "".to_string());

  let oninput = {
    let title = title.clone();
    Callback::from(move |e: InputEvent| {
      let value = e.data();

      match value {
        Some(value) => {
          title.set((*title).clone() + &value);
        }
        None => {
          title.set("".to_string());
        }
      }
    })
  };

  let onclick = {
    let on_add = props.on_add.clone();
    let title = title.clone();
    Callback::from(move |e: MouseEvent| {

      e.prevent_default(); // Web API の Event.preventDefault() と同じ
      title.set("".to_string()); // 追加ボタンを押したら入力を空に
      on_add.emit((*title).clone()); // 親にイベント伝える
      log::info!("title: {:?}", *title);
    })
  };

  html! {
    <form class="mb-5">
      <div class="mb-3">
        <label for="title" class="form-label">{"タイトル"}</label>
        <input type="text" value={(*title).clone()} {oninput} class="form-control" id="title" />
      </div>
      <div class="mb-3">
        {&*title}
      </div>
      <button type="submit" onclick={onclick} class="btn btn-primary">{"追加"}</button>
    </form>
  }
}