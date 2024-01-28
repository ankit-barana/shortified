use rocket::response::content;
// 2px solid rgb(255, 81, 0) - my orange if want to use it
pub const FRONT_END: content::RawHtml<& 'static str> = content::RawHtml(include_str!("../static/index.html"));