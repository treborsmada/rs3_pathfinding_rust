use pathfinding::prelude::astar;
use crate::{map_section::MapSection,
            pathfinding::Heuristic,
            state::State};

pub fn a_star_end_buffer(start: State, end: (u16, u16), map: &MapSection, heuristic: Heuristic) -> (Vec<State>, usize) {
    let result = astar(&start, |s| successors(s, map), |s| heuristic.h(s, end), |s| s.at_goal(&end)).unwrap();
    result
}

fn successors(state: &State, map: &MapSection) -> Vec<(State, usize)> {
    let mut adjacent = Vec::with_capacity(500);
    for pos in map.walk_range(state.pos_x, state.pos_y) {
        adjacent.push((state.r#move(pos.0, pos.1, pos.2).update(), 1));
    }
    if state.can_bd() {
        for pos in map.bd_range(state.pos_x, state.pos_y) {
            adjacent.push((state.bd(pos.0, pos.1, pos.2), 0));
        }
    }
    if state.can_surge() {
        adjacent.push((state.surge(&map), 0));
    }
    if state.can_escape() {
        adjacent.push((state.escape(&map), 0));
    }
    adjacent.push((state.update(), 1));
    adjacent
}
