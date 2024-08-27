use prettytable::{Cell, Row, Table};
use crate::structs::*;
use crate::hash_table::*;

pub fn print_player_and_rating(
    ratings: &Vec<RatingPlayer>,
    players_table: &HashMap<u32, Player>,
    rating_table: &HashMap<u32, RatingPlayer>,
) {
    let mut table: Table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("sofifa_id"),
        Cell::new("short_name"),
        Cell::new("long_name"),
        Cell::new("global_rating"),
        Cell::new("rating"),
        Cell::new("count"),
    ]));

    for player in ratings {
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

pub fn print_player_and_rating_with_lim(
    ratings: &Vec<RatingPlayer>,
    players_table: &HashMap<u32, Player>,
    rating_table: &HashMap<u32, RatingPlayer>,
    lim: usize,
) {
    let slice_len = std::cmp::min(lim, ratings.len());
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

pub fn print_player_and_rating_long(
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

pub fn print_player_and_rating_with_positon(
    ratings: &Vec<RatingPlayer>,
    players_table: &HashMap<u32, Player>,
) {
    let mut table: Table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("sofifa_id"),
        Cell::new("short_name"),
        Cell::new("long_name"),
        Cell::new("player_position"),
        Cell::new("rating"),
        Cell::new("count"),
    ]));

    for player in ratings {
        let player_infos: &Player = players_table.search_non_mut(&player.sofifa_id).unwrap();

        let rating = player.rating_sum / player.num_ratings as f32;

        table.add_row(Row::new(vec![
            Cell::new(&player_infos.sofifa_id.to_string()),
            Cell::new(&player_infos.short_name),
            Cell::new(&player_infos.long_name),
            Cell::new(&player_infos.player_positions),
            Cell::new(&format!("{:.6}", rating).to_string()),
            Cell::new(&player.num_ratings.to_string()),
        ]));
    }
    table.printstd();
}


pub fn print_table_statistics(
    duration: std::time::Duration,
    players_table: &HashMap<u32, Player>,
    rating_table: &HashMap<u32, RatingPlayer>,
    user_table: &HashMap<u32, User>,
)
{
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
    println!(
        "user table avg {:?}",
        user_table.average_bucket_length()
    );
}