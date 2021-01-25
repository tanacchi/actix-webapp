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

pub fn signin_form() -> String {
    let content = html! {
        p { "Sign In" }
        form method="POST" {
            label for="name" { "Name" }
            input type="string" name="name";
        }
    };
    content.into_string()
}

pub fn dashboard(logged_in: bool) -> String {
    let content = html! {
        h1 { "ProChart" }
        @if logged_in {
            p { "Welcome" }
            p {
                a href="/" { "Top" }
            }
            p {
                a href="/signout" { "SignOut" }
            }
            p {
                a href="/reports/new" { "New Report" }
            }
            p {
                a href="/categories" { "Category List" }
            }
            p {
                a href="/categories/new" { "New Category" }
            }
            p {
                a href="/signout" { "SignOut" }
            }
        } @else {
            p { "Please login." }
            p {
                a href="/signin" { "SignIn" }
            }
            p {
                a href="/signup" { "SignUp" }
            }
            p {
                a href="/users" { "User List" }
            }
        }
    };
    content.into_string()
}

use crate::models::Category;
pub fn report_form(logged_in: bool, categories: Vec<Category>) -> String {
    let content = html! {
        h1 { "New Report" }
        @if logged_in {
            form method="POST" {
                label for="date"  { "Date" }
                input type="date" name="date";

                label for="comment" { "Comment" }
                input type="textarea" name="comment";


                label for="category" { "Category" }
                select name="category" {
                    @for category in &categories {
                        option value=(category.id) {
                            (category.name)
                        }
                    }
                }
            }
        } @else {
            p { "Please login." }
            p {
                a href="/signin" { "SignIn" }
            }
        }
    };
    content.into_string()
}
