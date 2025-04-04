use askama::Template;

#[derive(askama::Template)]
#[template(path = "calculator/index.html")]
struct CalculatorTemplate;

pub fn index() -> String {
    let page = CalculatorTemplate {};
    page.render().unwrap()
}

pub fn style() -> &'static str {
    include_str!("../../templates/calculator/style.css")
}
