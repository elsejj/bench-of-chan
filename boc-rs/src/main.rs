use std::{ops::Sub, sync::atomic::AtomicUsize};

use clap::Parser;
use tokio::sync::mpsc;

mod  pprof;

#[derive(Parser, Clone, Debug, Default)]
#[clap(author, version, about, long_about = None)]
struct Opts {

    /// the size of event, no used when event type is int(0)
    #[clap(long, short, default_value="16")]
    size: i64,

    /// event type, 0 for int 1 for str
    #[clap(long, short='t', default_value="0")]
    etype: i64,

    /// the count of workers
    #[clap(long, short, default_value="100")]
    worker: usize,

    /// the count of events per worker
    #[clap(long, short, default_value="100")]
    event: usize,

    /// the size of worker queue
    #[clap(long, short, default_value="16")]
    queue: usize,

    /// output as csv format, default will be json
    #[clap(long, short)]
    csv: bool,

    /// cpuprofile result
    #[clap(long, default_value="")]
    cpuprofile: String,

    /// more output
    #[clap(long, short)]
    verbose: bool,
}

static SN : AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Event{
    Int(i64),
    Str(String),
}

impl Event{
    pub fn new(etype: i64, seed: i64) -> Self {
        match etype {
            0 => Self::Int(seed),
            1 => Self::Str("A".repeat(seed as usize)),
            _ => panic!("invalid event type"),
        }


    }
    pub fn is_exit(&self) -> bool {
        match self {
            Event::Int(v) => (-1).eq(v),
            Event::Str(v) => v.eq("exit"),
        }
    } 
}


type EventSender = mpsc::Sender<Event>;
type EventReceiver = mpsc::Receiver<Event>;

async fn worker(mut queue: EventReceiver, done: mpsc::Sender<usize>, events:usize) {
    let order = std::sync::atomic::Ordering::Relaxed;
    for _i in 0..events {
        if let Some(event) = queue.recv().await {
            SN.fetch_add(1, order);
            if event.is_exit() {
                break;
            }
        }
    }
    done.send(1).await.unwrap();
}

async fn dispatch_to(event: Event, events: usize, addr: EventSender) {
    for _ in 0..events {
        addr.send(event.clone()).await.unwrap();
    }
}

async fn dispatch(opts: Opts, address: Vec<EventSender>) {

    let event = Event::new(opts.etype, opts.size);
    address.into_iter().for_each(|addr| {
        let event =  event.clone();
        let events = opts.event;
        tokio::spawn( async move {
            dispatch_to(event, events, addr).await
        });
    });
}


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let opts = Opts::parse();

    let prof = pprof::PProf::new(!opts.cpuprofile.is_empty());

    let (done_tx, mut done_rx) = mpsc::channel(opts.worker);
    let total_events = opts.event * opts.worker;
    
    // startup workers
    // address is the send port of all workers
    let mut address = vec![];
    for _ in 0..opts.worker {
        let (tx, rx) = mpsc::channel(opts.queue);
        address.push(tx);
        tokio::spawn(worker(rx, done_tx.clone(), opts.event));
    }

    // t1 is the time of startup
    let t1 = time::Instant::now();

    // spawn the dispatch
    tokio::spawn( dispatch(opts.clone(), address));


    // wait all task done
    for _i in 0..opts.worker {
        done_rx.recv().await;
    }

    let t2 = time::Instant::now();
    if opts.verbose {
        // verfiy done is real done
        println!("total events {} done events {}", total_events, SN.fetch_add(0, std::sync::atomic::Ordering::Relaxed));
    }
    let ts = t2.sub(t1).as_seconds_f64();
    let speed = (total_events) as f64 / ts;
    if opts.csv {
        println!("rs,{},{},{:.3},{:.3}", opts.worker, opts.event, ts, speed)
    }else{
        println!("workers   : {}", opts.worker);
        println!("events    : {}", opts.event);
        println!("time used : {:.3}S", ts);
        println!("Speed     : {:.3}/S", speed);
    }
    if !opts.cpuprofile.is_empty() {
        prof.report(&opts.cpuprofile)?;
    }
    Ok(())
}
