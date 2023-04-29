#![allow(unused_variables)]
use crossbeam::channel::{Receiver, Sender};
use std::collections::{BinaryHeap, HashMap};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;

use crate::utils::helpers::expand;
use crate::utils::structs::{Grid, Log, Node, StateHash};

pub fn astar<T: StateHash + Clone + Send + 'static>(
    init_state: &Grid,
    end_state: &Grid,
    h_func: fn(&Grid, &Grid) -> i32,
    num_threads: usize,
    hasher: T,
) -> Option<Node> {
    // let num_threads = 8;
    // Initialize termination variables
    let msg_sent = Arc::new(AtomicU64::new(0));
    let msg_recv = Arc::new(AtomicU64::new(0));
    let term = Arc::new(AtomicBool::new(false));

    // Initialize communication channels
    let mut senders = Vec::with_capacity(num_threads);
    let mut receivers = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let (s, r) = crossbeam::channel::unbounded();
        senders.push(s);
        receivers.push(r);
    }

    // with RwLock
    let incumbent = Arc::new(RwLock::new(Node::new(init_state.clone())));
    {
        let mut inc = incumbent.write().unwrap();
        inc.f = i32::MAX;
    }

    // Initialize threads
    // let hasher = ZHasher::new(init_state.size);
    // let hasher = AZHasher::new(init_state.size);
    let mut handles = Vec::with_capacity(num_threads);
    for i in 0..num_threads {
        let init_state = init_state.clone();
        let end_state = end_state.clone();
        let senders = senders.clone();
        let rx = receivers.remove(0);
        let msg_sent = msg_sent.clone();
        let msg_recv = msg_recv.clone();
        let term = term.clone();
        let incumbent = incumbent.clone();
        let hasher = hasher.clone();
        let h = thread::spawn(move || {
            search(
                &init_state,
                &end_state,
                incumbent,
                num_threads as i32,
                h_func,
                i as i32,
                rx,
                senders,
                msg_sent,
                msg_recv,
                term,
                hasher,
            )
        });
        handles.push(h);
    }

    // let mut main_log = Log::new();
    for h in handles {
        let log = h.join().unwrap();
        // main_log.merge(log);
    }
    // println!(
    //     "average iteration: {}",
    //     main_log.iter_cnt / num_threads as i32
    // );
    // println!("average abort: {}", main_log.abort_cnt / num_threads as i32);
    // println!("total nodes expanded: {}", main_log.iter_cnt);

    // println!("terminated!");

    // with RwLock
    let end = incumbent.read().unwrap().clone();
    Some(end)
}

pub fn search<T: StateHash>(
    start_state: &Grid,
    end_state: &Grid,
    incumbent: Arc<RwLock<Node>>,
    num_threads: i32,
    h_func: fn(&Grid, &Grid) -> i32,
    thread_num: i32,
    rx: Receiver<Node>,
    senders: Vec<Sender<Node>>,
    msg_sent: Arc<AtomicU64>,
    msg_recv: Arc<AtomicU64>,
    term: Arc<AtomicBool>,
    hasher: T,
) -> Log {
    // let mut first_iteration = true;
    let mut buffer: BinaryHeap<Node> = BinaryHeap::new();
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut open_states: HashMap<Grid, i32> = HashMap::new(); // map grid -> f
    let mut closed_states: HashMap<Grid, i32> = HashMap::new(); // map grid -> f
    let mut log = Log::new();

    // Initialization
    let mut start = Node::new(start_state.clone());
    start.calc_cost(end_state, h_func);
    open_states.insert(start.state.clone(), start.f);
    queue.push(start);

    loop {
        log.iter_cnt += 1;
        // Termination detection
        if term.load(Ordering::SeqCst) {
            // println!("sent: {}", msg_sent.load(Ordering::SeqCst));
            // println!("received: {}", msg_recv.load(Ordering::SeqCst));
            // println!("#iter: {}", log.iter_cnt);
            break;
        }
        // first_iteration = false;

        if log.iter_cnt % 2 == 0 {
            loop {
                if let Ok(msg) = rx.try_recv() {
                    // msg_recv.fetch_add(1, Ordering::SeqCst);
                    buffer.push(msg);
                    continue;
                }
                break;
            }
        }

        // Handle incoming messages
        while !buffer.is_empty() {
            let node = buffer.pop().unwrap();

            // if incoming node is in closed states
            if let Some(&g) = closed_states.get(&node.state) {
                if g > node.g {
                    // reopen state if it has lower cost.
                    closed_states.remove(&node.state);
                } else {
                    continue;
                }
            }

            // If incoming node is in open states
            if open_states.contains_key(&node.state) {
                // skip if node has higher cost
                if node.f >= *open_states.get(&node.state).unwrap() {
                    continue;
                }
            }

            // incoming node either not exists in open states
            // or has lower cost
            open_states
                .entry(node.state.clone())
                .and_modify(|f| *f = node.f)
                .or_insert(node.f);
            queue.push(node);
        }

        // Expand node from local queue
        // skip if open_states is empty or local node is worse than incumbent
        if open_states.is_empty() || queue.peek().unwrap().f >= incumbent.read().unwrap().f {
            log.abort_cnt += 1;
            continue;
        }

        let mut node;
        // nodes in local queue may no longer exist in open states
        loop {
            node = queue.pop().unwrap();
            if open_states.contains_key(&node.state) {
                break;
            }
        }

        open_states.remove(&node.state);
        closed_states.insert(node.state.clone(), node.g);

        if node.state == *end_state {
            // println!("Reach end state.");
            term.store(true, Ordering::SeqCst);
            let mut incumbent = incumbent.write().unwrap();
            if node.f < incumbent.f {
                *incumbent = node.clone();
                continue;
            }
        }

        let successors = expand(&node, end_state, h_func);
        for succ in successors {
            loop {
                let i = succ.state.hash_with(&hasher) % (num_threads as u32);
                if i == thread_num as u32 {
                    buffer.push(succ);
                    break;
                }
                match senders[i as usize].send(succ.clone()) {
                    Ok(_) => {
                        log.node_cnt += 1;
                        // msg_sent.fetch_add(1, Ordering::SeqCst);
                        break;
                    }
                    Err(_) => continue,
                };
            }
        }
    }
    log
}
