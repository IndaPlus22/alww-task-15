use std::{
    io::{self, BufRead},
    process,
};

use rand::Rng;

fn main() {
    make_massive_dipshit_file();
    eprintln!("Beginning");
    let mut set: AUF = AUF::new();
    let input = io::stdin();
    let mut lines = input.lock().lines();

    let mut command_counter = 0;
    let mut first_line = true;
    while let Some(line) = lines.next() {
        let input = line.unwrap();
        if first_line {
            let mut input = input.trim().split_ascii_whitespace();
            let elements = input.next().unwrap().parse::<usize>().expect("parse error");
            let commands = input.next().unwrap().parse::<usize>().expect("parse error");

            first_line = false;
            set.update(elements, commands);
            continue;
        }
        // eprintln!("{}", input);

        if input.len() == 0 {
            break;
        }

        if command_counter == set.commands {
            panic!()
        }
        command_counter += 1;
        let parsed_input = string_to_vec(&input);
        match parsed_input[0] {
            1 => set.balanced_union(parsed_input[1] - 1, parsed_input[2] - 1),
            2 => set.balanced_move(parsed_input[1] - 1, parsed_input[2] - 1),
            3 => set.find(parsed_input[1] - 1),
            _ => panic!(),
        }
    }
    // let mut set = AUF::new();
    // set.update(6);
    // set.balanced_union(0, 1);
    // set.balanced_move(2, 3);
    // set.balanced_union(2, 4);
    // // println!("{:?}", set);
    // set.find(4 - 1);
    // set.balanced_move(3, 0);
    // println!("{:?}", set);
    // // set.balanced_move(1, 2);
    // // println!("{:?}", set);
    // set.find(4 - 1);
    // set.find(3 - 1);
    if command_counter != set.commands {
        panic!()
    }
    process::exit(0);
}
//AUF = Almost union find
#[warn(unused_parens)]
#[derive(Debug)]
pub struct AUF {
    collection: Vec<usize>,
    tree_size: Vec<usize>,
    commands: usize,
}
impl AUF {
    pub fn new() -> Self {
        let collection = Vec::default();
        let tree_size = Vec::default();
        Self {
            collection,
            tree_size,
            commands: 0,
        }
    }
    pub fn update(&mut self, size: usize, commands: usize) {
        let size = size + 1;
        self.collection = (1..size).collect();
        let mut tree_size = Vec::with_capacity(size);
        for _ in 1..size {
            tree_size.push(1)
        }
        self.tree_size = tree_size;
        self.commands = commands;
    }
    pub fn balanced_union(&mut self, a: usize, b: usize) {
        let root_of_a = self.root(a);
        let root_of_b = self.root(b);
        if root_of_a == root_of_b {
            return;
        }
        if self.tree_size[root_of_a] < self.tree_size[root_of_b] {
            self.collection[root_of_a] = self.collection[root_of_b];
            self.size(root_of_b, root_of_a)
        } else {
            self.collection[root_of_b] = self.collection[root_of_a];
            self.size(root_of_a, root_of_b)
        }
    }
    pub fn balanced_move(&mut self, a: usize, b: usize) {
        let root_of_a = self.root(a);
        let root_of_b = self.root(b);
        if self.root(a) != self.root(b) {
            self.collection[a] = root_of_b + 1;
            let mut first = 0;
            for i in 0..(self.collection.len()) {
                if a + 1 == self.collection[i] {
                    if first == 0 {
                        first = i + 1;
                    }
                    self.collection[i] = first;
                }
            }
            if self.tree_size[root_of_a] == 1 {
                self.tree_size[root_of_b] += 1;
            } else {
                if first == 0 {
                    first = 1
                }
                self.tree_size[first - 1] = self.tree_size[root_of_a] - 1;
                self.tree_size[root_of_a] = 1;
                self.tree_size[root_of_b] += 1;
            }
        }
    }
    pub fn root(&mut self, a: usize) -> usize {
        let tmp = a;
        let mut tmp_for_value = a + 1;
        while self.collection[tmp] != tmp_for_value {
            self.collection[tmp] = self.collection[self.collection[tmp] - 1];
            tmp_for_value = self.collection[tmp]
        }
        tmp_for_value - 1
    }
    pub fn size(&mut self, a: usize, b: usize) {
        self.tree_size[a] += self.tree_size[b];
        let root = self.root(a);
        if self.tree_size[b] > 1 {
            self.tree_size[b] = 1;
            for i in 0..(self.collection.len()) {
                if b + 1 == self.collection[i] {
                    self.collection[i] = root + 1;
                }
            }
        }
    }
    pub fn find(&mut self, a: usize) {
        let root_of_a = self.root(a);
        let mut sum = 0;
        for i in 0..(self.collection.len()) {
            if self.collection[i] == root_of_a + 1 {
                sum += i + 1
            }
        }
        println!("{} {}", self.tree_size[root_of_a], sum);
        println!("{:?}", self)
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

fn make_massive_dipshit_file() {
    let elem = 10;
    let com = 100;
    println!("{} {}", elem, com);
    let mut rng = rand::thread_rng();
    let mut d = 1;
    for _ in 0..com {
        if d % 5 == 0 {
            let a = 3;
            let b = rng.gen_range(1..=elem);
            println!("{} {}", a, b);
        } else {
            let a = rng.gen_range(1..=2);
            let b = rng.gen_range(1..=elem);
            let c = rng.gen_range(1..=elem);
            println!("{} {} {}", a, b, c);
        }
        d += 1;
    }
}
