use std::io::{stdin, Lines, StdinLock};

pub struct InputGetter<'a> {
    lines: Lines<StdinLock<'a>>
}

impl InputGetter<'_> {
    pub fn new() -> Self {
        Self {
            lines: stdin().lines()
        }
    }

    pub fn get_input(&mut self) -> String {
        self.lines.next().unwrap().unwrap()
    }
}