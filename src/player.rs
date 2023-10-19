use std::time::Duration;
use crate::{COLS, ROWS};
use crate::frame::{drawable, Frame};
use crate::shot::Shot;

pub struct Player{
    x: usize,
    y: usize,
    shot: Vec<Shot>,
}

impl Player{
    pub fn new() -> Self{
        Self{
            x:COLS/2,
            y:ROWS - 1,
            shot: Vec::new(),
        }
    }

    pub fn move_left(&mut self){
        if self.x > 0 {self.x -= 1;}
    }
    pub fn move_right(&mut self) {
        if self.x < COLS - 1 { self.x += 1; }
    }
    pub fn shoot(&mut self) -> bool{
        if self.shot.len() < 2
        {
            self.shot.push(Shot::new(self.x, self.y - 1));
            return true;
        }
        else{
            return false;
        }
    }
    pub fn update(&mut self, delta: Duration)
    {
        for shot in self.shot.iter_mut(){
            shot.update(delta);
        }
        self.shot.retain(|shot| !shot.dead());
    }


}

impl drawable for Player{
    fn draw(&self, frame:&mut Frame){
        frame[self.x][self.y] = "A";
        for shot in self.shot.iter(){
            shot.draw(frame);
        }
    }
}

