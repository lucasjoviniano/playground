use std::{
    sync::{Arc, Mutex},
    thread, time,
};

struct Table {
    forks: Vec<Mutex<()>>,
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
            left,
            right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(time::Duration::from_millis(1000));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating!", self.name);

        thread::sleep(time::Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philos = vec![
        Philosopher::new("Immanuel Kant", 0, 1),
        Philosopher::new("Karl Marx", 1, 2),
        Philosopher::new("Simone de Beauvoir", 2, 3),
        Philosopher::new("René Descartes", 3, 4),
        Philosopher::new("Hannah Arendt", 0, 4),
    ];

    let handles: Vec<_> = philos
        .into_iter()
        .map(|philo| {
            let table = table.clone();

            thread::spawn(move || {
                philo.eat(&table);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
