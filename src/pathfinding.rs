use std::cmp::{Reverse, max};
use std::collections::HashMap;
use ndarray::Array5;
use ndarray_npy::read_npy;
use radix_heap::RadixHeapMap;
use crate::map_section::MapSection;
use crate::state::State;

fn reconstruct_path( mut came_from: HashMap<State, (State, &str)>, current_state: State) -> (Vec<State>, Vec<String>){
    let mut path = Vec::new();
    path.push(current_state.clone());
    let mut moves = Vec::new();
    let mut current_state = current_state;
    let mut current_move;
    loop {
        (current_state, current_move) = came_from.remove(&current_state).unwrap();
        path.push(current_state.clone());
        if moves.is_empty() || current_move == "walk" || current_move == "wait" {
            moves.push(String::from(current_move));
        } else {
            let temp = moves.pop().unwrap();
            moves.push(format!("{} {}", current_move, temp));
        }
        if !came_from.contains_key(&current_state) {
            break;
        }
    }
    (path.into_iter().rev().collect(), moves.into_iter().rev().collect())
}

pub fn a_star_end_buffer(start: State, end: (u16, u16), map: &MapSection, heuristic: Heuristic) -> (Vec<State>, Vec<String>){
    let mut queue: RadixHeapMap<Reverse<usize>, State> = RadixHeapMap::new();
    let mut g_score: HashMap<State, usize> = HashMap::new();
    let mut came_from: HashMap<State, (State, &str)> = HashMap::new();
    g_score.insert(start.clone(), 0);
    queue.push(Reverse(0), start.clone());
    let mut count = 0;
    while !queue.is_empty() {
        let current_node =  queue.pop().unwrap().1;
        count += 1;
        if current_node.at_goal(&end) {
            println!("{count}");
            return reconstruct_path(came_from, current_node);
        }
        let tentative_g_score = g_score.get(&current_node).unwrap() + 1;
        let next_node = current_node.update();
        if !g_score.contains_key(&next_node) || tentative_g_score < *g_score.get(&next_node).unwrap() {
            came_from.insert(next_node.clone(), (current_node.clone(), "wait"));
            g_score.insert(next_node.clone(), tentative_g_score);
            let f_score = tentative_g_score + heuristic.h(&next_node, end);
            queue.push(Reverse(f_score), next_node.clone());
        }
        let walk_range = map.walk_range_2(current_node.pos_x, current_node.pos_y);
        for pos in walk_range {
            let next_node = current_node.r#move(pos.0, pos.1, pos.2).update();
            if !g_score.contains_key(&next_node) || tentative_g_score < *g_score.get(&next_node).unwrap() {
                came_from.insert(next_node.clone(), (current_node.clone(), "walk"));
                g_score.insert(next_node.clone(), tentative_g_score);
                let f_score = tentative_g_score + heuristic.h(&next_node, end);
                queue.push(Reverse(f_score), next_node.clone());
            }
        }
        let tentative_g_score = *g_score.get(&current_node).unwrap();
        if current_node.can_surge() {
            let next_node = current_node.surge(&map);
            if !g_score.contains_key(&next_node) || tentative_g_score < *g_score.get(&next_node).unwrap() {
                came_from.insert(next_node.clone(), (current_node.clone(), "surge"));
                g_score.insert(next_node.clone(), tentative_g_score);
                let f_score = tentative_g_score + heuristic.h(&next_node, end);
                queue.push(Reverse(f_score), next_node.clone());
            }
        }
        if current_node.can_bd() {
            let bd_range = map.bd_range_2(current_node.pos_x, current_node.pos_y);
            for pos in bd_range {
                let next_node = current_node.bd(pos.0, pos.1, pos.2);
                if !g_score.contains_key(&next_node) || tentative_g_score < *g_score.get(&next_node).unwrap() {
                    came_from.insert(next_node.clone(), (current_node.clone(), "bd"));
                    g_score.insert(next_node.clone(), tentative_g_score);
                    let f_score = tentative_g_score + heuristic.h(&next_node, end);
                    queue.push(Reverse(f_score), next_node.clone());
                }
            }
        }
        if current_node.can_escape() {
            let next_node = current_node.escape(&map);
            if !g_score.contains_key(&next_node) || tentative_g_score < *g_score.get(&next_node).unwrap() {
                came_from.insert(next_node.clone(), (current_node.clone(), "escape"));
                g_score.insert(next_node.clone(), tentative_g_score);
                let f_score = tentative_g_score + heuristic.h(&next_node, end);
                queue.push(Reverse(f_score), next_node.clone());
            }
        }
    }
    panic!();
}

pub struct Heuristic {
    data: Array5<u64>,
}

impl Heuristic {
    pub fn new() -> Heuristic{
        let data: Array5<u64> = read_npy("HeuristicData/l_infinity_cds.npy").unwrap();
        Heuristic {
            data
        }
    }
    pub fn h(&self, state: &State, end: (u16, u16)) -> usize{
        let distance = (max(state.pos_x.abs_diff(end.0), state.pos_y.abs_diff(end.1)) - 1) as usize;
        self.data[[distance, state.secd as usize, state.scd as usize, state.ecd as usize, state.bdcd as usize]] as usize
    }
}
