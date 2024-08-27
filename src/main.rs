use regex::Regex;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::time::{Duration, Instant};

mod hash_table;
mod sort;
mod structs;
mod trie;
mod prints;

use hash_table::*;
use structs::Identifiable;
use structs::*;
use prints::*;
use trie::*;

use sort::*;

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // Made to use sofifa dataset with 22M ratings and 19k players infos
    let mut rating_table: HashMap<u32, RatingPlayer> = HashMap::new(3_000); // +- 19k inseritons
    let mut players_table: HashMap<u32, Player> = HashMap::new(3_000); // +- 19k inseritons
    let mut user_table: HashMap<u32, User> = HashMap::new(20_000); // +- 140k inseritons

    let mut name_index = Trie::new();
    let mut tag_player = Trie::new();
    let mut position_player = Trie::new();

    let start = Instant::now();

    let x = read_csv("players.csv", |record: Player| {
        players_table.insert(record.id(), record.clone());

        let temp = RatingPlayer {
            sofifa_id: record.sofifa_id,
            rating_sum: 0.0,
            num_ratings: 0,
        };

        rating_table.insert(record.sofifa_id, temp);
        name_index.insert_with_id(&record.long_name, record.id());
        let all_positons: Vec<String> = record
            .player_positions
            .split(",")
            .map(|s| s.trim()) // Remove leading and trailing whitespace
            .map(|s| s.replace(" ", "")) // Remove spaces within the strings
            .collect();

        for positions in all_positons {
            position_player.insert_with_id(&positions, record.id());
        }
    });

    let x = read_csv("rating.csv", |record: RatingFile| {
        match user_table.search(&record.user_id) {
            Some(user) => {
                user.ratings.push(RatingPlayer {
                    sofifa_id: record.sofifa_id,
                    rating_sum: record.rating,
                    num_ratings: 1,
                });
            }
            None => {
                let user = User {
                    user_id: record.user_id,
                    ratings: vec![RatingPlayer {
                        sofifa_id: record.sofifa_id,
                        rating_sum: record.rating,
                        num_ratings: 1,
                    }],
                };
                user_table.insert(user.user_id, user.clone());
            }
        }

        let mut rating = rating_table.search(&record.sofifa_id);
        rating.as_mut().unwrap().add_rating(record.rating);
    });

    let x = read_csv("tags.csv", |record: Tags| {
        tag_player.insert_with_id(&record.tag, record.sofifa_id);
    });

    let duration = start.elapsed();

    print_table_statistics(duration, &players_table, &rating_table, &user_table);

    let mut s = String::new();

    while s != "n" {
        s.clear();
        print!("> ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let mut words: Vec<&str> = parse_string(&s);

        if words.len() < 2 {
            println!("Insufficient arguments");
            words.clear();
        }

        match (words[0].to_lowercase()).as_str() {
            "player" => {
                let arg: String = remove_outer_quotes(&words[1..words.len()].join(" "));
                get_player_start_with(&arg, &mut name_index, &players_table, &rating_table);
            }
            "user" => {
                let id: Result<u32, std::num::ParseIntError> =
                    remove_outer_quotes(words[1]).parse::<u32>();

                if id.is_err() {
                    println!("Invalid user id");
                }
                if id.is_ok() {

                    if let Some(user) = &mut user_table.search(&id.unwrap()) {
                        let user_rating: &mut Vec<RatingPlayer> = &mut user.ratings;
                        sort_user_ratings(user_rating);
                        sort_user_global_ratings(user_rating, &rating_table);
    
                        print_player_and_rating_with_lim(
                            user_rating,
                            &players_table,
                            &rating_table,
                            20,
                        );
                    }else {
                        println!("User does not exist");
                    }

                }
            }
            "tags" => {
                let mut players: Vec<_> = tag_player
                    .get_id(&remove_outer_quotes(words[1]))
                    .unwrap_or_default();

                for tag in 2..words.len() {
                    if let Some(search) = tag_player.get_id(&remove_outer_quotes(words[tag])) {
                        players.retain(|id| search.contains(id));
                    }
                }

                let mut user_rating: Vec<RatingPlayer> = Vec::new();

                for player in players {
                    if let Some(n) = rating_table.search_non_mut(&player) {
                        user_rating.push(n.clone());
                    }
                }

                sort_user_ratings(&mut user_rating);
                print_player_and_rating(&user_rating, &players_table, &rating_table);
            }
            _ if words[0].to_lowercase().starts_with("top") => {
                let number_part = &words[0][3..];

                let mut top_num = 0;

                match number_part.parse::<u32>() {
                    Ok(number) => {
                        top_num = number;
                    }
                    Err(_) => println!("Invalid top number"),
                }

                let position = remove_outer_quotes(words[1]);
                let players_in_position = position_player.get_id(&position);

                let mut players_position_ratings = Vec::new();

                match players_in_position {
                    Some(player_position) => {
                        for player in player_position {
                            let rating = rating_table.search_non_mut(&player).unwrap();
                            if rating.num_ratings >= 1000 {
                                players_position_ratings.push(rating);
                            }
                        }
                        sort_players_by_rating(&mut players_position_ratings);
                        print_player_and_rating_long(
                            &players_position_ratings,
                            top_num,
                            &players_table,
                        )
                    }
                    None => {
                        println!("No players in position {}", position);
                    }
                }
            }
            _ => println!("Invalid"),
        }
    }
}

fn parse_string(input: &str) -> Vec<&str> {
    let re = Regex::new(r"'([^']*)'|\S+").unwrap();
    let mut result = Vec::new();

    for cap in re.captures_iter(input) {
        if let Some(matched) = cap.get(1) {
            result.push(matched.as_str());
        } else {
            result.push(cap.get(0).unwrap().as_str());
        }
    }

    result
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

fn remove_outer_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '"' || c == '\'').to_string()
}

pub fn get_player_start_with(
    prefix: &str,
    trie: &mut Trie,
    players_table: &HashMap<u32, Player>,
    rating_table: &HashMap<u32, RatingPlayer>,
) -> Option<Vec<Player>> {
    let mut ratings = Vec::<RatingPlayer>::new();

    for name in trie.get_words_starting_with(prefix) {
        match trie.get_id(&name) {
            Some(n) => {
                for i in n {
                    if let Some(rate) = rating_table.search_non_mut(&i) {
                        ratings.push(rate.clone());
                    }
                }
            }
            None => {
                println!("No match found for {}", name);
            }
        }
    }
    sort_user_ratings(&mut ratings);

    print_player_and_rating_with_positon(&ratings, players_table);

    None
}
