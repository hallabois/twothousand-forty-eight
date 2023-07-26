use std::sync::Arc;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use twothousand_forty_eight::v2::recording::SeededRecording;

fn print_status(seed: u32, failtype: &str, seeds_tried: u32) {
    print!(
        "seed {: >20} failed ({}), {:.20}% of all possible (≅{} tried)\r",
        seed,
        failtype,
        (seeds_tried as f64 / usize::MAX as f64) * 100.0,
        seeds_tried
    );
}

fn main() {
    println!();
    const LOG_INTERVAL: u32 = 10000;
    let seeds_tried = Arc::new(std::sync::atomic::AtomicU32::new(0));

    // Needs to be valid base64
    let moves_to_seed = "GSc8MSo2PBwNFyAzOgENEwA4ADUhETowOgUrLyk5GScaFgovGDEYGRwtMSA2IBkhLSweBxkgBwgGIAQgIAALJTQ3CwcfNzwrJiYTJio9ERspFCkEBAY";
    let seed = (0..u32::MAX).into_par_iter().find_any(|i| {
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
                            print_status(*i, "m", tried);
                        }
                        false
                    }
                }
            }
            Err(_e) => {
                if i % LOG_INTERVAL == 0 {
                    seeds_tried.fetch_add(LOG_INTERVAL, std::sync::atomic::Ordering::Relaxed);
                    let tried = seeds_tried.load(std::sync::atomic::Ordering::Relaxed);
                    print_status(*i, "p", tried);
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
