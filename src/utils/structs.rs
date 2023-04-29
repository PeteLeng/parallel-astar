#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Grid {
    pub data: Vec<Option<i32>>,
    pub size: i32,
    pub empty_idx: i32,
}

// The state trait makes the framework generic.
// For later.
// pub trait State {
//     fn expand(&self) -> Vec<Self>;
// }

impl Grid {
    pub fn new(nums: Vec<Option<i32>>, size: i32, empty_idx: i32) -> Self {
        Grid {
            data: nums,
            size,
            empty_idx,
        }
    }

    pub fn rand(size: i32) -> Self {
        let mut rng = rand::thread_rng();
        let empty_idx = rng.gen_range(0..size.pow(2));
        let mut nums: Vec<i32> = (0..size.pow(2) - 1).collect();
        let mut data = vec![];
        for i in 0..size.pow(2) {
            if i == empty_idx {
                data.push(None);
            } else {
                let num = nums.remove(rng.gen_range(0..nums.len()));
                data.push(Some(num))
            }
        }
        Grid {
            data,
            size,
            empty_idx,
        }
    }

    pub fn rand_with_seed(size: i32, seed: u64) -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let empty_idx = rng.gen_range(0..size.pow(2));
        let mut nums: Vec<i32> = (0..size.pow(2) - 1).collect();
        let mut data = vec![];
        for i in 0..size.pow(2) {
            if i == empty_idx {
                data.push(None);
            } else {
                let num = nums.remove(rng.gen_range(0..nums.len()));
                data.push(Some(num))
            }
        }
        Grid {
            data,
            size,
            empty_idx,
        }
    }

    pub fn get_coord(&self, idx: i32) -> (i32, i32) {
        assert!(idx < self.size.pow(2));
        let x = idx / self.size;
        let y = idx % self.size;
        (x, y)
    }

    pub fn valid_action(&self, action: (i32, i32)) -> bool {
        let (mut x, mut y) = self.get_coord(self.empty_idx);
        x += action.0;
        y += action.1;
        if x >= 0 && x < self.size && y >= 0 && y < self.size {
            return true;
        }
        false
    }

    pub fn do_action(&self, action: (i32, i32)) -> Self {
        // Given a valid action, return a new grid
        let mut data = self.data.clone();
        let (mut x, mut y) = self.get_coord(self.empty_idx);
        x += action.0;
        y += action.1;
        let empty_idx = x * self.size + y;
        assert!(empty_idx >= 0 && empty_idx < self.size.pow(2));
        data[self.empty_idx as usize] = data[empty_idx as usize];
        data[empty_idx as usize] = None;
        Grid {
            data,
            size: self.size,
            empty_idx,
        }
    }

    pub fn try_action(&self, action: (i32, i32)) -> Option<Self> {
        if self.valid_action(action) {
            return Some(self.do_action(action));
        } else {
            None
        }
    }

    pub fn do_actions(&self, actions: Vec<(i32, i32)>) -> Self {
        actions
            .iter()
            .fold(self.clone(), |grid, &act| match grid.try_action(act) {
                Some(grid) => grid,
                None => grid,
            })
    }

    pub fn rand_actions(&self, n: i32) -> Self {
        // let mut r = rand::rngs::StdRng::seed_from_u64(10);
        let mut r = rand::thread_rng();
        let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let actions: Vec<(i32, i32)> = (0..n).map(|_| moves[r.gen_range(0..4)]).collect();
        self.do_actions(actions)
    }

    pub fn rand_actions_with_seed(&self, n: i32, s: u64) -> Self {
        let mut r = rand::rngs::StdRng::seed_from_u64(s);
        let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let actions: Vec<(i32, i32)> = (0..n).map(|_| moves[r.gen_range(0..4)]).collect();
        self.do_actions(actions)
    }

    pub fn expand(&self) -> Vec<Self> {
        let moves = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        moves
            .iter()
            .filter_map(|&action| self.try_action(action))
            .collect()
    }

    pub fn hash_with<T: StateHash>(&self, hasher: &T) -> u32 {
        (0..self.size.pow(2))
            .filter(|&i| self.data[i as usize].is_some())
            .map(|i| {
                let n = self.data[i as usize].unwrap();
                hasher.hash_prop(n, i)
            })
            .reduce(|acc, e| acc ^ e)
            .unwrap()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size.pow(2) {
            if i % self.size == 0 {
                if i == 0 {
                    write!(f, "[")?;
                } else {
                    write!(f, "\n[")?;
                }
            }
            match self.data[i as usize] {
                Some(num) => write!(f, "{:>5}", num)?,
                _ => write!(f, "{:>5}", "-")?,
            }
            if i % self.size == self.size - 1 {
                write!(f, "]")?;
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.size, other.size);
        (0..self.size.pow(2)).all(|i| self.data[i as usize] == other.data[i as usize])
    }
}

impl Eq for Grid {}

#[derive(Debug, Clone)]
pub struct Node {
    pub state: Grid,
    pub f: i32,
    pub g: i32,
    pub h: i32,
    // pub prev_actions: Box<Vec<(i32, i32)>>,
    // pub prev_node: Option<Box<Node>>,
}

impl Node {
    pub fn new(state: Grid) -> Self {
        Node {
            state,
            f: 0,
            g: 0,
            h: 0,
            // prev_actions: Box::new(vec![]),
            // prev_node: None,
        }
    }

    pub fn calc_cost(&mut self, end_state: &Grid, h_func: fn(&Grid, &Grid) -> i32) {
        self.h = h_func(&self.state, end_state);
        self.f = self.g + self.h;
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

pub struct Log {
    pub iter_cnt: i32,
    pub node_cnt: i32,
    pub abort_cnt: i32,
    // pub com_time: i32,
}

impl Log {
    pub fn new() -> Self {
        Log {
            iter_cnt: 0,
            node_cnt: 0,
            abort_cnt: 0,
        }
    }

    pub fn merge(&mut self, log: Log) -> () {
        self.iter_cnt += log.iter_cnt;
        self.abort_cnt += log.abort_cnt;
        self.node_cnt += log.abort_cnt;
    }
}

pub trait StateHash {
    // fn hash_grid(&self, g: &Grid) -> u32;
    // fn get_htable(&self) -> &Vec<Vec<u32>>;
    fn hash_prop(&self, _: i32, _: i32) -> u32;
}

#[derive(Debug, Clone)]
pub struct ZHasher {
    // num[loc[bits]]
    pub htable: Vec<Vec<u32>>,
}

impl ZHasher {
    pub fn new(size: i32) -> Self {
        let mut r = rand::rngs::StdRng::seed_from_u64(420);
        let htable: Vec<Vec<u32>> = (0..size.pow(2) - 1)
            .map(|_| (0..size.pow(2)).map(|_| r.gen()).collect())
            .collect();
        ZHasher { htable }
    }
}

impl StateHash for ZHasher {
    // fn hash_grid(&self, g: &Grid) -> u32 {
    //     let hval = 0;
    //     (0..g.size.pow(2))
    //         .filter(|&i| g.data[i as usize].is_some())
    //         .map(|i| {
    //             let n = g.data[i as usize].unwrap();
    //             self.htable[n as usize][i as usize]
    //         })
    //         .reduce(|acc, e| acc ^ e)
    //         .unwrap()
    // }

    // fn get_htable(&self) -> &Vec<Vec<u32>> {
    //     &self.htable
    // }

    fn hash_prop(&self, n: i32, i: i32) -> u32 {
        // Hash one propsition
        self.htable[n as usize][i as usize]
    }
}

#[derive(Debug, Clone)]
pub struct AZHasher {
    pub htable: Vec<Vec<u32>>,
    pub size: i32,
}

impl AZHasher {
    // This implementation uses a hand-craft abstract function.
    pub fn new(size: i32) -> Self {
        let mut r = rand::rngs::StdRng::seed_from_u64(100);
        let htable: Vec<Vec<u32>> = (0..size.pow(2) - 1)
            .map(|_| (0..size).map(|_| r.gen()).collect())
            .collect();
        AZHasher { htable, size }
    }

    pub fn abx(&self, i: i32) -> i32 {
        i / self.size
    }
}

impl StateHash for AZHasher {
    fn hash_prop(&self, n: i32, i: i32) -> u32 {
        self.htable[n as usize][self.abx(i) as usize]
    }
}
