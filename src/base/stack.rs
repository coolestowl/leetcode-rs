pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, elem: T) {
        self.data.push(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = Stack::new();
        s.push(233);

        let s = String::from("_");

        if s.contains("asd") {
            println!("{}", s);
        }

        if s.contains("paasdt") {
            println!("{}", s);
        }

        let seps = s.split("asd").collect::<Vec<&str>>();
        for i in seps {
            println!("{}", i);
        }
    }
}
