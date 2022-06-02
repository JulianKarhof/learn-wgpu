pub struct Instances<T> {
    list: Vec<T>,
}

impl<T> Instances<T> {
    pub fn new() -> Self {
        Self {
            list: Vec::with_capacity(100),
        }
    }

    pub fn add(&mut self, instance: T) {
        self.list.push(instance);
    }

    pub fn remove(&mut self, index: u32) {
        self.list.remove(index as usize);
    }

    pub fn length(&self) -> u32 {
        return self.list.len() as u32;
    }

    // pub fn get(&self) -> Vec<T> {
    //     return self.list;
    // }
}
