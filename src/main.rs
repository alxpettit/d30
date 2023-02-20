use derive_more::Display;
use opencv::core::{Point, Scalar, Size, VecN, CV_8UC4};
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
use tracing::debug;
use tracing_subscriber::EnvFilter;

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
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    debug!("Init");
    let print_size = Size::new(320, 96);
    let image = Mat::new_size_with_default(print_size, CV_8UC4, Scalar::all(255.0))
        .with_whatever_context(|e| format!("Could not make Mat canvas: {}", e))?;

    let mut image_uwu = image.clone();

    let mut ft2 = opencv::freetype::create_free_type2().unwrap();
    ft2.load_font_data("RobotoMono-Medium.ttf", 0).unwrap();
    // negative thickness = filled
    let text_thickness = -1;
    let mut font_height = print_size.height;
    let text_str = "UwUu";
    let mut base_line = 0;
    let mut text_size = ft2
        .get_text_size(text_str, font_height, text_thickness, &mut base_line)
        .expect("Failed to get text size");
    let mut word_wrap_count = 0;

    // 5 is offset
    while text_size.width > print_size.width {
        //word_wrap_count += 1;
        //font_height /= 2;
        font_height -= 1;
        text_size = ft2
            .get_text_size(text_str, font_height, text_thickness, &mut base_line)
            .expect("Failed to get text size");
        // 2;
    }
    //
    // for _ in 0..=word_wrap_count {
    //     let (first_half, second_half) = text_str.chars().into_iter().chunks();

    let x_offset = (print_size.width - text_size.width) / 2;
    let y_offset = -(print_size.height - text_size.height) / 2;
    ft2.put_text(
        &mut image_uwu,
        text_str,
        Point::new(x_offset, y_offset),
        font_height,
        VecN::new(200., 100., 200., 100.),
        text_thickness,
        LINE_AA,
        false, // our origin is top-left
    )
    .with_whatever_context(|_| format!("Failed to put text."))?;
    //}
    //ft2.get_text_size();

    WINDOW.mkwin(WND_PROP_VISIBLE)?;

    let show_uwu = Arc::new(AtomicBool::new(false));
    let show_uwu_ptr = show_uwu.clone();

    thread::spawn(move || loop {
        let v = show_uwu_ptr.load(Ordering::SeqCst);
        show_uwu_ptr.store(!v, Ordering::SeqCst);
        thread::sleep(Duration::from_secs(1));
    });

    let mut update_frame = false;
    let mut old_show_uwu: bool = true;
    'main_loop: while WINDOW.win_visible()? {
        if show_uwu.load(Ordering::SeqCst) != old_show_uwu {
            update_frame = true;
        }

        if update_frame {
            update_frame = false;
            WINDOW.win_display_frame(match show_uwu.load(Ordering::SeqCst) {
                true => &image_uwu,
                false => &image,
            })?;
            old_show_uwu = show_uwu.load(Ordering::SeqCst);
        }
        let key = highgui::wait_key(10).expect("Oops");
        if key == 'q' as i32 {
            break 'main_loop;
        }
    }

    Ok(())
}
