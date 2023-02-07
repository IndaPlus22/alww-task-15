use std::{
    io::{self, BufRead},
    process,
    slice::SliceIndex,
    sync::{Arc, Mutex},
    thread::{self, Thread},
};

// use rand::seq::index;
// use gr::make_massive_dipshit_file;
// mod gr;

fn main() {
    // make_massive_dipshit_file();
    eprintln!("Beginning");
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
            2 => set.balanced_move(parsed_input[1] - 1, parsed_input[2] - 1),
            3 => set.find(parsed_input[1] - 1),
            _ => panic!(),
        }
    }
    // let mut set = AUF::new();
    // set.update(5, 20);
    // set.balanced_union(0, 1);
    // set.balanced_move(2, 3);
    // set.balanced_union(2, 4);
    // // println!("{:?}", set);
    // set.find(4 - 1);
    // set.balanced_move(3, 0);
    // // println!("{:?}", set);
    // // set.balanced_move(1, 2);
    // // println!("{:?}", set);
    // set.find(4 - 1);
    // set.find(3 - 1);
    // if command_counter != set.commands {
    //     panic!()
    // }
    process::exit(0);
}
//AUF = Almost union find
#[warn(unused_parens)]
#[derive(Debug)]
pub struct AUF {
    collection: Vec<usize>,
    tree_size: Vec<usize>,
    n: usize,
    commands: usize,
    sum_vec: Vec<usize>,
}
impl AUF {
    fn new() -> Self {
        let collection = Vec::default();
        let tree_size = Vec::default();
        Self {
            collection,
            tree_size,
            n: 0,
            commands: 0,
            sum_vec: Vec::default(),
        }
    }
    fn update(&mut self, size: usize, commands: usize) {
        self.n = size;
        let size = size + 1;
        self.collection = (1..size).collect();
        self.sum_vec = (1..size).collect();
        let mut tree_size = Vec::with_capacity(size);
        for _ in 1..size {
            tree_size.push(1)
        }
        self.tree_size = tree_size;
        self.commands = commands;
    }
    fn balanced_union(&mut self, a: usize, b: usize) {
        let root_of_a = self.root(a);
        let root_of_b = self.root(b);
        if root_of_a == root_of_b {
            return;
        }
        let mut do_correction = false;
        if self.tree_size[root_of_a] < self.tree_size[root_of_b] {
            self.collection[root_of_a] = self.collection[root_of_b];
            self.tree_size[root_of_b] += self.tree_size[root_of_a];
            self.sum_vec[root_of_b] += self.sum_vec[root_of_a];
            // if self.tree_size[root_of_a] != 0 {
            //     // do_correction = true
            // }
            self.tree_size[root_of_a] = 1;
            self.sum_vec[root_of_a] = 0;
            // if do_correction {
            //     let first_child: Arc<Mutex<usize>> =
            //         Arc::new(Mutex::new(self.collection[self.collection[a] - 1] - 1));
            //     self.correction(self.collection[a] - 1, root_of_a, first_child, false)
            // }
        } else {
            self.collection[root_of_b] = self.collection[root_of_a];
            self.tree_size[root_of_a] += self.tree_size[root_of_b];
            self.sum_vec[root_of_a] += self.sum_vec[root_of_b];
            // if self.tree_size[root_of_b] != 0 {
            //     // do_correction = true
            // }
            self.tree_size[root_of_b] = 1;
            self.sum_vec[root_of_b] = 0;
            // if do_correction {
            //     let first_child: Arc<Mutex<usize>> =
            //         Arc::new(Mutex::new(self.collection[self.collection[b] - 1] - 1));
            //     self.correction(self.collection[b] - 1, root_of_a, first_child, false)
            // }
        }
    }
    fn balanced_move(&mut self, a: usize, b: usize) {
        if self.collection[a] == self.collection[b] {
            return;
        }
        let root_of_a = self.root(a);
        let root_of_b = self.root(b);
        // eprintln!("ra {}", root_of_a);
        // eprintln!("rb {}", root_of_b);
        // eprintln!("a {}", a);
        // eprintln!("b {}", b);
        if self.root(a) == self.root(b) {
            return;
        }
        if self.collection[a] != a + 1 {
            // eprintln!("------1--------");
            self.tree_size[root_of_b] += 1;
            self.sum_vec[root_of_b] += a + 1;
            self.sum_vec[root_of_a] -= a + 1;
            if self.tree_size[root_of_a] > 1 {
                self.tree_size[root_of_a] -= 1;
            }
            self.collection[a] = root_of_b + 1;
        } else {
            // eprintln!("------2--------");
            self.sum_vec[root_of_b] += self.collection[root_of_a];
            self.sum_vec[root_of_a] -= self.collection[root_of_a];
            self.collection[root_of_a] = root_of_b + 1;
            self.tree_size[root_of_a] = 1;
            self.tree_size[root_of_b] += 1;
            let first_child: Arc<Mutex<usize>> = Arc::new(Mutex::new(100001));
            self.correction(a, root_of_a, first_child, true);
        }
    }
    fn correction(
        &mut self,
        a: usize,
        root_of_a: usize,
        first_child: Arc<Mutex<usize>>,
        change_tree: bool,
    ) {
        let mut thread_count = self.n;
        if thread_count > 4 {
            thread_count = 4;
        }
        let mut chunks = self.collection.chunks(self.n / thread_count);
        // eprintln!("{}", chunks.len());
        let indexes: Vec<usize> = (0..self.n).collect();
        let mut indexes = indexes.chunks(self.n / thread_count);
        let tree_size = Arc::new(Mutex::new(self.tree_size.clone()));
        let sum_vec = Arc::new(Mutex::new(self.sum_vec.clone()));
        let mut computations = Vec::new();
        for _ in 0..chunks.len() {
            let mut chunk = chunks.next().unwrap().to_owned();
            let indexes = indexes.next().unwrap().to_owned();
            let first_child = Arc::clone(&first_child);
            let tree_size = Arc::clone(&tree_size);
            let sum_vec = Arc::clone(&sum_vec);
            // eprintln!("soon spawned a thread");
            let computation = thread::spawn(move || {
                // eprintln!("spawned a thread");
                for x in 0..chunk.len() {
                    if chunk[x] == a + 1 {
                        let mut first_child = first_child.lock().unwrap();
                        if *first_child == 100001 {
                            let mut sum = sum_vec.lock().unwrap();
                            sum[indexes[x]] = sum[root_of_a];
                            sum[root_of_a] = 0;
                            *first_child = indexes[x];
                            chunk[x] = *first_child + 1;
                            continue;
                        }
                        let mut tree_size = tree_size.lock().unwrap();
                        tree_size[*first_child] += 1;
                        chunk[x] = *first_child + 1;
                    }
                }
                chunk
            });
            computations.push(computation);
        }
        let mut result: Vec<usize> = Vec::new();
        for computation in computations {
            let value = computation.join().unwrap();
            result.extend(value);
        }
        self.collection = result;
        if change_tree {
            self.sum_vec = sum_vec.lock().unwrap().clone();
            self.tree_size = tree_size.lock().unwrap().clone();
        }
    }
    fn root(&mut self, a: usize) -> usize {
        let tmp = a;
        let mut tmp_for_value = a + 1;
        while self.collection[tmp] != tmp_for_value {
            self.collection[tmp] = self.collection[self.collection[tmp] - 1];
            tmp_for_value = self.collection[tmp]
        }
        tmp_for_value - 1
    }
    fn find(&mut self, a: usize) {
        let root_of_a = self.root(a);
        // let a = self.collection[a] - 1;

        // let mut thread_count = self.n;
        // if thread_count > 4 {
        //     thread_count = 4;
        // }
        // let indexes: Vec<usize> = (0..self.n).collect();
        // let mut indexes = indexes.chunks(self.n / thread_count);
        // let mut chunks = self.collection.chunks(self.n / thread_count);
        // // eprintln!("{}", chunks.len());
        // let mut computations = Vec::new();
        // for _ in 0..chunks.len() {
        //     let chunk = chunks.next().unwrap().to_owned();
        //     let indexes = indexes.next().unwrap().to_owned();
        //     // eprintln!("soon spawned a thread");
        //     let computation = thread::spawn(move || {
        //         // eprintln!("spawned a thread");
        //         let mut value = 0;
        //         for x in 0..chunk.len() {
        //             if chunk[x] == a + 1 {
        //                 value += indexes[x] + 1;
        //             }
        //         }
        //         value
        //     });
        //     computations.push(computation)
        // }

        // let mut result: usize = 0;
        // for computation in computations {
        //     let value = computation.join().unwrap();
        //     result += value;
        // }
        println!("{} {}", self.tree_size[root_of_a], self.sum_vec[root_of_a]);
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
