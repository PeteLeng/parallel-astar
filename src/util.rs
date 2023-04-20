#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::{Rng, SeedableRng};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};

// To make a general framework
// define trait, where a type has states represenation

#[derive(Debug, Clone, Hash)]
pub struct Grid {
    pub nsize: i32,
    pub empty: (i32, i32),
    pub tiles: Vec<Vec<Option<i32>>>,
}

pub trait State {
    fn expand(&self) -> Vec<Self>;
}

impl Grid {
    pub fn new(nsize: i32, empty: (i32, i32), tiles: Vec<Vec<Option<i32>>>) -> Self {
        Grid {
            nsize,
            empty,
            tiles,
        }
    }

    pub fn rand(nsize: i32, empty: (i32, i32)) -> Self {
        let mut nums: Vec<i32> = (0..nsize.pow(2) - 1).collect();
        let mut rng = rand::thread_rng();
        let mut tiles = vec![];
        for i in 0..nsize {
            let mut row = vec![];
            for j in 0..nsize {
                if i == empty.0 && j == empty.1 {
                    row.push(None);
                    continue;
                }
                let idx = rng.gen_range(0..nums.len()) as usize;
                row.push(Some(nums.remove(idx)));
                // row.push(Some(nums[idx]));
                // nums.remove(idx);
            }
            tiles.push(row);
        }
        // let (i, j) = empty;
        // tiles[i as usize][j as usize] = None;
        Grid {
            nsize,
            empty,
            tiles,
        }
    }

    pub fn rand_with_seed(nsize: i32, empty: (i32, i32), seed: u64) -> Self {
        let mut nums: Vec<i32> = (0..nsize.pow(2) - 1).collect();
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let mut tiles = vec![];
        for i in 0..nsize {
            let mut row = vec![];
            for j in 0..nsize {
                if i == empty.0 && j == empty.1 {
                    row.push(None);
                    continue;
                }
                let idx = rng.gen_range(0..nums.len()) as usize;
                row.push(Some(nums.remove(idx)));
                // row.push(Some(nums[idx]));
                // nums.remove(idx);
            }
            tiles.push(row);
        }
        // let (i, j) = empty;
        // tiles[i as usize][j as usize] = None;
        Grid {
            nsize,
            empty,
            tiles,
        }
    }

    pub fn valid_action(&self, action: (i32, i32)) -> bool {
        let x = self.empty.0 + action.0;
        let y = self.empty.1 + action.1;
        if x >= 0 && x < self.nsize && y >= 0 && y < self.nsize {
            return true;
        }
        false
    }

    pub fn move_on_copy(&self, action: (i32, i32)) -> Self {
        let mut tiles = self.tiles.clone();
        let empty = (self.empty.0 + action.0, self.empty.1 + action.1);
        tiles[self.empty.0 as usize][self.empty.1 as usize] =
            tiles[empty.0 as usize][empty.1 as usize];
        tiles[empty.0 as usize][empty.1 as usize] = None;
        Grid {
            nsize: self.nsize,
            empty,
            tiles,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.nsize {
            write!(f, "[")?;
            for j in 0..self.nsize {
                match self.tiles[i as usize][j as usize] {
                    Some(ele) => write!(f, "{:>5}", ele)?,
                    _ => write!(f, "{:>5}", "-")?,
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.nsize, other.nsize);
        for i in 0..self.nsize as usize {
            for j in 0..self.nsize as usize {
                if self.tiles[i][j] != other.tiles[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Grid {}

#[derive(Debug, Clone)]
pub struct Node {
    pub grid: Grid,
    pub f: i32,
    pub g: i32,
    pub h: i32,
    pub action: Option<(i32, i32)>,
}

impl Node {
    pub fn new(grid: Grid) -> Self {
        Node {
            grid,
            f: 0,
            g: 0,
            h: 0,
            action: None,
        }
    }

    pub fn evaluate(&mut self, end_state: &Grid, h_func: fn(&Grid, &Grid) -> i32) {
        self.h = h_func(&self.grid, end_state);
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
    for i in 0..g1.nsize {
        for j in 0..g1.nsize {
            map.insert(g2.tiles[i as usize][j as usize], (i, j));
        }
    }

    let mut dist_tot = 0;
    for i in 0..g1.nsize {
        for j in 0..g1.nsize {
            let v = g1.tiles[i as usize][j as usize];
            let dst = map[&v];
            // println!("({}, {}) -> ({}, {})", i, j, dst.0, dst.1);
            let dist = (dst.0 - i).abs() + (dst.1 - j).abs();
            dist_tot += dist;
        }
    }
    dist_tot
}

pub fn expand(node: &Node, end_state: &Grid, h_func: fn(&Grid, &Grid) -> i32) -> Vec<Node> {
    let actions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut nodes = vec![];
    for action in actions {
        if node.grid.valid_action(action) {
            let grid = node.grid.move_on_copy(action);
            let g = node.g + 1;
            let h = h_func(&grid, end_state);
            let f = g + h;
            nodes.push(Node {
                grid,
                f,
                g,
                h,
                action: Some(action),
            });
        }
    }
    nodes
}
