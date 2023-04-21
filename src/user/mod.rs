use rocket::form::FromForm;

#[derive(FromForm)]
pub struct User<'r> {
    #[field(validate = len(1..))]
    pub username: &'r str,
}
