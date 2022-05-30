#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate reqwest;

// use futures::executor;
use std::collections::HashMap;
use rocket_contrib::templates::Template;

#[derive(serde::Serialize)]
struct GuessContext {
    dog: &'static str,
    breed: &'static str,
    parent: &'static str
}

#[get("/")]
fn index() -> Template {
    let res = dog();
    let tup = match res {
        Ok(pair) => pair,
        Err(error) => panic!("The request didn't work, idc: {}", error),
    };
    let (int_url, int_breed) = tup;
    let image_url = string_to_static_str(int_url);
    let breed = string_to_static_str(int_breed);
    Template::render("index", &GuessContext{
        dog: image_url,
        breed: breed,
        parent: "layout",
    })
}

// Fkn hell what am i doing
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[tokio::main]
async fn dog() -> Result<(String, String), reqwest::Error> {
    let res = reqwest::get("https://dog.ceo/api/breeds/image/random").await?
    .json::<HashMap<String, String>>()
    .await?;
    let url = res.get("message").unwrap();
    // 30
    let end = &url[30..].find('/').unwrap() + 30;
    println!("{}", &url[30..]);
    let breed = String::from(&url[30..end]);
    println!("{}", breed);
    let image_url = String::from(url);

    Ok((image_url, breed))
}

fn main() {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![index]).launch();
}