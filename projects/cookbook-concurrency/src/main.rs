// https://rust-lang-nursery.github.io/rust-cookbook/concurrency.html

extern crate crossbeam;
extern crate crossbeam_channel;

use crossbeam_channel::bounded;
use error_chain::error_chain;
use error_chain::ChainedError;
use glob::{glob_with, MatchOptions};
use image::{imageops::FilterType, ImageError};
use rand::{distributions::Alphanumeric, Rng};
use rayon::{
    prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::path::PathBuf;
use std::{fs::create_dir_all, path::Path, thread, time::Duration};

error_chain! {
    foreign_links {
        Image(ImageError);
        Io(std::io::Error);
        Glob(glob::PatternError);
    }
}

fn main() {
    let funcs = [
        spawn_a_shortlived_thread,
        create_a_parallel_pipeline,
        mutate_elements_of_an_array_in_parallel,
        search_for_predicate_in_values,
        sort_vector_in_parallel,
    ];
    for func in funcs.iter() {
        func();
        println!("–––––––––––––––––––––––––––––––––––––––");
    }
    match jpeg_thumbnails_in_parallel() {
        Ok(_) => println!("Successfully made thumbnails."),
        Err(x) => println!("Errored while making thumbnails: {}", x),
    };
    println!("–––––––––––––––––––––––––––––––––––––––");
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

fn jpeg_thumbnails_in_parallel() -> Result<()> {
    let options: MatchOptions = Default::default();
    let files: Vec<PathBuf> = glob_with("*.jpg", options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        error_chain::bail!("No JPEG files found in current directory.");
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .map_err(|e| e.chain_err(|| path.display().to_string()))
        })
        .filter_map(|x| x.err())
        .collect();

    println!("Failed images:");
    image_failures
        .iter()
        .for_each(|x| println!(" {}", x.display_chain()));
    println!("{} thumbnails created.", files.len() - image_failures.len());
    Ok(())
}

fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);
    Ok(img
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}
