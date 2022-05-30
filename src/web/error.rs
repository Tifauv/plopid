use rocket_dyn_templates::Template;

/**
 * Error page generation
 */
#[derive(serde::Serialize)]
pub struct ErrorContext<'a> {
	page_name: &'a str,
	error_code: u16,
	error_message: &'a str,
	error_details: &'a str,
}

pub fn error_page(p_code: u16, p_message: &str, p_details: &str) -> Template {
	Template::render("error", &ErrorContext {
		page_name: "Error",
		error_code: p_code,
		error_message: p_message,
		error_details: p_details,
	})
}
