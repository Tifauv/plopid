use rocket::post;
use rocket::form::{Form, FromForm, Strict};
use rocket::http::{Status, ContentType};


#[derive(FromForm)]
pub struct AuthnForm<'r> {
	u_login: &'r str,
	u_password: &'r str,
}


#[post("/login_pwd", data="<p_authn>")]
pub fn login_pwd(p_authn: Form<Strict<AuthnForm<'_>>>) -> (Status, (ContentType, String)) {
	if p_authn.u_login.eq("ptramo") && p_authn.u_password.eq("plopplopplop") {
		(Status::Ok, (ContentType::HTML, "Yay! Welcome to Pierre TRAMO, our great J2EE Architect!".to_string()))
	}
	else {
		(Status::Unauthorized, (ContentType::HTML, "Boo! We not know you!".to_string())) 
	}
}
