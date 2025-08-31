use std::fmt::Debug;

#[derive(Debug)]
pub struct SeqList<T> {
    data: Vec<T>,
}

impl<T> SeqList<T>
where
    T: Clone + PartialEq + Debug,
{
    pub fn new() -> Self {
        SeqList { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push_back(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn push_front(&mut self, value: T) {
        self.data.insert(0, value);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    pub fn insert(&mut self, pos: usize, value: T) -> bool {
        if pos > self.len() {
            false
        } else {
            self.data.insert(pos, value);
            true
        }
    }

    pub fn remove(&mut self, pos: usize) -> Option<T> {
        if pos >= self.len() {
            None
        } else {
            Some(self.data.remove(pos))
        }
    }

    pub fn find(&self, value: &T) -> Option<usize> {
        self.data.iter().position(|x| x == value)
    }

    pub fn modify(&mut self, pos: usize, value: T) -> bool {
        if pos >= self.len() {
            false
        } else {
            self.data[pos] = value;
            true
        }
    }

    pub fn print(&self) {
        for item in &self.data {
            print!("{:?} ", item);
        }
        println!();
    }
}
