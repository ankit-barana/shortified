pub mod database;
pub mod site;

#[macro_use] 
extern crate rocket;
use rocket::{response::Redirect, http::Status};
use rocket::response::content::RawHtml;
use sha2::{Digest, Sha256};  
use rusqlite::Connection;
use site::FRONT_END;
use base64;
use urlencoding::decode;

#[get("/")]
fn index() -> RawHtml<& 'static str> {
    FRONT_END
}

#[launch]
fn rocket() -> _ {
    let connection = database::open_connection().expect("Failed to open database connection");
    database::create_table(&connection).expect("Failed to initialize database");
    rocket::build()
        .mount("/", routes![index, redirect, shorten])
}

// redirects from the short url to the longer one
#[get("/<short_url>")]
fn redirect(short_url: &str) -> Result<Redirect, Status> {
    match database::open_connection() {
        Ok(connection) => {
            match database::get_long_url(&connection, short_url) {
                Ok(Some(long_url)) => {
                    Ok(rocket::response::Redirect::to(long_url))
                },
                _ => {
                    Err(Status::InternalServerError)
                },
            }
        },
        Err(_) => Err(Status::InternalServerError)
    }
}


// checks if the URL is already in the database
fn check_eixsting(connection: &Connection, url: &str) -> bool {
    match database::get_long_url(connection, url) {
        Ok(Some(_)) => true,
        _ => false,
    }
}

// shortens the url
#[post("/", data = "<url>")]
fn shorten(url: &str) -> RawHtml<String> {
    let url_noprefix = &url[4..];
    let decoded_url = decode(url_noprefix).expect("UTF-8").to_string();
    let mut hasher = Sha256::new();    
    hasher.update(decoded_url.as_bytes().to_vec());  
    let hash = hasher.finalize();      
    let hash_output = format!("{:x}", hash);
    let truncated_hash = &hash_output[..12];
    #[allow(deprecated)]
    let short_url = format!("{}", base64::encode(truncated_hash));

    let connection = database::open_connection().expect("Failed to open the database connection");
    if !check_eixsting(&connection, &short_url) {
        database::insert_url(&connection, &short_url, &decoded_url).expect("Failed to store the URL");
    }
    RawHtml(format!(
        "{}<div class='shortURL'>Short URL: {}</div>",
        FRONT_END.0, short_url
    ))
}
