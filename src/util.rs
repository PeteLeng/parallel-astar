#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::{Rng, SeedableRng};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash)]
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

    pub fn rand(size: i32, empty_idx: i32) -> Self {
        let mut rng = rand::thread_rng();
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

    pub fn rand_with_seed(size: i32, empty_idx: i32, seed: u64) -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
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

    pub fn expand(&self) -> Vec<Self> {
        let moves = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        moves
            .iter()
            .filter_map(|&action| self.try_action(action))
            .collect()
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
    pub prev_actions: Box<Vec<(i32, i32)>>,
}

impl Node {
    pub fn new(state: Grid) -> Self {
        Node {
            state,
            f: 0,
            g: 0,
            h: 0,
            prev_actions: Box::new(vec![]),
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

pub fn man_dist(g1: &Grid, g2: &Grid) -> i32 {
    // Assume grids with same size and same elements.
    let mut map = HashMap::new();
    for i in 0..g1.size.pow(2) {
        map.insert(g2.data[i as usize], g2.get_coord(i));
    }
    // println!("{:?}", map);

    let mut dist = 0;
    for i in 0..g1.size.pow(2) {
        let (x1, y1) = g1.get_coord(i);
        let &(x2, y2) = map.get(&g1.data[i as usize]).unwrap();
        dist += (x1 - x2).abs() + (y1 - y2).abs();
    }
    dist
}

pub fn expand(node: &Node, end_state: &Grid, h_func: fn(&Grid, &Grid) -> i32) -> Vec<Node> {
    let states = node.state.expand();
    states
        .into_iter()
        .map(|state| {
            let g = node.g + 1;
            let h = h_func(&state, end_state);
            let f = g + h;
            let (x, y) = state.get_coord(state.empty_idx);
            let (px, py) = node.state.get_coord(node.state.empty_idx);
            let mut prev_actions = (*node.prev_actions).clone();
            prev_actions.push((x - px, y - py));
            Node {
                state,
                f,
                g,
                h,
                prev_actions: Box::new(prev_actions),
            }
        })
        .collect()
}

pub fn print_path(node: &Node) {
    let mut state = node.state.clone();
    let mut actions = (*node.prev_actions).clone();
    println!("{}", state);
    while let Some(act) = actions.pop() {
        let act_rev = (-act.0, -act.1);
        state = state.do_action(act_rev);
        println!("{}", state);
    }
}

pub struct Log {
    pub iter_cnt: i32,
    pub expn_cnt: i32,
}

impl Log {
    pub fn new() -> Self {
        Log {
            iter_cnt: 0,
            expn_cnt: 0,
        }
    }
}
