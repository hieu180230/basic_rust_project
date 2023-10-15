use crate::{COLS, ROWS};
use crate::frame::{drawable, Frame};

pub struct Player{
    x: usize,
    y: usize,
}

impl Player{
    pub fn new() -> Self{
        Self{
            x:COLS/2,
            y:ROWS - 1,
        }
    }

    pub fn move_left(&mut self){
        if self.x > 0 {self.x -= 1;}
    }
    pub fn move_right(&mut self) {
        if self.x < COLS - 1 { self.x += 1; }
    }
}

impl drawable for Player{
    fn draw(&self, frame:&mut Frame){
        frame[self.x][self.y] = "A";
    }
}