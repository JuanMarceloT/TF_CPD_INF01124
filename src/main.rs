use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};
use std::cmp::Ordering;

mod hash_table;
mod trie;

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
    user_id: u32,
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
struct Tags {
    user_id: u32,
    sofifa_id: u32,
    tag: String,
}



#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct User {
    user_id: u32,
    ratings: Vec<RatingPlayer>
}


trait Identifiable {
    fn id(&self) -> u32;
}

impl Identifiable for Player {
    fn id(&self) -> u32 {
        self.sofifa_id as u32
    }
}

impl Identifiable for User {
    fn id(&self) -> u32 {
        self.user_id as u32
    }
}

impl Identifiable for RatingFile {
    fn id(&self) -> u32 {
        self.sofifa_id as u32
    }
}

impl Identifiable for RatingPlayer {
    fn id(&self) -> u32 {
        self.sofifa_id as u32
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




fn get_player_start_with(prefix: &str, trie: &mut trie::Trie, play : &mut  hash_table::HashMap<u32, Player>, rating : &mut hash_table::HashMap<u32, RatingPlayer>) -> Option<Vec<Player>>{

    let mut result = Vec::<Player>::new();

    let mut ratings = Vec::<RatingPlayer>::new();

    for name in trie.get_words_starting_with(prefix) {
        //println!("{:?}",  trie.get_id(&name));
        match trie.get_id(&name) {
            Some(n) => {
                for i in n {

                    match rating.search(&i) {
                        Some(rate ) =>{
                            // println!("{:?} {:?} {:?} ", (rate.rating_sum/(rate.num_ratings as f32)), rate.num_ratings as f32, rate.rating_sum);
                            ratings.push(rate.clone());
                        },
                        None => {}
                    }
                }
            },
            None => {
                println!("No match found for {}", name);
            }
        }
    }

    ratings.sort_by(|a, b| {
        let avg_a = if a.num_ratings > 0 {
            a.rating_sum / a.num_ratings as f32
        } else {
            0.0  // for players with no ratings
        };

        let avg_b = if b.num_ratings > 0 {
            b.rating_sum / b.num_ratings as f32
        } else {
            0.0
        };

        avg_b.partial_cmp(&avg_a).unwrap_or(Ordering::Equal)
    });

    //println!("{:?}", ratings);

    for rating in ratings {
        if let Some(player) = play.search(&rating.sofifa_id) {
             result.push(player.clone());

            let mut stars = 0.0;

            if rating.num_ratings > 0 {
                stars = rating.rating_sum/(rating.num_ratings as f32);
            }
            

             println!("{:?} {:?} {:?} {:?} {:?} {:?}", player.sofifa_id, player.short_name, player.long_name, player.player_positions, format!("{:.6}", stars), rating.num_ratings);

         }
    }

    None
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let modulo = 2000;
    let mut rating_table: hash_table::HashMap<u32, RatingPlayer> = hash_table::HashMap::new(modulo);
    let mut players_table: hash_table::HashMap<u32, Player> = hash_table::HashMap::new(modulo);
    let mut user_table: hash_table::HashMap<u32, User> = hash_table::HashMap::new(modulo);

    let mut name_index = trie::Trie::new();
    let mut tag_player = trie::Trie::new();


    let x = read_csv("players.csv", |record: Player| {
        //println!("{:?}", record);
        players_table.insert(record.id(), record.clone());

        let temp = RatingPlayer{
            sofifa_id: record.sofifa_id,
            rating_sum: 0.0,
            num_ratings: 0,
        };

        rating_table.insert(record.sofifa_id, temp);
        name_index.insert_with_id(&record.long_name, record.id());
    });

    let x = read_csv("minirating.csv", |record: RatingFile| {
        //println!("{:?}", record);
        match user_table.search(&record.user_id) {
            Some(user) => {
                user.ratings.push(RatingPlayer{
                    sofifa_id: record.sofifa_id,
                    rating_sum: record.rating,
                    num_ratings: 1,
                });
               // println!("{:?}\n\n", user)
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
    
    let x = read_csv("tags.csv", |record: Tags| {
        //println!("{:?}", record.tag);
        tag_player.insert_with_id(&record.tag, record.sofifa_id);
    });


    // get_player_start_with("fer", &mut name_index, &mut players_table, &mut rating_table);
    

    use std::io::{stdin,stdout,Write};
    let mut s=String::new();

    while s != "n" {
        s.clear();
        print!("Enter a player's name: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        get_player_start_with(&s, &mut name_index, &mut players_table, &mut rating_table);
    }
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
