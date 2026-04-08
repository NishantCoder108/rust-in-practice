use std::sync::mpsc;
// use std::sync::mpsc::Sender;
use std::thread;

fn main() {
    // let handler = thread::spawn(|| {
    //     println!("Handler");
    // })
    // .join()
    // .unwrap();

    // let num = [1, 2, 3, 3, 4, 5];
    // let vec = Vec::from(&num[0..6]);

    // println!("VEC: {vec:?}");

    let (tx, rx) = mpsc::channel::<u32>();

    let process_instruction = move || {
        let tx_clone = tx.clone();
        tx_clone.send(100).unwrap();
        tx_clone.send(200).unwrap();
        tx.send(300).unwrap();
    };

    let handler = thread::spawn(process_instruction);

    // handler.join().unwrap();
    // drop(tx);
    for re in rx {
        // let rv_res = re.recv().unwrap();
        println!("Receiver result: {re:?}");
    }
}

//Sending data between threads
// We can use - MPSC (multiple producer Single Consumer)
