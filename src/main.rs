use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::time::{Duration, Instant};
use std::io::{BufWriter, Write};
use csv::StringRecord;

#[derive(Debug, Clone)]
struct Player {
    id: i32,
    short_name: String,
    long_name: String,
    player_positions: Vec<String>,
    nationality: String,
    club_name: String,
    league_name: String,
}

trait Identifiable {
    fn id(&self) -> i32;
}

impl Identifiable for Player {
    fn id(&self) -> i32 {
        self.id
    }
}

fn main() {
    println!("Hello, world!");
}


fn insert<T: Identifiable>(item: T, modulo: usize, hash_table: &mut [Vec<T>]) {
    let index = (item.id() as usize) % modulo;
    hash_table[index].push(item);
}
fn search<T: Identifiable + Clone>(
    id: i32,
    modulo: usize,
    hash_table: &[Vec<T>],
) -> Option<T> {
    let index = (id as usize) % modulo;

    for item in &hash_table[index] {
        if item.id() == id {
            return Some(item.clone());
        }
    }
    None
}

fn read_csv<P, F>(filename: P, mut func: F) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    F: FnMut(StringRecord),
{
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        func(record);
    }
    Ok(())
}



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