
use slug;
use rocket_contrib::json::{JsonValue};

use rand::{
    distributions::Alphanumeric,
    thread_rng,
    Rng
};

#[catch(404)]
pub fn error_status() -> JsonValue {
    json!({
        "status": "Error",
        "reason": "Resource was not found {}",
    })
}

pub fn slugify(title: &str) -> String {
    if cfg!(feature = "random-suffix") {
        format!("{}-{}", 
            slug::slugify(title), 
            generate_suffix(10).to_lowercase())
    } else {
        slug::slugify(title)
    }
}

pub fn generate_suffix(len: usize)-> String {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}