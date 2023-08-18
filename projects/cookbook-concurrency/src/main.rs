// https://rust-lang-nursery.github.io/rust-cookbook/concurrency.html

extern crate crossbeam;
extern crate crossbeam_channel;

use crossbeam_channel::bounded;
use rand::{distributions::Alphanumeric, Rng};
use rayon::{
    prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::{thread, time::Duration};

fn main() {
    let funcs = [
        spawn_a_shortlived_thread,
        create_a_parallel_pipeline,
        mutate_elements_of_an_array_in_parallel,
        search_for_predicate_in_values,
        sort_vector_in_parallel,
    ];
    for func in funcs.iter() {
        func()
    }
}

fn spawn_a_shortlived_thread() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    if let Some(max_val) = max {
        println!("Max value: {}", max_val)
    } else {
        println!("No max found!")
    }
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));
        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;
        Some(max_l.max(max_r))
    })
    .unwrap()
}

fn create_a_parallel_pipeline() {
    let (snd1, rcv1) = bounded::<i32>(1);
    let (snd2, rcv2) = bounded::<i32>(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // Producer thread.
        s.spawn(|_| {
            // Makes 4 messages.
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Sender1 sent {}", i);
            }
            // Close the channel from `snd1` – no more messages will be sent.
            drop(snd1);
        });

        for _ in 0..n_workers {
            // `rcv1` recieves messages from `snd1` and passes them to `snd2`
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                for msg in recvr.iter() {
                    println!("Worker {:?} received {}.", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        // Close the channel from `snd2` – no more messages will be sent.
        drop(snd2);

        // Collect all messages at the end of the pipeline.
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap();
}

fn mutate_elements_of_an_array_in_parallel() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}

fn search_for_predicate_in_values() {
    let vec = vec![2, 4, 6, 8];
    let any_even = vec.par_iter().any(|x| (*x % 2) == 0);
    println!("Any even values: {}", any_even);
}

fn sort_vector_in_parallel() {
    let mut vec = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|x| {
        let mut rng = rand::thread_rng();
        *x = (0..5).map(|_| rng.sample(&Alphanumeric) as char).collect()
    });
    vec.par_sort_unstable();
}

fn jpeg_thumbnails_in_parallel() {
    // TODO
    // https://rust-lang-nursery.github.io/rust-cookbook/concurrency/parallel.html#generate-jpg-thumbnails-in-parallel
}
