use std::cmp::{max, min};
use std::collections::{HashSet, HashMap, VecDeque};
use ndarray::{Array2, Array3, Array5};
use ndarray_npy::{read_npy, write_npy};
use crate::{util::{adj_positions, free_direction}};

const RS_HEIGHT: usize = 12800;
const RS_LENGTH: usize = 6400;

struct Process {
    movement_data: HashMap<(usize, usize, usize), Array2<u8>>
}

impl Process {
    fn new() -> Process {
        Process {
            movement_data: HashMap::new()
        }
    }

    fn walk_range(&mut self, x: usize, y: usize, floor: usize) -> Vec<(usize, usize, usize)> {
        let mut tiles = Vec::new();
        let start = self.get_movement_data(x, y, floor);
        let adj = adj_positions(x, y);
        let mut visited = HashSet::new();
        visited.insert((x, y));
        let mut queue = VecDeque::new();
        for i in 0..8 {
            let j = (2*i + i/4) % 8;
            if free_direction(start, j) {
                tiles.push((adj[j].0, adj[j].1, j));
                visited.insert(adj[j]);
                queue.push_back(adj[j]);
            }
        }
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let current_move_data = self.get_movement_data(current.0, current.1, floor);
            let temp_adj = adj_positions(current.0, current.1);
            for i in 0..8 {
                if free_direction(current_move_data, i) {
                    if !visited.contains(&temp_adj[i]) {
                        tiles.push((temp_adj[i].0, temp_adj[i].1, i));
                        visited.insert(temp_adj[i]);
                    }
                }
            }
        }
        tiles
    }

    fn get_movement_data(&mut self, x: usize, y: usize, floor: usize) -> u8 {
        if x < RS_LENGTH && y < RS_HEIGHT {
            let chunk_size = 1280;
            let (chunk_x, chunk_y) = (x / chunk_size, y / chunk_size);
            if let Some(arr) = self.movement_data.get(&(chunk_x, chunk_y, floor)) {
                arr[[x % chunk_size, y % chunk_size]]
            } else {
                let path = format!("MapData/Map/move-{chunk_x}-{chunk_y}-{floor}.npy");
                println!("{path}");
                let data: Array2<u8> = read_npy(path).unwrap();
                let result = data[[x % chunk_size, y % chunk_size]];
                self.movement_data.insert((chunk_x, chunk_y, floor), data);
                result
            }
        } else {
            0
        }
    }

    fn process_walk_data(&mut self, x: usize, y: usize, floor: usize) -> (u64, u64) {
        let tiles = self.walk_range(x, y, floor);
        let mut walk_data = u128::MAX;
        for tile in tiles {
            let u = x - 2;
            let v = y - 2;
            if tile.0 < RS_LENGTH && tile.1 < RS_HEIGHT {
                let temp = (15 - tile.2 as u128) << (4*(tile.0 - u + (tile.1 - v)*5));
                walk_data = walk_data - temp;
            }
        }
        (walk_data as u64, (walk_data >> 64) as u64)
    }
}

fn build_walk_array(chunk_x: usize, chunk_y: usize, floor: usize) -> Array3<u64> {
    let mut process = Process::new();
    let mut walk_array = Array3::zeros([640, 640, 2]);
    let start_x = chunk_x * 640;
    let start_y = chunk_y * 640;
    for i in 0..640 {
        for j in 0..640 {
            let walk_data = process.process_walk_data(start_x + j, start_y + i, floor);
            walk_array[[j, i, 0]] = walk_data.0;
            walk_array[[j, i, 1]] = walk_data.1;
        }
    }
    walk_array
}

pub fn process_walk_data() {
    for i in 0..4 {
        for j in 0..10 {
            for k in 0..20 {
                let arr = build_walk_array(j, k ,i);
                let path = format!("MapData/Walk/walk-{j}-{k}-{i}.npy");
                write_npy(path, &arr).unwrap();
            }
        }
    }
}

pub fn process_heuristic_data(max_distance: usize) {
    let mut arr : Array5<u64> = Array5::zeros([max_distance+1, 18, 18, 18, 18]);
    let mut memo = Memo::new();
    for distance in 0..=max_distance {
        for secd in 0..=17 {
            for scd in 0..=17 {
                for ecd in 0..=17 {
                    for bdcd in 0..=17 {
                        arr[[distance, secd, scd, ecd, bdcd]] = memo.distance_cds_rec(distance as isize, secd, scd, ecd, bdcd) as u64;
                    }
                }
            }
        }
    }
    println!("{:?}",arr);
    write_npy("HeuristicData/l_infinity_cds.npy", &arr).unwrap();
}

struct Memo {
    data: HashMap<(isize, usize, usize, usize, usize), usize>
}

impl Memo {
    fn new() -> Memo {
        Memo {
            data: HashMap::new()
        }
    }

    fn distance_cds_rec(&mut self, distance: isize, secd: usize, scd: usize, ecd: usize, bdcd: usize) -> usize {
        if self.data.contains_key(&(distance, secd, scd, ecd, bdcd)) {
            return *self.data.get(&(distance, secd, scd, ecd, bdcd)).unwrap()
        }
        if distance <= 0 {
            return 0;
        }
        let mut bd = usize::MAX;
        let mut surge = usize::MAX;
        let mut escape = usize::MAX;
        let mut walk = usize::MAX;
        if bdcd == 0 {
            bd = self.distance_cds_rec(distance - 10, secd, scd, ecd, 17);
        }
        if secd == 0 {
            surge = self.distance_cds_rec(distance - 10, 17, max(2, scd), 17, bdcd);
        } else if scd == 0 {
            surge = self.distance_cds_rec(distance - 10, max(2, secd), 17, max(2, ecd), bdcd);
        }
        if secd == 0 {
            escape = self.distance_cds_rec(distance - 7, 17, 17, max(2, ecd), bdcd);
        } else if ecd == 0 {
            escape = self.distance_cds_rec(distance - 7, max(2, secd), max(2, scd), 17, bdcd);
        }
        if secd != 0 && bdcd != 0 {
            walk = self.distance_cds_rec(distance - 2, max(secd, 1) - 1, max(scd, 1) - 1, max(ecd, 1) - 1, max(bdcd, 1) - 1) + 1;
        }
        let result = min(min(min(bd, surge), escape), walk);
        self.data.insert((distance, secd, scd, ecd, bdcd), result);
        result
    }
}




