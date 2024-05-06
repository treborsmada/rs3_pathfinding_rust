use std::cmp::max;
use crate::map_section::MapSection;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct State {
    pub pos_x: u16,
    pub pos_y: u16,
    pub direction: u8,
    pub secd: u8,
    pub scd: u8,
    pub ecd: u8,
    pub bdcd: u8,
    pub wait_time: u8,
}

impl State {
    pub fn update(&self) -> State {
        let pos_x = self.pos_x;
        let pos_y = self.pos_y;
        let direction = self.direction;
        let secd = max(self.secd, 1) - 1;
        let scd = max(self.scd, 1) - 1;
        let ecd= max(self.ecd, 1) - 1;
        let bdcd = max(self.bdcd, 1) - 1;
        let wait_time = max(self.wait_time, 1) - 1;
        State {
            pos_x,
            pos_y,
            direction,
            secd,
            scd,
            ecd,
            bdcd,
            wait_time,
        }
    }

    pub fn r#move(&self, x: u16, y: u16, direction: u8) -> State {
        State {
            pos_x: x,
            pos_y: y,
            direction: direction,
            secd: self.secd,
            scd: self.scd,
            ecd: self.ecd,
            bdcd: self.bdcd,
            wait_time: self.wait_time,
        }
    }

    pub fn surge(&self, section: &MapSection) -> State{
        let (new_x, new_y) = section.surge_range(self.pos_x, self.pos_y, self.direction);
        if self.secd == 0 {
            State {
                pos_x: new_x as u16,
                pos_y: new_y as u16,
                direction: self.direction,
                secd: 17,
                scd: max(2, self.scd),
                ecd: 17,
                bdcd: self.bdcd,
                wait_time: self.wait_time,
            }
        } else if self.scd == 0 {
            State {
                pos_x: new_x as u16,
                pos_y: new_y as u16,
                direction: self.direction,
                secd: max(2, self.secd),
                scd: 17,
                ecd: max(2, self.ecd),
                bdcd: self.bdcd,
                wait_time: self.wait_time,
            }
        } else {
            panic!()
        }
    }

    pub fn escape(&self, section: &MapSection) -> State {
        let (new_x, new_y) = section.escape_range(self.pos_x, self.pos_y, self.direction);
        if self.secd == 0 {
            State {
                pos_x: new_x as u16,
                pos_y: new_y as u16,
                direction: self.direction,
                secd: 17,
                scd: 17,
                ecd: max(2, self.ecd),
                bdcd: self.bdcd,
                wait_time: self.wait_time,
            }
        } else if self.ecd == 0 {
            State {
                pos_x: new_x as u16,
                pos_y: new_y as u16,
                direction: self.direction,
                secd: max(2, self.secd),
                scd: max(2, self.scd),
                ecd: 17,
                bdcd: self.bdcd,
                wait_time: self.wait_time,
            }
        } else {
            panic!()
        }
    }

    pub fn bd(&self, x:u16, y: u16, direction: u8) -> State{
        assert_eq!(self.bdcd, 0);
        State {
            pos_x: x,
            pos_y: y,
            direction,
            secd: self.secd,
            scd: self.scd,
            ecd: self.ecd,
            bdcd: 17,
            wait_time: self.wait_time
        }
    }

    pub fn can_bd(&self) -> bool{
        self.bdcd == 0
    }

    pub fn can_surge(&self) -> bool {
        self.secd == 0 || self.scd == 0
    }

    pub fn can_escape(&self) -> bool {
        self.secd == 0 || self.ecd == 0
    }

}