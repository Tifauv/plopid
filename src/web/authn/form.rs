use rocket::get;
use rocket_dyn_templates::Template;

#[derive(serde::Serialize)]
struct LoginContext<'a> {
    page_name: &'a str
}

#[get("/login")]
pub fn login() -> Template {
	Template::render("authn_login", &LoginContext {
        page_name: "Authentication"
    })
}
