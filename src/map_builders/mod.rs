pub mod map;
pub mod bsp_dungeon;
pub mod cellular_automata;
pub mod df_aggregation;
pub mod drunkard;

use std::time::{Instant, Duration};

use self::map::Map;

pub trait MapBuilder<'a> {
    fn build(&mut self);
    fn get_map(&self) -> Map;
    fn update_map_data(&self, map_data: &mut Vec<(&'a str, String)>);
    fn iterate(&mut self);
    fn notes(&self) -> &str;
}

pub fn rebuild<'a, T: MapBuilder<'a>>(builder: &mut T) -> Duration {
    let start = Instant::now();
    builder.build();
    start.elapsed()
}

pub fn iterate<'a, T: MapBuilder<'a>>(builder: &mut T) -> Duration {
    let start = Instant::now();
    builder.iterate();
    start.elapsed()
}