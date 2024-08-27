use std::cmp::Ordering;

use crate::hash_table;
use crate::structs::*;


pub fn sort_user_global_ratings(user_rating: &mut Vec<RatingPlayer>, rating_table: &hash_table::HashMap<u32, RatingPlayer>) {

    selection_sort(user_rating,|a, b| {
        let global_rating_info_a = rating_table.search_non_mut(&a.sofifa_id).unwrap();
        let global_rating_info_b = rating_table.search_non_mut(&b.sofifa_id).unwrap();

        let user_rating_a = a.rating_sum / (a.num_ratings as f32);
        let user_rating_b = b.rating_sum / (b.num_ratings as f32);

        let avg_a = if global_rating_info_a.num_ratings > 0 {
            (global_rating_info_a.rating_sum / global_rating_info_a.num_ratings as f32) + user_rating_a * 10.0
        } else {
            0.0 // for players with no ratings
        };

        let avg_b = if global_rating_info_b.num_ratings > 0 {
            (global_rating_info_b.rating_sum / global_rating_info_b.num_ratings as f32) + user_rating_b * 10.0
        } else {
            0.0
        };

        avg_b.partial_cmp(&avg_a).unwrap_or(Ordering::Equal)
    })
}

pub fn sort_user_ratings(user_rating: &mut Vec<RatingPlayer>) {
    selection_sort(user_rating, |a, b| {
        let avg_a = if a.num_ratings > 0 {
            a.rating_sum / a.num_ratings as f32
        } else {
            0.0 // for players with no ratings
        };

        let avg_b = if b.num_ratings > 0 {
            b.rating_sum / b.num_ratings as f32
        } else {
            0.0
        };

        avg_b.partial_cmp(&avg_a).unwrap_or(Ordering::Equal)
    })
}



pub fn sort_players_by_rating(players: &mut Vec<&RatingPlayer>) {

    selection_sort(players, |a, b| {
        let avg_a = if a.num_ratings > 0 {
            a.rating_sum / a.num_ratings as f32
        } else {
            0.0 // for players with no ratings
        };

        let avg_b = if b.num_ratings > 0 {
            b.rating_sum / b.num_ratings as f32
        } else {
            0.0
        };

        avg_b.partial_cmp(&avg_a).unwrap_or(Ordering::Equal)
    })
}


    pub fn selection_sort<T, F>(arr: &mut Vec<T>, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let n = arr.len();
    
        for i in 0..n {
            let mut best_index = i;
    
            for j in (i + 1)..n {
                if compare(&arr[j], &arr[best_index]) == Ordering::Less {
                    best_index = j;
                }
            }
    
            arr.swap(i, best_index);
        }
    }