use crossbeam::atomic::{self, AtomicCell};
use crossbeam::channel::{Receiver, Sender};
use rand::{thread_rng, Rng, SeedableRng};
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;

use crate::util::{expand, man_dist, Grid, Log, Node};

pub fn astar(init_state: &Grid, end_state: &Grid, h_func: fn(&Grid, &Grid) -> i32) -> Option<Node> {
    let num_threads = 8;
    // Initialize termination variables
    let msg_sent: Arc<AtomicCell<i32>> = Arc::new(AtomicCell::new(0));
    let msg_recv: Arc<AtomicCell<i32>> = Arc::new(AtomicCell::new(0));
    let term: Arc<AtomicCell<bool>> = Arc::new(AtomicCell::new(false));

    // Initialize communication channels
    let mut senders = Vec::with_capacity(num_threads);
    let mut receivers = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let (s, r) = crossbeam::channel::unbounded();
        senders.push(s);
        receivers.push(r);
    }

    // Initialize incumbent
    let incumbent = Arc::new(Mutex::new(Node::new(init_state.clone())));
    incumbent.lock().unwrap().f = i32::MAX;
    // let mut incumbent = Node::new(init_state.clone());
    // incumbent.f = i32::MAX;
    // let p_incumbent: Arc<AtomicPtr<_>> =
    //     Arc::new(AtomicPtr::new(Box::into_raw(Box::new(incumbent))));

    // Initialize threads
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
        let h = thread::spawn(move || {
            search(
                &init_state,
                &end_state,
                incumbent,
                num_threads as i32,
                h_func,
                rx,
                senders,
                msg_sent,
                msg_recv,
                term,
            );
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("terminated!");
    // let end;
    // unsafe {
    //     end = (*p_incumbent.load(Ordering::Relaxed)).clone();
    // }
    // Some(end)
    let end = incumbent.lock().unwrap();
    Some(end.clone())
}

pub fn search(
    start_state: &Grid,
    end_state: &Grid,
    incumbent: Arc<Mutex<Node>>,
    num_threads: i32,
    h_func: fn(&Grid, &Grid) -> i32,
    rx: Receiver<Node>,
    senders: Vec<Sender<Node>>,
    msg_sent: Arc<AtomicCell<i32>>,
    msg_recv: Arc<AtomicCell<i32>>,
    term: Arc<AtomicCell<bool>>,
) {
    // let mut first_iteration = true;
    let mut buffer: BinaryHeap<Node> = BinaryHeap::new();
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut open_states: HashMap<Grid, i32> = HashMap::new(); // map grid -> f
    let mut closed_states: HashMap<Grid, i32> = HashMap::new(); // map grid -> f
    let mut rng = rand::rngs::StdRng::seed_from_u64(10);
    let mut log = Log::new();

    // Initialization
    let mut start = Node::new(start_state.clone());
    start.calc_cost(end_state, h_func);
    open_states.insert(start.state.clone(), start.f);
    queue.push(start);

    // for _ in 0..1000
    loop {
        log.iter_cnt += 1;
        // Termination detection
        if term.load() {
            // println!("sent: {}", msg_sent.load(Ordering::SeqCst));
            // println!("received: {}", msg_recv.load(Ordering::SeqCst));
            println!("#iter: {}", log.iter_cnt);
            break;
        }
        // first_iteration = false;

        loop {
            if let Ok(msg) = rx.try_recv() {
                // msg_recv.fetch_add(1, Ordering::SeqCst);
                buffer.push(msg);
                continue;
            }
            break;
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
        if open_states.is_empty() || queue.peek().unwrap().f >= incumbent.lock().unwrap().f {
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
            println!("Found solution");
            term.store(true);
            let mut incumbent = incumbent.lock().unwrap();
            if node.f < incumbent.f {
                *incumbent = node.clone();
                continue;
            }
            // let p_incumbent = incumbent.load(Ordering::SeqCst);
            // if node.f < unsafe { (*p_incumbent).f } {
            //     incumbent.compare_exchange(
            //         p_incumbent,
            //         Box::into_raw(Box::new(node)),
            //         Ordering::SeqCst,
            //         Ordering::SeqCst,
            //     );
            //     continue;
            // }
        }

        let successors = expand(&node, end_state, h_func);
        for succ in successors {
            loop {
                let i = rng.gen_range(0..num_threads);
                match senders[i as usize].send(succ.clone()) {
                    Ok(_) => {
                        // msg_sent.fetch_add(1, Ordering::SeqCst);
                        // println!("send node f: {}, h: {}", c.f, c.h);
                        break;
                    }
                    Err(_) => continue,
                };
            }
        }
    }
}