#![feature(atomic_bool_fetch_not)]

use opencv::core::{Point, VecN};
use opencv::imgcodecs::IMREAD_COLOR;
use opencv::imgproc::{FONT_HERSHEY_COMPLEX, FONT_HERSHEY_PLAIN, LINE_4, LINE_8};
use opencv::prelude::*;
use opencv::viz::Color;
use opencv::{highgui, imgcodecs, Result};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    //let mut continue_running = Arc::new(AtomicBool::new(false));
    let mut image = imgcodecs::imread("twily.png", IMREAD_COLOR)?;

    let mut image_uwu = image.clone();

    opencv::imgproc::put_text(
        &mut image_uwu,
        "UwU",
        Point::new(10, 250),
        FONT_HERSHEY_PLAIN,
        10.0,
        VecN::new(200., 100., 200., 255.),
        10,
        LINE_4,
        false,
    )
    .expect("TODO: panic message");

    highgui::named_window("hello opencv!", 0)?;
    let mut show_uwu = Arc::new(AtomicBool::new(false));
    let show_uwu_ptr = show_uwu.clone();
    thread::spawn(move || loop {
        let v = show_uwu_ptr.load(Ordering::SeqCst);
        show_uwu_ptr.store(!v, Ordering::SeqCst);
        thread::sleep(Duration::from_secs(1));
    });
    'main_loop: loop {
        highgui::imshow(
            "hello opencv!",
            match show_uwu.load(Ordering::SeqCst) {
                true => &image_uwu,
                false => &image,
            },
        )
        .unwrap();
        let key = highgui::wait_key(1).expect("Oops");
        if key == 'q' as i32 {
            break 'main_loop;
        }
    }
    Ok(())
}
