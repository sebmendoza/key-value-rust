pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}



pub struct KvStore {
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {};
        panic!()
    }

    pub fn set(&mut self, key: String, value: String) {
        panic!()
    }

    pub fn get(&self, key: String) -> Option<String> {
        panic!()
    }

    pub fn remove(&mut self, key: String) {
        panic!()
    }
}