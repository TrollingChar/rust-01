#[derive(Debug)]
pub struct State {
    time: i64
}


impl State {
    pub fn new() -> Self {
        State { time: 0 }
    }
    pub fn update(&mut self) {
        self.time += 1;
    }
}