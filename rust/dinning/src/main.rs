use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} now started eating!", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} now ended eating!", self.name);
    }
}

fn main() {
    let p = vec![
        Philosopher::new("fumiphys"),
        Philosopher::new("fumifumi"),
    ];
    for ph in &p{
        ph.eat();
    }
}
