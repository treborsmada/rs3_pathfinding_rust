use ndarray::{Array3, Axis, concatenate};
use ndarray_npy::read_npy;
use std::cmp;

#[derive(Debug)]
pub struct MapSection {
    floor: usize,
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
    bd_data: Array3<u64>,
    se_data: Array3<u8>,
    walk_data: Array3<u64>,
}

impl MapSection {
    pub fn bd_range(&self, x: u16, y: u16) -> Vec<(u16, u16, u8)> {
        let (x, y) = (x as usize, y as usize);
        let mut tiles = Vec::new();
        for i in 0..7 {
            let bd_data = self.bd_data[[x - self.x_start, y - self.y_start, i]];
            for j in 0..64 {
                if (bd_data >> j) & 1 == 1 {
                    let u = x - 10 + (j+64*i) % 21;
                    let v = y - 10 + (j+64*i) / 21;
                    let x_diff = (u as isize) - (x as isize);
                    let y_diff = (v as isize) - (y as isize);
                    let mut direction: u8 = 0;
                    if x_diff == 0 {
                        if y_diff > 0 {
                            direction = 0;
                        } else {
                            direction = 4;
                        }
                    } else if y_diff == 0 {
                        if x_diff > 0 {
                            direction = 2;
                        } else {
                            direction = 6;
                        }
                    } else if (14 * x_diff.abs() + 7) / (2 * y_diff.abs() + 1) > 15 {
                        if x_diff > 0 {
                            direction = 2;
                        } else {
                            direction = 6;
                        }
                    } else if (14 * y_diff.abs() + 7) / (2 * x_diff.abs() + 1) > 15 {
                        if y_diff > 0 {
                            direction = 0;
                        } else {
                            direction = 4;
                        }
                    } else if x_diff > 0 {
                        if y_diff > 0 {
                            direction = 1;
                        } else {
                            direction = 3;
                        }
                    } else if x_diff < 0 {
                        if y_diff > 0 {
                            direction = 7;
                        } else {
                            direction = 5;
                        }
                    }
                    tiles.push((u as u16, v as u16, direction));
                }
            }
        }
        tiles
    }

    pub fn surge_range(&self, x: u16, y: u16, direction: u8) -> (usize, usize){
        let (x, y, direction) = (x as usize, y as usize, direction as usize);
        let offset = (self.se_data[[x-self.x_start, y - self.y_start, direction]] & 15) as usize;
        match direction {
            0 => (x, y + offset),
            1 => (x + offset, y + offset),
            2 => (x + offset, y),
            3 => (x + offset, y - offset),
            4 => (x, y - offset),
            5 => (x - offset, y - offset),
            6 => (x - offset, y),
            7 => (x - offset, y + offset),
            _ => panic!()
        }
    }

    pub fn escape_range(&self, x: u16, y: u16, direction: u8) -> (usize, usize) {
        let (x, y, direction) = (x as usize, y as usize, direction as usize);
        let offset = (self.se_data[[x-self.x_start, y - self.y_start, direction]] >> 4) as usize;
        match direction {
            0 => (x, y - offset),
            1 => (x - offset, y - offset),
            2 => (x - offset, y),
            3 => (x - offset, y + offset),
            4 => (x, y + offset),
            5 => (x + offset, y + offset),
            6 => (x + offset, y),
            7 => (x + offset, y - offset),
            _ => panic!()
        }
    }

    pub fn walk_range(&self, x: u16, y: u16) -> Vec<(u16, u16, u8)> {
        let (x, y) = (x as usize, y as usize);
        let mut tiles = Vec::new();
        for i in 0..2 {
            let walk_data = self.walk_data[[x-self.x_start, y - self.y_start, i]];
            for j in 0..16 {
                let direction = (walk_data >> (j * 4)) & 15;
                if direction < 8 {
                    let u = x - 2 + (j + 16 * i) % 5;
                    let v = y - 2 + (j + 16 * i) / 5;
                    tiles.push((u as u16, v as u16, direction as u8))
                }
            }
        }
        tiles
    }

    pub fn create_map_section(x_start: usize, x_end: usize, y_start: usize, y_end: usize, floor: usize) -> MapSection {
        let bd_data = build_bd_array(x_start, x_end, y_start, y_end, floor);
        let se_data = build_se_array(x_start, x_end, y_start, y_end, floor);
        let walk_data = build_walk_array(x_start, x_end, y_start, y_end, floor);
        MapSection {
            floor,
            x_start,
            x_end,
            y_start,
            y_end,
            bd_data,
            se_data,
            walk_data,
        }
    }
}

fn build_bd_array(x_start: usize, x_end: usize, y_start: usize, y_end: usize, floor: usize) -> Array3<u64> {
    let chunk_size =  640;
    let chunk_x = (x_start/chunk_size, x_end/chunk_size);
    let chunk_y = (y_start/chunk_size, y_end/chunk_size);
    let mut rows = Vec::new();
    for j in chunk_y.0..=chunk_y.1 {
        let mut row  = Vec::new();
        for i in chunk_x.0..=chunk_x.1 {
            let path = format!("MapData/BD/bd-{i}-{j}-{floor}.npy");
            let arr: Array3<u64> = read_npy(path).unwrap();
            let x_1 = cmp::max(x_start % chunk_size,(i - chunk_x.0) * chunk_size) - (i - chunk_x.0) * chunk_size;
            let x_2 = cmp::min(x_end - x_start + (x_start % chunk_size) + 1, chunk_size);
            let y_1 = cmp::max(y_start % chunk_size, (j - chunk_y.0) * chunk_size) - (j - chunk_y.0) * chunk_size;
            let y_2 = cmp::min(y_end - y_start + (y_start % chunk_size) - (j - chunk_y.0) * chunk_size + 1, chunk_size);
            let arr = arr.slice(ndarray::s![x_1..x_2, y_1..y_2, ..]).to_owned();
            row.push(arr);
        }
        let views: Vec<_> = row.iter().map(|arr| arr.view()).collect();
        rows.push(concatenate(Axis(0), &views[..]).unwrap());
    }
    let views: Vec<_> = rows.iter().map(|arr| arr.view()).collect();
    concatenate(Axis(1), &views[..]).unwrap()
}

fn build_se_array(x_start: usize, x_end: usize, y_start: usize, y_end: usize, floor: usize) -> Array3<u8> {
    let chunk_size =  640;
    let chunk_x = (x_start/chunk_size, x_end/chunk_size);
    let chunk_y = (y_start/chunk_size, y_end/chunk_size);
    let mut rows = Vec::new();
    for j in chunk_y.0..=chunk_y.1 {
        let mut row  = Vec::new();
        for i in chunk_x.0..=chunk_x.1 {
            let path = format!("MapData/SE/se-{i}-{j}-{floor}.npy");
            let arr: Array3<u8> = read_npy(path).unwrap();
            let x_1 = cmp::max(x_start % chunk_size,(i - chunk_x.0) * chunk_size) - (i - chunk_x.0) * chunk_size;
            let x_2 = cmp::min(x_end - x_start + (x_start % chunk_size) + 1, chunk_size);
            let y_1 = cmp::max(y_start % chunk_size, (j - chunk_y.0) * chunk_size) - (j - chunk_y.0) * chunk_size;
            let y_2 = cmp::min(y_end - y_start + (y_start % chunk_size) - (j - chunk_y.0) * chunk_size + 1, chunk_size);
            let arr = arr.slice(ndarray::s![x_1..x_2, y_1..y_2, ..]).to_owned();
            row.push(arr);
        }
        let views: Vec<_> = row.iter().map(|arr| arr.view()).collect();
        rows.push(concatenate(Axis(0), &views[..]).unwrap());
    }
    let views: Vec<_> = rows.iter().map(|arr| arr.view()).collect();
    concatenate(Axis(1), &views[..]).unwrap()
}

fn build_walk_array(x_start: usize, x_end: usize, y_start: usize, y_end: usize, floor: usize) -> Array3<u64> {
    let chunk_size =  640;
    let chunk_x = (x_start/chunk_size, x_end/chunk_size);
    let chunk_y = (y_start/chunk_size, y_end/chunk_size);
    let mut rows = Vec::new();
    for j in chunk_y.0..=chunk_y.1 {
        let mut row  = Vec::new();
        for i in chunk_x.0..=chunk_x.1 {
            let path = format!("MapData/Walk/walk-{i}-{j}-{floor}.npy");
            let arr: Array3<u64> = read_npy(path).unwrap();
            let x_1 = cmp::max(x_start % chunk_size,(i - chunk_x.0) * chunk_size) - (i - chunk_x.0) * chunk_size;
            let x_2 = cmp::min(x_end - x_start + (x_start % chunk_size) + 1, chunk_size);
            let y_1 = cmp::max(y_start % chunk_size, (j - chunk_y.0) * chunk_size) - (j - chunk_y.0) * chunk_size;
            let y_2 = cmp::min(y_end - y_start + (y_start % chunk_size) - (j - chunk_y.0) * chunk_size + 1, chunk_size);
            let arr = arr.slice(ndarray::s![x_1..x_2, y_1..y_2, ..]).to_owned();
            row.push(arr);
        }
        let views: Vec<_> = row.iter().map(|arr| arr.view()).collect();
        rows.push(concatenate(Axis(0), &views[..]).unwrap());
    }
    let views: Vec<_> = rows.iter().map(|arr| arr.view()).collect();
    concatenate(Axis(1), &views[..]).unwrap()
}