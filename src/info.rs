use std::env;
use std::option::Option;
///Returns the repl id
#[must_use]
pub fn id() -> Option<String> {
    match env::var("REPL_ID") {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}
///Returns the repl slug
#[must_use]
pub fn slug() -> Option<String> {
    match env::var("REPL_SLUG") {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}
///Returns the owner of the repl
#[must_use]
pub fn owner() -> Option<String> {
    match env::var("REPL_OWNER") {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}
///Returns the language of the repl
#[must_use]
pub fn language() -> Option<String> {
    match env::var("REPL_LANGUAGE") {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}
///Returns the <{id}.id.repl.co/>
#[must_use]
pub fn id_co_url() -> Option<String> {
    id().map(|n| "https://".to_string() + &n + ".id.repl.co")
}
/// Returns the <{slug}.{owner}.repl.co/>
#[must_use]
pub fn co_url() -> Option<String> {
    let repl_slug: String;
    match slug() {
        Some(n) => repl_slug = n,
        None => return None,
    }
    owner().map(|n| {
        format!(
            "https://{}.{}.repl.co",
            repl_slug.to_lowercase(),
            n.to_lowercase()
        )
    })
}
/// Returns the <https://replit.com/@{owner}/{slug}/>
#[must_use]
pub fn replit_url() -> Option<String> {
    let repl_slug: String;
    match slug() {
        Some(n) => repl_slug = n,
        None => return None,
    }
    owner().map(|n| "https://replit.com/@".to_string() + &n + "/" + &repl_slug)
}
/// Returns the <https://replit.com/replid/{id}/>
#[must_use]
pub fn replit_id_url() -> Option<String> {
    id().map(|n| "https://replit.com/replid/".to_string() + &n)
}
