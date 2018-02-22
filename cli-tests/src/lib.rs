extern crate failure;

pub fn test<F>(name: String, callback: F)
    where F : Fn(Test) {
    let t = Test::new(name);
    callback(t);
}

pub struct Test {
    pub name: String,
}

impl Test {
    pub fn new(name: String) -> Test {
        Test{
            name: name,
        }
    }

    pub fn assert(&self, value: bool) {
        if value != true {
            println!("failed {:?}", value);
        }
    }
}
