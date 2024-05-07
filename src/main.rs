use rs3_pathfinding::{map_section, pathfinding, state, pathfinding2_test, preprocessing};
use std::cmp::{max, min};
use std::time::Instant;
use ndarray_npy::read_npy;
use ndarray::{Array2, Order};


fn main() {
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
    let (path, moves)  = pathfinding2_test::a_star_end_buffer(start, end, &section, heuristic);
    let elapsed = now.elapsed();
    println!("{:?}", moves);
    // println!("{:?}", moves.len());
    println!("{:?}", path);
    println!("Elapsed: {:.2?}", elapsed);

}
