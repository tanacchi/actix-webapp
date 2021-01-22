use maud::html;

pub fn category_form() -> String {
    let content = html! {
        p { "Hello" }
        form {
            label for="comment" { "Input Some comment." }
            input type="textarea" name="comment";
        }
    };
    content.into_string()
}
