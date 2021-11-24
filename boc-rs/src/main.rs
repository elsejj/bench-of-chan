use std::{ops::Sub, sync::{Arc, atomic::AtomicUsize}};

use clap::Clap;
use tokio::sync::mpsc;
use bytes::Bytes;

#[derive(Clap, Clone)]
struct Opts {

    #[clap(long, short, default_value="16", about="the size of event")]
    size: usize,
    #[clap(long, short, default_value="100", about="the count of workers")]
    worker: usize,
    #[clap(long, short, default_value="100", about="the count of events")]
    event: usize,
    #[clap(long, short, default_value="16", about="the size of worker queue")]
    queue: usize,
    #[clap(long, short, about="output as csv format")]
    csv: bool,
    #[clap(long, short, about="more output")]
    verbose: bool,
}

static SN : AtomicUsize = AtomicUsize::new(0);

type EventSender = mpsc::Sender<Bytes>;
type EventReceiver = mpsc::Receiver<Bytes>;

async fn worker(mut queue: EventReceiver, done: mpsc::Sender<usize>, target:usize) {
    while let Some(event) = queue.recv().await {
        if event.len() == 4 && event.eq_ignore_ascii_case(b"quit") {
            break;
        }
        let sn = SN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if sn + 1 == target {
            done.send(sn+1).await.unwrap();
        }
    }
}

async fn dispatch(opts: Opts, address: Vec<EventSender>) {

    let buff: Vec<u8> = Vec::with_capacity(opts.size);

    for addr in address.iter() {
        let addr = addr.clone();
        let events = opts.event;
        let event = Bytes::copy_from_slice(&buff);
        tokio::spawn(async move {
            for _ in 0..events {
                addr.send(event.clone()).await.unwrap();
            }
        });
    }
}


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let opts = Opts::parse();

    let (done_tx, mut done_rx) = mpsc::channel(1);
    let target = opts.event * opts.worker;
    
    // address is the send port of all workers
    let mut address = vec![];

    // startup workers
    for _ in 0..opts.worker {
        let (tx, rx) = mpsc::channel(opts.queue);
        address.push(tx);
        tokio::spawn(worker(rx, done_tx.clone(), target));
    }

    // t1 is the time of startup
    let t1 = chrono::Local::now();

    // spawn the dispatch
    tokio::spawn( dispatch(opts.clone(), address));

    // wait all task done
    if let Some(_done) = done_rx.recv().await {
        let t2 = chrono::Local::now();
        if opts.verbose {
            // verfiy done is real done
            println!("done target {}", _done);
        }
        if let Ok(ts) = t2.sub(t1).to_std() {
            let ts = ts.as_secs_f64();
            let speed = (opts.worker * opts.event) as f64 / ts;
            if opts.csv {
                println!("rs,{},{},{:.3},{:.3}", opts.worker, opts.event, ts, speed)
            }else{
                println!("workers   : {}", opts.worker);
                println!("events    : {}", opts.event);
                println!("time used : {:.3}S", ts);
                println!("Speed     : {:.3}/S", speed);
            }
        }
    }
    Ok(())
}
