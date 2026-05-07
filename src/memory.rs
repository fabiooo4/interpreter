use std::collections::HashMap;

pub trait Memory<T: Clone> {
    fn add(&mut self, id: String, val: T);
    fn get(&self, id: &str) -> Option<&T>;
    fn update(&mut self, id: String, val: T) -> Option<T> {
        let old_val = self.get(&id)?.clone();
        self.add(id, val);
        Some(old_val)
    }
}

#[derive(Default, Debug)]
pub struct HashMemory<T> {
    content: HashMap<String, T>,
}

impl<T: Clone> Memory<T> for HashMemory<T> {
    fn add(&mut self, id: String, val: T) {
        self.content.insert(id, val);
    }

    fn get(&self, id: &str) -> Option<&T> {
        self.content.get(id)
    }
}
