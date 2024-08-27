
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Player {
    pub sofifa_id: u32,
    pub short_name: String,
    pub long_name: String,
    pub player_positions: String,
    pub nationality: String,
    pub club_name: String,
    pub league_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct RatingFile {
    pub user_id: u32,
    pub sofifa_id: u32,
    pub rating: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct RatingPlayer {
    pub sofifa_id: u32,
    pub rating_sum: f32,
    pub num_ratings: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Tags {
    pub user_id: u32,
    pub sofifa_id: u32,
    pub tag: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub user_id: u32,
    pub ratings: Vec<RatingPlayer>,
}
pub trait Identifiable {
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
pub trait AddRating<T> {
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
