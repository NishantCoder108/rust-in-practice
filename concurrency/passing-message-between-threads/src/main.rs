use std::sync::mpsc;
use std::time::Duration;
// use std::sync::mpsc::Sender;
use std::{thread, time};

fn main() {
    let (tx, receiver) = mpsc::channel::<String>();

    for thread_id in 0..10 {
        println!("Thread Id: {thread_id:?}");

        let tx_clone = tx.clone();

        thread::spawn(move || {
            for i in 0..10 {
                let mess = format!("Id {i}: spawn message sending at {thread_id}");
                tx_clone.clone().send(mess).unwrap();
            }
        });
    }

    println!("REC: {:?}", receiver.recv().unwrap());

    drop(tx); //it is important to drop `tx`, otherwise it will wait till all execution complete
    // for res in receiver {
    //     println!("RES: {res:?}");
    // }

    while let Ok(res) = receiver.recv_timeout(Duration::from_secs(1)) {
        println!("RES: {res:?}");
    }
    // thread::sleep(Duration::from_secs(2));
    // let send_res = tx.send("Sending data between threads".to_string()).unwrap();

    // // match send_res {
    // //     Ok(v) => println!("Sending data ..., {:#?}", v),
    // //     Err(e) => println!("Not able to sending...., Err: {}", e),
    // // }

    // thread::spawn(move || {
    //     tx.send("Sending data from separate threads".to_string())
    //         .unwrap();
    //     tx.send("Another sending data from separate threads".to_string())
    //         .unwrap();
    //     println!("Sent message from thread.");
    // });

    // for res_data in receiver {
    //     println!("Received message : {res_data:?}");
    // }
    // while let Ok(res) = receiver.recv() {
    //     println!("Received Mess: {res:?}");
    // }
    // let message = receiver.recv().unwrap();
    // println!("Main thread got data : {}", message);

    // let handler = thread::spawn(|| {
    //     println!("Handler");
    // })
    // .join()
    // .unwrap();

    // let num = [1, 2, 3, 3, 4, 5];
    // let vec = Vec::from(&num[0..6]);

    // println!("VEC: {vec:?}");

    // let (tx, rx) = mpsc::channel::<u32>();

    // let process_instruction = move || {
    //     let tx_clone = tx.clone();
    //     tx_clone.send(100).unwrap();
    //     tx_clone.send(200).unwrap();
    //     tx.send(300).unwrap();
    // };

    // let handler = thread::spawn(process_instruction);

    // // handler.join().unwrap();
    // // drop(tx);
    // for re in rx {
    //     // let rv_res = re.recv().unwrap();
    //     println!("Receiver result: {re:?}");
    // }
}

//Sending data between threads
// We can use - MPSC (multiple producer Single Consumer)

//for checking number of cpu: sysctl -n hw.ncpu
