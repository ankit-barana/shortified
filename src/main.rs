pub mod database;

#[macro_use] 
extern crate rocket;
use rocket::{response::Redirect, http::Status};
use rocket::response::content;
use sha2::{Digest, Sha256};  
use rusqlite::Connection;
use base64;

#[get("/")]
fn index() -> content::RawHtml<& 'static str> {
    content::RawHtml(r#"
        <style>
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
                color: black;
            }
        
            body {
                background-color: #1B1717;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100vh;
            }
        
            .logo {
                color: #EEEBDD;
                font-weight: bold;
                font-size: 4em;
                margin-bottom: 30px;
            }

            .tagline {
                color: #DFDCCB;
                font-size: 100%;
            }
            .wrapper {
                display: flex;
                flex-direction: column;
                align-items: center;
                margin-bottom: 20px;
            }
        
            .wrapper input {
                background: #2e2727;
                border: 2px solid rgb(255, 81, 0);
                border-radius: 5px;
                padding: 1rem 2rem;
                width: 20vw;
                margin-bottom: 20px;
            }
        </style>
        <body>
            <div class="logo">
                <h1 >Shortified!</h1>
                <p class="tagline">Official Selection: Programming Failures of The Century</p>   
            </div>
            <div class="wrapper">
                <input type="text" placeholder="URL goes here" width="20vw"/>
                <button>
                    shorten
                </button>
            </div>
        </body>
    "#)
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
fn shorten(url: &str) -> String {
    let mut hasher = Sha256::new();    
    hasher.update(url.as_bytes().to_vec());  
    let hash = hasher.finalize();      
    let hash_output = format!("{:x}", hash);
    let truncated_hash = &hash_output[..12];
    let short_url = format!("t.ax/{}", base64::encode(truncated_hash));

    let connection = database::open_connection().expect("Failed to open the database connection");
    if !check_eixsting(&connection, &short_url) {
        database::insert_url(&connection, &short_url, &url).expect("Failed to store the URL");
    } 
    short_url
}
