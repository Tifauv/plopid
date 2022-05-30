use rocket::get;
use rocket_dyn_templates::Template;

#[derive(serde::Serialize)]
struct HomeContext<'a> {
    page_name: &'a str
}

#[get("/")]
pub fn home() -> Template {
	Template::render("home", &HomeContext {
        page_name: "Home"
    })
}
