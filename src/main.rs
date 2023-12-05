// Place pub before mod otherwise youu will get warnings about multiple methods not used in lcd_panel
pub mod lcd_panel;

use log::*;

use cstr_core::CString;

use std::time::Instant;

use esp_idf_hal::{delay, peripherals::Peripherals, units::FromValueType};

use esp_idf_hal::ledc::{
    config::TimerConfig,
    {LedcDriver, LedcTimerDriver},
};

use lvgl::font::Font;
use lvgl::style::Style;
use lvgl::widgets::Label;
use lvgl::{Align, Color, Display, DrawBuffer, Part, TextAlign, Widget};

use crate::lcd_panel::{LcdPanel, PanelConfig, PanelFlagsConfig, TimingFlagsConfig, TimingsConfig};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("=================== Starting APP! =========================");

    const HOR_RES: u32 = 800;
    const VER_RES: u32 = 480;
    const LINES: u32 = 12; // The number of lines (rows) that will be refreshed

    let peripherals = Peripherals::take().unwrap();

    #[allow(unused)]
    let pins = peripherals.pins;

    //============================================================================================================
    //               Create the LedcDriver to drive the backlight on the Lcd Panel
    //============================================================================================================
    let mut channel = LedcDriver::new(
        peripherals.ledc.channel0,
        LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &TimerConfig::new().frequency(25.kHz().into()),
        )
        .unwrap(),
        pins.gpio2,
    )
    .unwrap();
    channel.set_duty(channel.get_max_duty() / 2).unwrap();
    info!("Backlight turned on");

    lvgl::init();

    //============================================================================================================
    //                         Create driver for the LCD Panel
    //============================================================================================================
    let mut lcd_panel = LcdPanel::new(
        &PanelConfig::new(),
        &PanelFlagsConfig::new(),
        &TimingsConfig::new(),
        &TimingFlagsConfig::new(),
    )
    .unwrap();

    info!("=============  Registering Display ====================");
    let buffer = DrawBuffer::<{ (HOR_RES * LINES) as usize }>::default();
    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        lcd_panel
            .set_pixels_lvgl_color(
                refresh.area.x1.into(),
                refresh.area.y1.into(),
                (refresh.area.x2 + 1i16).into(),
                (refresh.area.y2 + 1i16).into(),
                refresh.colors,
            )
            .unwrap();
    })
    .unwrap();

    //===========================================================================================================
    //                               Create the User Interface
    //===========================================================================================================
    // Create screen and widgets
    let mut screen = display.get_scr_act().unwrap();
    let mut screen_style = Style::default();
    screen_style.set_bg_color(Color::from_rgb((0, 0, 255)));
    screen_style.set_radius(0);
    screen.add_style(Part::Main, &mut screen_style);

    let mut time = Label::new().unwrap();
    let mut style_time = Style::default();
    style_time.set_text_color(Color::from_rgb((255, 255, 255))); // white
    style_time.set_text_align(TextAlign::Center);

    // Custom font requires lvgl-sys in Cargo.toml and 'use lvgl_sys' in this file
    style_time.set_text_font(unsafe { Font::new_raw(lvgl_sys::gotham_bold_80) });

    time.add_style(Part::Main, &mut style_time);

    // Time text will be centered in screen
    time.set_align(Align::Center, 0, 0);

    let mut i = 0;

    loop {
        let start = Instant::now();
        if i > 59 {
            i = 0;
        }

        let val = CString::new(format!("21:{:02}", i)).unwrap();
        time.set_text(&val).unwrap();
        i += 1;

        lvgl::task_handler();

        // Simulate clock - so sleep for one second so time text is incremented in seconds
        delay::FreeRtos::delay_ms(1000);

        lvgl::tick_inc(Instant::now().duration_since(start));
    }
}
