use std::comm::{sync_channel, SyncSender, Receiver};

fn is_divisible(num: int) -> bool {
    num % 3 == 0 || num % 5 == 0
}

fn finder(sender: SyncSender<int>, min: int, max: int) {
    println!("Starting finder: ({}, {})", min, max);
    for val in range(min, max) {
        if is_divisible(val) {
            sender.send(val as int);
        }
    }

    sender.send(-1);
}

fn collector(receiver: &Receiver<int>, main_sender: &Sender<int>) {
    let mut value:   int;
    let mut total:   int = 0;
    let mut workers: int = 10;

    println!("Starting collector...")

    while workers > 0 {
        value = receiver.recv();
        if value < 0 {
            workers -= 1;
            println!("Worker stopped, left: {}", workers);
        }
        else {
            total += value;
        }
    }

    main_sender.send(total);
}

fn main() {
    let (sender, receiver): (SyncSender<int>, Receiver<int>) = sync_channel(1);
    let (main_sender, main_receiver): (Sender<int>, Receiver<int>) = channel();

    for val in range(0i, 10) {
        let child_sender = sender.clone();
        spawn(proc() {
            finder(child_sender, val * 100, (val + 1) * 100);
        });
    }

    spawn(proc() {
        collector(&receiver, &main_sender);
    });

    let total_sum: int = main_receiver.recv();
    println!("Program finished with total value: {}", total_sum);
}
