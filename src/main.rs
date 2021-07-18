#[macro_use]
extern crate rocket;
use comrak::{markdown_to_html, ComrakOptions};
use glob::glob;
use rocket::form::{Form, Strict};
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

mod graph;

const RELATIVE_DB_PATH: &str = "db/";
const RELATIVE_STATIC_PATH: &str = "static/";
const LATEST_DAG_NAME: &str = "latest.json";
const LATEST_SCRATCH_NAME: &str = "latest.md";

#[get("/static/<path..>")]
async fn _static(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(RELATIVE_STATIC_PATH).join(path);
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ScratchContext {
    scratch: String,
}

#[get("/scratch")]
fn scratch() -> Template {
    let content = std::fs::read_to_string(
        [RELATIVE_DB_PATH, LATEST_SCRATCH_NAME]
            .iter()
            .collect::<PathBuf>(),
    )
    .unwrap();
    let content = markdown_to_html(&content, &ComrakOptions::default());
    Template::render("scratch", ScratchContext { scratch: content })
}

#[get("/structure")]
fn structure() -> Template {
    Template::render("structure", EmptyContext {})
}

fn open(id: &str) -> Template {
    let dag = graph::DirectedAcyclicGraph::from_file(
        [RELATIVE_DB_PATH, LATEST_DAG_NAME]
            .iter()
            .collect::<PathBuf>(),
    )
    .unwrap();
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

#[derive(FromForm)]
pub struct CreateForm<'r> {
    id: &'r str,
    description: &'r str,
    significance: &'r str,
    proof: &'r str,
}

#[get("/create")]
fn create_get() -> Template {
    let dag = graph::DirectedAcyclicGraph::from_file(
        [RELATIVE_DB_PATH, LATEST_DAG_NAME]
            .iter()
            .collect::<PathBuf>(),
    )
    .unwrap();
    Template::render("create", graph::context::CreateContext::from(dag))
}

#[post("/create", data = "<nv>")]
fn create_post(nv: Form<Strict<CreateForm<'_>>>) -> Template {
    let mut dag = graph::DirectedAcyclicGraph::from_file(
        [RELATIVE_DB_PATH, LATEST_DAG_NAME]
            .iter()
            .collect::<PathBuf>(),
    )
    .unwrap();
    if let Err(msg) = dag.add_vertex(nv.id, nv.description, nv.significance, nv.proof) {
        return Template::render(
            "create",
            graph::context::CreateContext::from((nv, msg, dag)),
        );
    }
    if let Err(msg) = dag.save_to_file(
        [RELATIVE_DB_PATH, LATEST_DAG_NAME]
            .iter()
            .collect::<PathBuf>(),
    ) {
        return Template::render(
            "create",
            graph::context::CreateContext::from((nv, msg, dag)),
        );
    }
    open(nv.id)
}

#[get("/update/<id>")]
fn update_get(id: String) -> Template {
    let dag = graph::DirectedAcyclicGraph::from_file(
        [RELATIVE_DB_PATH, LATEST_DAG_NAME]
            .iter()
            .collect::<PathBuf>(),
    )
    .unwrap();
    Template::render(
        "update",
        graph::context::UpdateContext::from((dag, id.as_str())),
    )
}

#[get("/graph")]
fn _graph() -> Template {
    let dag = graph::DirectedAcyclicGraph::from_file(
        [RELATIVE_DB_PATH, LATEST_DAG_NAME]
            .iter()
            .collect::<PathBuf>(),
    )
    .unwrap();
    Template::render("graph", graph::context::GraphContext::from(dag))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct DbContext {
    json_files: Vec<String>,
    md_files: Vec<String>,
}

#[get("/db/<path..>")]
async fn db_file(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(RELATIVE_DB_PATH).join(path);
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
}

#[get("/db")]
fn db_list() -> Template {
    fn get_files(pattern: &str) -> Vec<String> {
        let mut files = glob(pattern)
            .unwrap()
            .into_iter()
            .map(|entry| match entry {
                Ok(pathbuf) => match pathbuf.to_str() {
                    Some(s) => String::from(s),
                    None => String::from("bad path, probably non UTF-8 stuff."),
                },
                Err(e) => format!("error {} for path {:?}", e.error(), e.path()),
            })
            .collect::<Vec<_>>();
        files.reverse();
        files
    }
    let json_files = get_files(&format!("{}*.json", RELATIVE_DB_PATH));
    let md_files = get_files(&format!("{}*.md", RELATIVE_DB_PATH));
    Template::render(
        "db",
        DbContext {
            json_files: json_files,
            md_files: md_files,
        },
    )
}

fn checkpoint_latest_file(filename: &str, dot_extension: &str) {
    let content =
        std::fs::read_to_string([RELATIVE_DB_PATH, filename].iter().collect::<PathBuf>()).unwrap();
    let dup_file_path = {
        let mut dup_file_name = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        dup_file_name.push_str(dot_extension);
        [RELATIVE_DB_PATH, dup_file_name.as_str()]
            .iter()
            .collect::<PathBuf>()
    };
    std::fs::write(dup_file_path, content).unwrap();
}

#[get("/checkpoint_latest_json")]
fn checkpoint_latest_json() -> Redirect {
    checkpoint_latest_file(LATEST_DAG_NAME, ".json");
    Redirect::to(uri!(db_list))
}

#[get("/checkpoint_latest_md")]
fn checkpoint_latest_md() -> Redirect {
    checkpoint_latest_file(LATEST_SCRATCH_NAME, ".md");
    Redirect::to(uri!(db_list))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                _static,
                index,
                why,
                learning,
                scratch,
                structure,
                open_empty,
                open_id,
                create_get,
                create_post,
                update_get,
                _graph,
                db_list,
                db_file,
                checkpoint_latest_json,
                checkpoint_latest_md,
            ],
        )
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
