use derive_more::Display;
use opencv::core::{Point, VecN};
use opencv::freetype::prelude::*;
use opencv::highgui::{QT_FONT_BOLD, WND_PROP_VISIBLE};
use opencv::imgcodecs::IMREAD_COLOR;
use opencv::imgproc::{FONT_HERSHEY_COMPLEX, FONT_HERSHEY_PLAIN, LINE_4, LINE_8, LINE_AA};
use opencv::prelude::*;
use opencv::viz::{Color, LINE_WIDTH};
use opencv::{highgui, imgcodecs, Result};
use snafu::{prelude::*, Error, Whatever};
use std::fmt::format;
use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Display)]
struct Window {
    name: &'static str,
}

impl From<Window> for &str {
    fn from(value: Window) -> Self {
        &value.name
    }
}

impl Window {
    fn name(&self) -> &str {
        self.name
    }

    fn mkwin(&self, flags: i32) -> Result<(), Whatever> {
        highgui::named_window(&self.name(), flags)
            .with_whatever_context(|_| format!("Failed to create window: {}", WINDOW.name))?;
        Ok(())
    }

    fn get_win_property(&self, prop_id: i32) -> std::result::Result<f64, Whatever> {
        highgui::get_window_property(&self.name(), prop_id).with_whatever_context(|_| {
            format!("Could not retrieve window property from OpenCV API to check visibility")
        })
    }

    fn win_visible(&self) -> Result<bool, Whatever> {
        let win_visible_prop = self.get_win_property(WND_PROP_VISIBLE)?;
        Ok(win_visible_prop != 0.)
    }

    fn win_display_frame(&self, mat: &dyn opencv::core::ToInputArray) -> Result<(), Whatever> {
        highgui::imshow(&self.name(), mat)
            .with_whatever_context(|_| format!("Could not render next frame"))?;
        Ok(())
    }
}

static WINDOW: Window = Window { name: "TEST" };

fn main() -> Result<(), Whatever> {
    let image = imgcodecs::imread("twily.png", IMREAD_COLOR)
        .with_whatever_context(|e| format!("Could not read source image"))?;

    let mut image_uwu = image.clone();

    let mut ft2 = opencv::freetype::create_free_type2().unwrap();
    ft2.load_font_data("RobotoMono-Medium.ttf", 0).unwrap();
    ft2.put_text(
        &mut image_uwu,
        "UwU",
        Point::new(10, 250),
        250,
        VecN::new(200., 100., 200., 255.),
        -1, // negative = filled
        LINE_AA,
        false,
    )
    .with_whatever_context(|_| format!("Failed to put text."))?;

    WINDOW.mkwin(WND_PROP_VISIBLE)?;

    let show_uwu = Arc::new(AtomicBool::new(false));
    let show_uwu_ptr = show_uwu.clone();

    thread::spawn(move || loop {
        let v = show_uwu_ptr.load(Ordering::SeqCst);
        show_uwu_ptr.store(!v, Ordering::SeqCst);
        thread::sleep(Duration::from_secs(1));
    });

    'main_loop: while WINDOW.win_visible()? {
        WINDOW.win_display_frame(match show_uwu.load(Ordering::SeqCst) {
            true => &image_uwu,
            false => &image,
        })?;
        let key = highgui::wait_key(1).expect("Oops");
        if key == 'q' as i32 {
            break 'main_loop;
        }
    }

    Ok(())
}
