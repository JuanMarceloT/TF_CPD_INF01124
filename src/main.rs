use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct Player {
    sofifa_id: u32,
    short_name: String,
    long_name: String,
    player_positions: String,
    nationality: String,
    club_name: String,
    league_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct Star {
    user_id: i32,
    sofifa_id: u32,
    rating: f32,
}

trait Identifiable {
    fn id(&self) -> i32;
}

impl Identifiable for Player {
    fn id(&self) -> i32 {
        self.sofifa_id as i32
    }
}

impl Identifiable for Star {
    fn id(&self) -> i32 {
        self.sofifa_id as i32
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let modulo = 2000;
    let mut hash_table: Vec<Vec<Star>> = initialize_hash_table(modulo);

    let x = read_csv("rating.csv", |record| {
        //println!("{:?}", record);
        insert(record, modulo, &mut hash_table);
    });
}

fn initialize_hash_table<T: Clone>(modulo: usize) -> Vec<Vec<T>> {
    vec![Vec::new(); modulo]
}

fn insert<T: Identifiable>(item: T, modulo: usize, hash_table: &mut [Vec<T>]) {
    let index = (item.id() as usize) % modulo;
    hash_table[index].push(item);
}
fn search<T: Identifiable + Clone>(id: i32, modulo: usize, hash_table: &[Vec<T>]) -> Option<T> {
    let index = (id as usize) % modulo;

    for item in &hash_table[index] {
        if item.id() == id {
            return Some(item.clone());
        }
    }
    None
}

fn read_csv<P, F, T>(filename: P, mut func: F) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    F: FnMut(T),
    T: DeserializeOwned,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut rdr = csv::Reader::from_reader(reader);

    for result in rdr.deserialize() {
        let record: T = result?;
        func(record);
    }
    Ok(())
}

#[allow(dead_code)]
fn measure_time<F>(func: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    func();
    let duration = start.elapsed();
    //println!("Tempo gasto: {:?}", duration);
    duration
}
