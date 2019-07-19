extern crate reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
mod constants;
mod emailer;

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

fn debug_print(resp_str: &str) -> Result<(), Box<std::error::Error>> {
    let cats: Value = serde_json::from_str(&resp_str)?;
    println!("{:#?}", cats);
    Ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    emailer::send_email();
    let resp = reqwest::get(constants::ADOPT_A_PET_API_STR)?.text()?;
    // TODO: Rip this out arg logic out into its own method once we find a
    //   good framework for this in rust
    let args: Vec<String> = env::args().collect();
    // If debug is true, we print out the raw api response, with all fields
    let debug_mode = args.contains(&"debug".to_string());
    // If print_cats is true, we print out the cat json objects we've built,
    //   which contain a subset of the fields in the raw json response
    let print_cats = args.contains(&"print_cats".to_string());
    if debug_mode {
        debug_print(&resp);
    }
    println!("Args were {:?}", args);

    let cats: CatList = serde_json::from_str(&resp)?;
    if print_cats {
        for obj in cats.body {
            println!("Noot noot {:#?}", obj)
        }
    }
    Ok(())
}
