use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub start: i32,
    pub end: i32
}

impl Default for Range {
    fn default() -> Self {
        Range{start:1, end: -1}
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "<>")
        } else {
            write!(f, "<{}, {}>", self.start, self.end)
        }
        
    }

}

impl Range {
    pub fn add(&mut self, value:i32) {
        if value < self.start {
            self.start = value
        }
        if value > self.end {
            self.end = value
        }
    }

    fn is_empty(&self) -> bool {
        self.end < self.start
    }

    pub fn iter(&self) -> impl Iterator<Item=i32> {
        self.start-1..=self.end+1
    }

    pub fn rev_iter(&self) -> impl Iterator<Item=i32> {
        self.end+1..=self.start-1
    }
}
