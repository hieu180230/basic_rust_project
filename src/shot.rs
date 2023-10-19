use std::time::Duration;
use rusty_time::prelude::Timer;
use crate::frame::{drawable, Frame};

pub struct Shot{
    pub x : usize,
    pub y: usize,
    pub explode: bool,
    pub timer: Timer,
}

impl Shot{
    pub fn new(x:usize, y:usize) -> Self {
        Self{x, y, explode: false, timer: Timer::from_millis(50)}
    }
    pub fn update(&mut self, delta: Duration){
        self.timer.update(delta);
        if self.timer.ready && !self.explode{
            if self.y > 0 {self.y -= 1;}
            self.timer.reset();
        }
    }
    pub fn explosion(&mut self){
        self.explode = true;
        self.timer = Timer::from_millis(250);
    }
    pub fn dead(&self) -> bool{
        return (self.explode && self.timer.ready) || (self.y == 0)
    }
}

impl drawable for Shot{
    fn draw(&self, frame: &mut Frame){
        frame[self.x][self.y] = if self.explode {"*"} else {"|"};
    }
}