use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    // TODO: implement parallel map!
    // init output_vec with 0
    output_vec.resize_with(input_vec.len(), Default::default);
    let (in_sender, in_receiver) =  crossbeam_channel::unbounded();
    let (out_sender, out_receiver) = crossbeam_channel::unbounded();
    let mut threads = vec![];

    // worker threads
    for _ in 0..num_threads{
        let in_receiver = in_receiver.clone();
        let out_sender = out_sender.clone();

        threads.push(
            thread::spawn(move ||{
                while let Ok(next_pair) = in_receiver.recv(){
                    let (index, value) = next_pair;
                    out_sender.send((index, f(value))).expect("NO Receiver");
                }
            })
        )
    }

    let len = input_vec.len();
    // start worker threads
    for i in 0..len{
        in_sender.send((len - i - 1, input_vec.pop().unwrap())).expect("NO Receiver");
    }

    // terminate the worker thread
    drop(in_sender);
    // terminate the main thread
    drop(out_sender);

    // process the worker thread messages
    while let Ok(result_pair) = out_receiver.recv(){
        let (index, value) = result_pair;
        output_vec[index] = value;
    }

    // join all worker threads
    for thread in threads{
        thread.join().expect("Some thing Wrong in the thread !");
    }

    output_vec
}


fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
