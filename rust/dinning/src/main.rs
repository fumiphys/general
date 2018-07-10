use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()> >,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} now started eating!", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} now ended eating!", self.name);
    }
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]
    });
    let p = vec![
        Philosopher::new("fumiphys", 0, 1),
        //Philosopher::new("fumifumi", 1, 2),
        Philosopher::new("physfumi", 2, 3),
        Philosopher::new("physphys", 0, 3),
    ];
    let handles: Vec<_> = p.into_iter().map(|ph| {
        let table = table.clone();
        thread::spawn(move || {
            ph.eat(&table);
        })
    }).collect();
    for h in handles {
        h.join().unwrap();
    }
}
