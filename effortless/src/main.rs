#[macro_use]
extern crate rocket;
use glob::glob;
use rocket::fs::NamedFile;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

mod graph;

const RELATIVE_DB_PATH: &str = "db/";
const RELATIVE_STATIC_PATH: &str = "static/";
const LATEST_DAG_NAME: &str = "latest.json";

#[get("/static/<path..>")]
async fn _static(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(RELATIVE_STATIC_PATH).join(path);
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
}

#[get("/db/<path..>")]
async fn db_file(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(RELATIVE_DB_PATH).join(path);
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
    let reader = BufReader::new(
        File::open(
            [RELATIVE_DB_PATH, LATEST_DAG_NAME]
                .iter()
                .collect::<PathBuf>(),
        )
        .unwrap(),
    );
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

#[get("/graph")]
fn _graph() -> Template {
    let reader = BufReader::new(
        File::open(
            [RELATIVE_DB_PATH, LATEST_DAG_NAME]
                .iter()
                .collect::<PathBuf>(),
        )
        .unwrap(),
    );
    let dag = graph::DirectedAcyclicGraph::new(reader).unwrap();
    Template::render("graph", graph::context::GraphContext::from(dag))
}

#[get("/db")]
fn db_list() -> Template {
    for entry in glob(&format!("{}*", RELATIVE_DB_PATH)).expect("could not open db path.") {
        match entry {
            Ok(path) => println!("p: {:?}", path.display()),
            Err(e) => println!("p: {:?}", e),
        }
    }
    Template::render("db", EmptyContext {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                _static, index, why, learning, structure, open_empty, open_id, create, _graph,
                db_list, db_file
            ],
        )
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
