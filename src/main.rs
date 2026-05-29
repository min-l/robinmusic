#[macro_use] extern crate rocket;
//use rocket::fs::NamedFile;
use rocket::fs::FileServer;
use rocket::fs::Options;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use std::path::Path;
//use std::path::PathBuf;
use std::fs;
//use system_shutdown::reboot;
use rocket::serde::{Serialize, json::Json};
use std::error::Error;
use rocket::form::FromForm;
use rocket::form::Form;


const SLSK_FOLDER: &str = "/home/minty/projects/robinpage/album_test";
const JF_FOLDER: &str = "/home/minty/projects/robinpage";

/*
#[get("/")]
async fn mainpage() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}
    
*/

#[get("/")]
fn mainpage() -> Template {
    let folders = fs::read_dir(SLSK_FOLDER).unwrap();
    let mut folders_vec = Vec::new();

    for folder in folders {
        folders_vec.push(folder.unwrap().path());
    }

    let a_folders = fs::read_dir(JF_FOLDER).unwrap();
    let mut a_folders_vec = Vec::new();

    for folder in a_folders {
        let item = folder.unwrap().path().display().to_string();
        let (_,artist) = item.rsplit_once("/").unwrap();
        a_folders_vec.push(artist.to_owned());
    }



    Template::render("index", context! {
        file_names: folders_vec,
        artist_names: a_folders_vec
    })
}


#[get("/album/<folder>")]
fn get_album_contents(folder: &str) -> Json<Vec<String>> {
    let mut folders_vec = Vec::new();

    let mut dir_loc: String = SLSK_FOLDER.to_owned();
    dir_loc.push_str("/");
    dir_loc.push_str(folder);

    let folders_result = fs::read_dir(dir_loc);
    let folders = match folders_result {
        Ok(path) => path,
        Err(error) => return Json(folders_vec)
    };
    

    for folder in folders {
        folders_vec.push(folder.unwrap().path().display().to_string());
    }

    Json(folders_vec)
}

//#[post("/reboot")]

#[derive(FromForm)]
struct MoveAction<'r> {
    #[field(validate = len(1..))]
    album_folder: &'r str,
    #[field(validate = len(1..))]
    artist_choice: &'r str
}

#[post("/move", data = "<move_action>")]
fn move_folder(move_action : Form<MoveAction<'_>>) {
    let mut from_loc: String = SLSK_FOLDER.to_owned();
    from_loc.push_str("/");
    
    let mut album = move_action.album_folder;
    if album.contains("/") {
        (_,album) = move_action.album_folder.rsplit_once("/").unwrap();
    }
    
    
    from_loc.push_str(album);

    let mut to_loc: String = JF_FOLDER.to_owned();
    to_loc.push_str("/");

    let mut artist = move_action.artist_choice;
    if artist.contains("/") {
        (_,artist) = move_action.artist_choice.rsplit_once("/").unwrap();
    };

    to_loc.push_str(artist);

    if !(Path::new(&to_loc).exists()) {
        fs::create_dir(&to_loc).unwrap();
    };

    to_loc.push_str("/");
    to_loc.push_str(album);

    println!("from: {} to: {}",from_loc,to_loc);

    fs::rename(from_loc,to_loc).unwrap();
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/fs/",FileServer::new(SLSK_FOLDER,Options::None))
        .mount("/", routes![mainpage,get_album_contents,move_folder])
        .attach(Template::fairing())
}
