use opencv::core::{Point, VecN};
use opencv::imgcodecs::IMREAD_COLOR;
use opencv::imgproc::{FONT_HERSHEY_COMPLEX, FONT_HERSHEY_PLAIN, LINE_4, LINE_8};
use opencv::prelude::*;
use opencv::viz::Color;
use opencv::{highgui, imgcodecs, Result};

fn main() -> Result<()> {
    let mut image = imgcodecs::imread("twily.png", IMREAD_COLOR)?;
    opencv::imgproc::put_text(
        &mut image,
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
    highgui::imshow("hello opencv!", &image)?;
    'main_loop: loop {
        let key = highgui::wait_key(10000)?;
        if key == 'q' as i32 {
            break 'main_loop;
        }
    }
    Ok(())
}
