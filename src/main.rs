use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};

mod hash_table;

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
struct RatingFile {
    user_id: i32,
    sofifa_id: u32,
    rating: f32,
}


#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct RatingPlayer {
    sofifa_id: u32,
    rating_sum: f32,
    num_ratings: u32,
}



#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct User {
    user_id: i32,
    ratings: Vec<RatingPlayer>
}


trait Identifiable {
    fn id(&self) -> i32;
}

impl Identifiable for Player {
    fn id(&self) -> i32 {
        self.sofifa_id as i32
    }
}

impl Identifiable for User {
    fn id(&self) -> i32 {
        self.user_id as i32
    }
}

impl Identifiable for RatingFile {
    fn id(&self) -> i32 {
        self.sofifa_id as i32
    }
}

impl Identifiable for RatingPlayer {
    fn id(&self) -> i32 {
        self.sofifa_id as i32
    }
}

trait AddRating<T> {
    fn add_rating(&mut self, rating: T);
}

impl AddRating<f32> for RatingPlayer {
    fn add_rating(&mut self, rating: f32) {
        self.rating_sum += rating;
        self.num_ratings += 1;
    }
}

impl AddRating<RatingPlayer> for User {
    fn add_rating(&mut self, rating: RatingPlayer) {
        self.ratings.push(rating);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let modulo = 2000;
    let mut rating_table: hash_table::HashMap<u32, RatingPlayer> = hash_table::HashMap::new(modulo);
    let mut players_table: hash_table::HashMap<i32, Player> = hash_table::HashMap::new(modulo);
    let mut user_table: hash_table::HashMap<i32, User> = hash_table::HashMap::new(modulo);


    let x = read_csv("players.csv", |record: Player| {
        //println!("{:?}", record);
        players_table.insert(record.id(), record.clone());

        let temp = RatingPlayer{
            sofifa_id: record.sofifa_id,
            rating_sum: 0.0,
            num_ratings: 0,
        };

        rating_table.insert(record.sofifa_id, temp);

    });

    let x = read_csv("minirating.csv", |record: RatingFile| {
        //println!("{:?}", record);
        match user_table.search(&record.id()) {
            Some(user) => {
                user.ratings.push(RatingPlayer{
                    sofifa_id: record.sofifa_id,
                    rating_sum: record.rating,
                    num_ratings: 1,
                });
                //println!("{:?}", user.ratings)
            }
            None => {
                let user = User {
                    user_id: record.user_id,
                    ratings: vec![RatingPlayer{
                        sofifa_id: record.sofifa_id,
                        rating_sum: record.rating,
                        num_ratings: 1,
                    }],
                };
                user_table.insert(user.user_id, user.clone());
                //println!("{:?}", user);
            }
        }

        let mut rating = rating_table.search(&record.sofifa_id);
        rating.as_mut().unwrap().add_rating(record.rating);
        //println!("{:?}", rating);
    });


    // println!("{:?}\n", players_table.search(&158023));
    // println!("{:?}\n", rating_table.search(&158023));
    
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
