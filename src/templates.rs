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

pub fn signup_form() -> String {
    let content = html! {
        p { "Sign Up" }
        form method="POST" {
            label for="name" { "Name" }
            input type="string" name="name";
        }
    };
    content.into_string()
}
