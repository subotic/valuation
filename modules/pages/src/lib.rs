use askama::Template;

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub fn index() -> String {
    let page = IndexTemplate {};
    page.render().unwrap()
}