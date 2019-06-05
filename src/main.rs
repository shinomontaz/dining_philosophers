use std::{thread, time};
use std::sync::{Mutex, Arc};

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
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep( time::Duration::from_secs(3) );

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {

    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ] });

    let phils = vec![
        Philosopher::new("One", 0, 1),
        Philosopher::new("Two", 1, 2),
        Philosopher::new("Three", 2, 3),
        Philosopher::new("Four", 3, 4),
        Philosopher::new("Five", 4, 0),
    ];

    let handles: Vec<_> = phils.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn( move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}