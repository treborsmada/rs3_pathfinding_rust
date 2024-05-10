use rs3_pathfinding::{map_section, pathfinding, state};
use std::cmp::{max, min};
use std::time::{Instant};



fn main() {
    rs3_pathfinding::preprocessing::setup(false);
    let floor = 0;
    let start = state::State {
        pos_x: 2125,
        pos_y: 5146,
        direction: 4,
        secd: 0,
        scd: 0,
        ecd: 0,
        bdcd: 0,
    };
    let end = (2134, 5162);
    let radius = 120;
    let section = map_section::MapSection::create_map_section(min(start.pos_x as usize, end.0 as usize) - radius,
                                                              max(start.pos_x as usize, end.0 as usize) + radius,
                                                              min(start.pos_y as usize, end.1 as usize) - radius,
                                                              max(start.pos_y as usize, end.1 as usize) + radius, floor);
    let heuristic = pathfinding::Heuristic::new();
    let now = Instant::now();
    let (path, moves)  = pathfinding::a_star_end_buffer(start, end, &section, heuristic);
    let elapsed = now.elapsed();
    println!("{:?}", moves);
    println!("{:?}", path);
    println!("Elapsed: {:.2?}", elapsed);
}
