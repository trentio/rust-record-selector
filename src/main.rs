extern crate csv;
extern crate rand;
#[macro_use]
extern crate serde;

use std::error::Error;
use csv::ReaderBuilder;
use std::iter::FromIterator;
use rand::Rng;
use std::ops::Index;

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Album {
    #[serde(rename = "Artist")]
    artist: String,
    album: String,
    #[serde(rename = "greatest hits")]
    hits: String,
    #[serde(rename = "live album")]
    live: String,
    owner: String,
    notes: String,
    condition: String
}

fn main() {
    println!("{:?}", read_record_list().as_ref().map(|l| rnd_album(l)));
}


fn read_record_list() -> Result<Vec<Album>, Box<dyn Error>> {
    let reader = ReaderBuilder::new().from_path("data/vinyl_9-25-19.csv")?;
    let iter = reader.into_deserialize::<Album>().filter_map(Result::ok);
    let vec = Vec::from_iter(iter);

    Ok(vec)
}


fn rnd_album(album: &Vec<Album>) ->  &Album {
    let mut rng = rand::thread_rng();
    let record_number = rng.gen_range(0, album.len() - 1);

    let x = album.index(record_number);
    x
}
