use std::sync::Arc;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use twothousand_forty_eight::v2::recording::SeededRecording;

fn main() {
    println!();
    const LOG_INTERVAL: usize = 1000;
    let seeds_tried = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    // Needs to be valid base64
    let moves_to_seed = "PSwAKA4lEhQIBzMXMDAmADYfBTYmBjQIEzYuHBsCMxABDAQ+GD0qFjg7DiwrEBwpBB0FDiw";
    let seed = (0..usize::MAX).into_par_iter().find_first(|i| {
        match format!("::2:4:4:{}:{}", i, moves_to_seed).parse::<SeededRecording>() {
            Ok(recording) => {
                //println!("Seed: {}", i);
                //println!("{:?}", recording);
                match recording.get_current_board() {
                    Ok(_board) => true,
                    Err(_e) => {
                        if i % LOG_INTERVAL == 0 {
                            seeds_tried
                                .fetch_add(LOG_INTERVAL, std::sync::atomic::Ordering::Relaxed);
                            let tried = seeds_tried.load(std::sync::atomic::Ordering::Relaxed);
                            print!(
                                "seed {: >20} failed, {:.20}% of all possible\r",
                                i,
                                (tried as f64 / usize::MAX as f64) * 100.0
                            );
                        }
                        false
                    }
                }
            }
            Err(_e) => {
                if i % LOG_INTERVAL == 0 {
                    seeds_tried.fetch_add(LOG_INTERVAL, std::sync::atomic::Ordering::Relaxed);
                    let tried = seeds_tried.load(std::sync::atomic::Ordering::Relaxed);
                    print!(
                        "seed {: >20} failed, {:.20}% of all possible\r",
                        i,
                        (tried as f64 / usize::MAX as f64) * 100.0
                    );
                }
                false
            }
        }
    });
    match seed {
        Some(seed) => {
            println!();
            println!("Found seed: {}", seed);
        }
        None => println!("No seed found"),
    }
}
