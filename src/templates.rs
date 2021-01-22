use maud::html;

pub fn category_form() -> String {
    let content = html! {
        p { "Hello" }
        form method="POST" {
            label for="comment" { "Input Some comment." }
            input type="textarea" name="name";
        }
    };
    content.into_string()
}
