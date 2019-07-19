extern crate reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct CatList {
    body: Vec<Cat>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cat {
    actQuickly: bool,
    age: String,
    bondedTo: u32,
    breedId: u32,
    breedName: String,
    city: String,
    clanId: u32, // What does this mean?
    description: String,
    image: String,
    name: String,
    petId: u32,
    sex: String,
    specialNeeds: bool,
    state: String,
    status: String,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut resp =
        reqwest::get("https://ra-api.adoptapet.com/v1/shelters/75215/pets/?start=1&limit=100")?
            .text()?;
    let cats: Value = serde_json::from_str(&resp)?;
    println!("{:#?}", cats);
    let cats: CatList = serde_json::from_str(&resp)?;
    for obj in cats.body {
        println!("Noot noot {:#?}", obj)
    }
    Ok(())
}
