use rand::{thread_rng, Rng, SeedableRng};
use std::cmp;
use std::collections::{BinaryHeap, HashMap};
use std::process::Termination;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Barrier, Mutex};
use std::thread;

use crate::util::{expand, man_dist, Grid, Node};

pub fn astar(init_state: &Grid, end_state: &Grid, h_func: fn(&Grid, &Grid) -> i32) -> () {
    let num_threads = 8;
    // Initialize termination variables
    let msg_sent = Arc::new(AtomicU64::new(0));
    let msg_recv = Arc::new(AtomicU64::new(0));
    let bar = Arc::new(Barrier::new(num_threads));
    let term = Arc::new(AtomicBool::new(false));

    // Initialize communication channels
    let mut senders = Vec::with_capacity(num_threads);
    let mut receivers = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let (tx, rx) = mpsc::channel();
        senders.push(tx);
        receivers.push(rx);
    }

    // Initialize incumbent
    let incumbent = Arc::new(Mutex::new(Node::new(init_state.clone())));
    incumbent.lock().unwrap().f = i32::MAX;

    // Initialize threads
    let mut handles = Vec::with_capacity(num_threads);
    for i in 0..num_threads {
        let init_state = init_state.clone();
        let end_state = end_state.clone();
        let senders = senders.clone();
        let rx = receivers.remove(0);
        let msg_sent = msg_sent.clone();
        let msg_recv = msg_recv.clone();
        let bar = bar.clone();
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
                bar,
                term,
            );
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("terminated!");
    let end = incumbent.lock().unwrap();
    println!("{}", end.state);
    println!("f: {}, g: {}, h: {}", end.f, end.g, end.h);
}

pub fn search(
    start_state: &Grid,
    end_state: &Grid,
    incumbent: Arc<Mutex<Node>>,
    num_threads: i32,
    h_func: fn(&Grid, &Grid) -> i32,
    rx: Receiver<Node>,
    senders: Vec<Sender<Node>>,
    msg_sent: Arc<AtomicU64>,
    msg_recv: Arc<AtomicU64>,
    bar: Arc<Barrier>,
    term: Arc<AtomicBool>,
) {
    // let mut first_iteration = true;
    let mut buffer: BinaryHeap<Node> = BinaryHeap::new();
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut open: HashMap<Grid, i32> = HashMap::new(); // map grid -> f
    let mut closed: HashMap<Grid, i32> = HashMap::new(); // map grid -> f

    // let mut termination = false;
    // let mut local_sent = 0;
    // let mut local_recv = 0;

    // Initialization
    let mut start = Node::new(start_state.clone());
    start.calc_cost(end_state, h_func);
    open.insert(start.state.clone(), start.f);
    queue.push(start);

    // for _ in 0..1540
    loop {
        // Termination detection
        if term.load(Ordering::SeqCst)
            && msg_sent.load(Ordering::SeqCst) == msg_recv.load(Ordering::SeqCst)
        {
            println!("sent: {}", msg_sent.load(Ordering::SeqCst));
            println!("received: {}", msg_recv.load(Ordering::SeqCst));
            break;
        }
        // first_iteration = false;

        // Check incoming messages
        // if term.load(Ordering::SeqCst) {
        //     msg_recv.fetch_add(local_recv, Ordering::SeqCst);
        //     msg_sent.fetch_add(local_sent, Ordering::SeqCst);
        // }
        loop {
            if let Ok(msg) = rx.try_recv() {
                msg_recv.fetch_add(1, Ordering::SeqCst);
                buffer.push(msg);
                continue;
            }
            break;
        }

        // Handle incoming messages
        while !buffer.is_empty() {
            let node = buffer.pop().unwrap();

            if let Some(&f) = closed.get(&node.state) {
                if f > node.f {
                    closed.remove(&node.state);
                } else {
                    continue;
                }
            }

            // Check if node is in open map.
            if open.contains_key(&node.state) {
                if node.f >= *open.get(&node.state).unwrap() {
                    continue;
                }
            }

            open.entry(node.state.clone())
                .and_modify(|f| *f = node.f)
                .or_insert(node.f);
            queue.push(node);
        }

        // Expand node from local queue
        // only if node has lower cost compared to incumbent
        if open.is_empty() {
            continue;
        }
        if term.load(Ordering::SeqCst) && queue.peek().unwrap().f >= incumbent.lock().unwrap().f {
            continue;
        }

        let node = queue.pop().unwrap();
        // Node with same grid but worse f value may still exist in queue.
        if let None = open.get(&node.state) {
            continue;
        }

        // println!("pop node f: {} h: {}", node.f, node.h);
        open.remove(&node.state);
        closed.insert(node.state.clone(), node.f);

        if node.state == *end_state {
            println!("Found solution");
            term.store(true, Ordering::SeqCst);
            let mut incumbent = incumbent.lock().unwrap();
            if node.f < incumbent.f {
                *incumbent = node.clone();
                continue;
            }
        }
        // lock dropped here.

        let cnodes = expand(&node, end_state, h_func);

        for c in cnodes {
            let mut rng = rand::rngs::StdRng::seed_from_u64(10);
            loop {
                let i = rng.gen_range(0..num_threads);
                match senders[i as usize].send(c.clone()) {
                    Ok(_) => {
                        msg_sent.fetch_add(1, Ordering::SeqCst);
                        // println!("send node f: {}, h: {}", c.f, c.h);
                        break;
                    }
                    Err(_) => continue,
                };
            }
        }
    }
}
