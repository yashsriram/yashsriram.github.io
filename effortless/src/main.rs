#[macro_use]
extern crate rocket;
use rocket::fs::{relative, NamedFile};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

mod graph;

const DEFAULT_DAG_PATH: &str = "db/latest.json";

#[get("/static/<path..>")]
async fn _static(path: PathBuf) -> Option<NamedFile> {
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
fn index() -> Template {
    Template::render("index", EmptyContext {})
}

#[get("/why")]
fn why() -> Template {
    Template::render("why", EmptyContext {})
}

#[get("/learning")]
fn learning() -> Template {
    Template::render("learning", EmptyContext {})
}

#[get("/structure")]
fn structure() -> Template {
    Template::render("structure", EmptyContext {})
}

fn open(id: &str) -> Template {
    let reader = BufReader::new(File::open(DEFAULT_DAG_PATH).unwrap());
    let dag = graph::DirectedAcyclicGraph::new(reader).unwrap();
    Template::render("open", graph::context::OpenContext::from((dag, id)))
}

#[get("/open")]
fn open_empty() -> Template {
    open("")
}

#[get("/open/<id>")]
fn open_id(id: String) -> Template {
    open(&id)
}

#[get("/create")]
fn create() -> Template {
    Template::render("create", EmptyContext {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![_static, index, why, learning, structure, open_empty, open_id, create],
        )
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
