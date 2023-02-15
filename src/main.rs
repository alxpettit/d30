use opencv::core::{Point, VecN};
use opencv::highgui::WND_PROP_VISIBLE;
use opencv::imgcodecs::IMREAD_COLOR;
use opencv::imgproc::{FONT_HERSHEY_COMPLEX, FONT_HERSHEY_PLAIN, LINE_4, LINE_8};
use opencv::prelude::*;
use opencv::viz::Color;
use opencv::{highgui, imgcodecs, Result};
use snafu::{prelude::*, Error, Whatever};
use std::fmt::format;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

static WINDOW_NAME: &str = "TEST";

fn win_visible() -> Result<bool, Whatever> {
    let win_visible_prop = highgui::get_window_property(WINDOW_NAME, WND_PROP_VISIBLE)
        .with_whatever_context(|_| {
            format!("Could not retrieve window property from OpenCV API to check visibility")
        })?;
    Ok(win_visible_prop != 0.)
}

fn main() -> Result<(), Whatever> {
    let image = imgcodecs::imread("twily.png", IMREAD_COLOR)
        .with_whatever_context(|e| format!("Could not read source image"))?;

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
    .with_whatever_context(|_| format!("Failed to put text."))?;

    highgui::named_window(WINDOW_NAME, WND_PROP_VISIBLE)
        .with_whatever_context(|_| format!("Failed to create window: {}", WINDOW_NAME))?;

    let show_uwu = Arc::new(AtomicBool::new(false));
    let show_uwu_ptr = show_uwu.clone();

    thread::spawn(move || loop {
        let v = show_uwu_ptr.load(Ordering::SeqCst);
        show_uwu_ptr.store(!v, Ordering::SeqCst);
        thread::sleep(Duration::from_secs(1));
    });

    'main_loop: while win_visible()? {
        highgui::imshow(
            WINDOW_NAME,
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
