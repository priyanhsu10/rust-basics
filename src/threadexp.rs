use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
pub fn exp() {
    // create_thread();
    thread_communication();
}
fn thread_communication() {
    let (join_tx, join_rx) = mpsc::channel();
    let (sarah_tx, sarah_rx) = mpsc::channel();
    let jont = thread::spawn(move || {
        john_chat(sarah_tx, join_rx);
    });
    let sarht = thread::spawn(move || sarah_chat(join_tx, sarah_rx));
    match jont.join() {
        Ok(_) => {}
        Err(_) => {}
    }
    match sarht.join() {
        Ok(_) => {}
        Err(_) => {}
    }
}
fn sarah_chat(joh_tx: Sender<&str>, sarar_rx: Receiver<&str>) {
    let resutlt = sarar_rx.recv();
    println!("{}", resutlt.unwrap());
    let _sr = joh_tx.send("hello John");
}
fn john_chat(sarah_s: Sender<&str>, john_r: Receiver<&str>) {
    let result = john_r.recv();
    println!("{}", result.unwrap());
    let s = sarah_s.send("helow Sarah");
}
fn create_thread() {
    let mut out = 234;
    let t = thread::spawn(move || {
        println!("i am in thread, {}", out);
    });
    t.join();
    println!("i am in main {}", out);
}
