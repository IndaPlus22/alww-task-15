use std::{
    io::{self, BufRead},
    process,
};

// use rand::Rng;

fn main() {
    // // make_massive_dipshit_file();
    // eprintln!("Beginning");
    let mut set: AUF = AUF::new();
    let input = io::stdin();
    let mut lines = input.lock().lines();

    let mut command_counter = 0;
    let mut first_line = true;
    while let Some(line) = lines.next() {
        let input = match line {
            Ok(string) => string,
            Err(_) => {
                continue;
            }
        };
        if first_line {
            if input.is_empty() == true {
                continue;
            }
            let mut input = input.trim().split_ascii_whitespace();
            let elements = input.next().unwrap().parse::<usize>().expect("parse error");
            let commands = input.next().unwrap().parse::<usize>().expect("parse error");

            first_line = false;
            set.update(elements, commands);
            continue;
        }

        if command_counter < set.commands {
            if input.is_empty() == true {
                continue;
            }
        }
        if input.len() == 0 {
            break;
        }

        if command_counter == set.commands {
            break;
        }
        command_counter += 1;
        let parsed_input = string_to_vec(&input);
        match parsed_input[0] {
            1 => set.balanced_union(parsed_input[1] - 1, parsed_input[2] - 1),
            2 => set.move_from(parsed_input[1] - 1, parsed_input[2] - 1),
            3 => set.find(parsed_input[1] - 1),
            _ => panic!(),
        }
    }

    process::exit(0);
    // let mut set = AUF::new();
    // set.update(5, 20);
    // set.balanced_union(0, 1);
    // set.move_from(2, 3);
    // set.balanced_union(2, 4);
    // set.find(4 - 1);
    // set.move_from(3, 0);
    // set.find(4 - 1);
    // set.find(3 - 1);
}
//AUF = Almost union find
#[warn(unused_parens)]
#[derive(Debug)]
pub struct AUF {
    collection: Vec<usize>,
    eh: Vec<Vec<usize>>,
    commands: usize,
    size: usize,
}
impl AUF {
    pub fn new() -> Self {
        let collection = Vec::default();
        let eh = Vec::<Vec<usize>>::default();
        Self {
            collection,
            eh,
            commands: 0,
            size: 0,
        }
    }
    pub fn update(&mut self, size: usize, commands: usize) {
        self.size = size;
        self.collection = (0..size).collect();
        let mut eh = vec![Vec::with_capacity(size); size];
        for x in 0..size {
            eh[x].push(x + 1)
        }
        self.eh = eh;
        self.commands = commands;
    }
    pub fn balanced_union(&mut self, a: usize, b: usize) {
        let root_of_a = self.root(a);
        let root_of_b = self.root(b);
        if root_of_a == root_of_b {
            return;
        }
        if self.eh[root_of_a].len() < self.eh[root_of_b].len() {
            let tmp = self.eh[root_of_a].clone();
            self.eh[root_of_b].extend(tmp);
            self.eh[root_of_a] = Vec::with_capacity(self.size);
            self.collection[root_of_a] = root_of_b;
        } else {
            let tmp = self.eh[root_of_b].clone();
            self.eh[root_of_a].extend(tmp);
            self.eh[root_of_b] = Vec::with_capacity(self.size);
            self.collection[root_of_b] = root_of_a;
        }
    }
    pub fn move_from(&mut self, a: usize, b: usize) {
        if self.collection[a] == self.collection[b] {
            return;
        }
        let root_of_a = self.root(a);
        let root_of_b = self.root(b);
        // eprintln!("ra {}", root_of_a + 1);
        // eprintln!("rb {}", root_of_b + 1);
        // eprintln!("a {}", a + 1);
        // eprintln!("b {}", b + 1);
        if self.root(a) != self.root(b) {
            let index_of_a = self.eh[root_of_a]
                .iter()
                .position(|&x| x == (a + 1))
                .unwrap();
            self.eh[root_of_a].remove(index_of_a);
            self.eh[root_of_b].push(a + 1);
            self.collection[a] = root_of_b;
            if root_of_a == a {
                let mut index_of_first_match = 0;
                let op_index_of_first_match = self.collection.iter().position(|&x| x == a);
                match op_index_of_first_match {
                    Some(value) => {
                        let tmp = self.eh[a].clone();
                        self.eh[value] = tmp;
                        self.eh[a] = Vec::with_capacity(self.size);
                        index_of_first_match = value;
                        self.collection[value] = value;
                    }
                    None => {
                        return;
                    }
                }

                let mut check = true;
                while check {
                    let op_index_of_nth_match = self.collection.iter().position(|&x| x == (a));
                    match op_index_of_nth_match {
                        Some(value) => {
                            self.collection[value] = index_of_first_match;
                        }
                        None => check = false,
                    };
                }
            }
        }
    }
    pub fn root(&mut self, mut a: usize) -> usize {
        while self.collection[a] != a {
            a = self.collection[a]
        }
        a
    }
    pub fn find(&mut self, a: usize) {
        let root_of_a = self.root(a);
        let sum: usize = self.eh[root_of_a].iter().sum();
        println!("{} {}", self.eh[root_of_a].len(), sum);
        eprintln!("{:?}", self)
    }
}

pub fn string_to_vec(a: &String) -> Vec<usize> {
    //https://stackoverflow.com/questions/26536871/how-can-i-convert-a-string-of-numbers-to-an-array-or-vector-of-integers-in-rust
    let numbers: Vec<usize> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Could not parse input!"))
        .collect();
    numbers
}

// fn make_massive_dipshit_file() {
//     let elem = 100000;
//     let com = 100000;
//     println!("{} {}", elem, com);
//     let mut rng = rand::thread_rng();
//     let mut d = 1;
//     for _ in 0..com {
//         if d % 10000 == 0 {
//             let a = 3;
//             let b = rng.gen_range(1..=elem);
//             println!("{} {}", a, b);
//         } else {
//             let a = rng.gen_range(1..=2);
//             let b = rng.gen_range(1..=elem);
//             let c = rng.gen_range(1..=elem);
//             println!("{} {} {}", a, b, c);
//         }
//         d += 1;
//     }
// }
