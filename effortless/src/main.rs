#[macro_use]
extern crate rocket;
use rocket::fs::{relative, NamedFile};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use std::path::{Path, PathBuf};

#[get("/static/<path..>")]
pub async fn _static(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(relative!("static")).join(path);
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct EmptyContext {}

#[catch(404)]
fn not_found() -> Template {
    Template::render("404", EmptyContext {})
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", EmptyContext {})
}

#[get("/why")]
pub fn why() -> Template {
    Template::render("why", EmptyContext {})
}

#[get("/learning")]
pub fn learning() -> Template {
    Template::render("learning", EmptyContext {})
}

#[get("/structure")]
pub fn structure() -> Template {
    Template::render("structure", EmptyContext {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![_static, index, why, learning, structure])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
