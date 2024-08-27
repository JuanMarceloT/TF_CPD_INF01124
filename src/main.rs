use prettytable::{Cell, Row, Table};
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

use hash_table::*;
use structs::Identifiable;
use structs::*;

use sort::*;

fn get_player_start_with(
    prefix: &str,
    trie: &mut trie::Trie,
    play: &hash_table::HashMap<u32, Player>,
    rating: &hash_table::HashMap<u32, RatingPlayer>,
) -> Option<Vec<Player>> {
    let mut result = Vec::<Player>::new();

    let mut ratings = Vec::<RatingPlayer>::new();

    for name in trie.get_words_starting_with(prefix) {
        //println!("{:?}",  trie.get_id(&name));
        match trie.get_id(&name) {
            Some(n) => {
                for i in n {
                    match rating.search_non_mut(&i) {
                        Some(rate) => {
                            ratings.push(rate.clone());
                        }
                        None => {}
                    }
                }
            }
            None => {
                println!("No match found for {}", name);
            }
        }
    }
    sort_user_ratings(&mut ratings);

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("sofifa_id"),
        Cell::new("short_name"),
        Cell::new("long_name"),
        Cell::new("player_positions"),
        Cell::new("rating"),
        Cell::new("count"),
    ]));

    for rating in ratings {
        if let Some(player) = play.search_non_mut(&rating.sofifa_id) {
            result.push(player.clone());

            let mut stars = 0.0;

            if rating.num_ratings > 0 {
                stars = rating.rating_sum / (rating.num_ratings as f32);
            }
            table.add_row(Row::new(vec![
                Cell::new(&player.sofifa_id.to_string()),
                Cell::new(&player.short_name),
                Cell::new(&player.long_name),
                Cell::new(&player.player_positions),
                Cell::new(&format!("{:.6}", stars).to_string()),
                Cell::new(&rating.num_ratings.to_string()),
            ]));
        }
    }

    table.printstd();

    None
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // Made to use sofifa dataset with 22M ratings and 19k players infos
    let mut rating_table: hash_table::HashMap<u32, RatingPlayer> = hash_table::HashMap::new(3_000); // +- 19k inseritons
    let mut players_table: hash_table::HashMap<u32, Player> = hash_table::HashMap::new(3_000);      // +- 19k inseritons
    let mut user_table: hash_table::HashMap<u32, User> = hash_table::HashMap::new(20_000);          // +- 140k inseritons

    let mut name_index = trie::Trie::new();
    let mut tag_player = trie::Trie::new();
    let mut position_player = trie::Trie::new();
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
    println!("Tempo gasto: {:?}", duration);
    println!("player table {:?}", players_table.occupancy());
    println!("rating table {:?}", rating_table.occupancy());
    println!("user table {:?}", user_table.occupancy());
    println!(
        "player table avg {:?}",
        players_table.average_bucket_length()
    );
    println!(
        "rating table avg {:?}",
        rating_table.average_bucket_length()
    );
    println!("user table avg {:?}", user_table.average_bucket_length());

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
                    let user_rating: &mut Vec<RatingPlayer> =
                        &mut user_table.search(&id.unwrap()).unwrap().ratings;

                    sort_user_ratings(user_rating);
                    sort_user_global_ratings(user_rating, &rating_table);

                    print_player_and_rating(user_rating, &players_table, &rating_table);
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

fn print_player_and_rating(
    ratings: &Vec<RatingPlayer>,
    players_table: &HashMap<u32, Player>,
    rating_table: &HashMap<u32, RatingPlayer>,
) {
    let slice_len = std::cmp::min(20, ratings.len());
    let slice: Vec<RatingPlayer> = ratings[..slice_len].to_vec();

    let mut table: Table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("sofifa_id"),
        Cell::new("short_name"),
        Cell::new("long_name"),
        Cell::new("global_rating"),
        Cell::new("rating"),
        Cell::new("count"),
    ]));

    for player in slice {
        let player_infos: &Player = players_table.search_non_mut(&player.sofifa_id).unwrap();

        let rating = player.rating_sum / player.num_ratings as f32;

        let global_rating_info: &RatingPlayer =
            rating_table.search_non_mut(&player.sofifa_id).unwrap();

        let global_rating: f32 =
            global_rating_info.rating_sum / global_rating_info.num_ratings as f32;

        table.add_row(Row::new(vec![
            Cell::new(&player_infos.sofifa_id.to_string()),
            Cell::new(&player_infos.short_name),
            Cell::new(&player_infos.long_name),
            Cell::new(&global_rating.to_string()),
            Cell::new(&rating.to_string()),
            Cell::new(&global_rating_info.num_ratings.to_string()),
        ]));
    }
    table.printstd();
}

fn print_player_and_rating_long(
    players_position_ratings: &Vec<&RatingPlayer>,
    top_num: u32,
    players_table: &HashMap<u32, Player>,
) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("sofifa_id"),
        Cell::new("short_name"),
        Cell::new("long_name"),
        Cell::new("Player_position"),
        Cell::new("nationality"),
        Cell::new("club_name"),
        Cell::new("league_name"),
        Cell::new("rating"),
        Cell::new("count"),
    ]));

    for final_players in 0..top_num {
        let ratings: &RatingPlayer = &players_position_ratings[final_players as usize];

        let temp_player: Option<&Player> = players_table.search_non_mut(&ratings.sofifa_id);

        let rating_global = ratings.rating_sum / ratings.num_ratings as f32;

        match temp_player {
            Some(player_infos) => {
                table.add_row(Row::new(vec![
                    Cell::new(&player_infos.sofifa_id.to_string()),
                    Cell::new(&player_infos.short_name),
                    Cell::new(&player_infos.long_name),
                    Cell::new(&player_infos.player_positions),
                    Cell::new(&player_infos.nationality),
                    Cell::new(&player_infos.club_name),
                    Cell::new(&player_infos.league_name),
                    Cell::new(&format!("{:.6}", rating_global).to_string()),
                    Cell::new(&ratings.num_ratings.to_string()),
                ]));
            }
            None => {}
        }
    }
    table.printstd();
}
