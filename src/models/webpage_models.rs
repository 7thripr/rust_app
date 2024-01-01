use askama::Template;

#[derive(Template, Default)]
#[template(path = "index.html")]
pub struct FormTemplate<'a> {
    name: &'a str,
    email: &'a str,
    message: &'a str,
    error_message: &'a str,
}