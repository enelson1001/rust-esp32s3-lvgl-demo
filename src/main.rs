// Place pub before mod otherwise youu will get warnings about multiple methods not used in lcd_panel
pub mod lcd_panel;

use log::*;

use cstr_core::CString;
use std::time::Duration;

use esp_idf_svc::hal::{
    delay::FreeRtos,
    ledc::{
        config::TimerConfig,
        {LedcDriver, LedcTimerDriver},
    },
    peripherals::Peripherals,
    units::FromValueType,
};
use esp_idf_svc::timer::EspTaskTimerService;

use lvgl::font::Font;
use lvgl::style::Style;
use lvgl::widgets::Label;
use lvgl::{Align, Color, Display, DrawBuffer, Part, TextAlign, Widget};

use crate::lcd_panel::{EspLcdRgbPanel, RgbPanelConfigBuilder};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("=================== Starting APP! =========================");

    const HOR_RES: u32 = 800;
    const VER_RES: u32 = 480;
    const LINES: u32 = 2; // The number of lines (rows) that will be refreshed

    let peripherals = Peripherals::take().unwrap();

    #[allow(unused)]
    let pins = peripherals.pins;

    //============================================================================================================
    //               Create the LedcDriver to drive the backlight on the Lcd Panel
    //============================================================================================================
    info!("Create LedcDriver");
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

    channel.set_duty(channel.get_max_duty()).unwrap();
    info!("Backlight turned on");

    //============================================================================================================
    //                         Create driver for the LCD Panel
    //============================================================================================================
    info!("Create LCD panel");
    let lcd_panel_config = RgbPanelConfigBuilder::new()
        .h_res(800)
        .v_res(480)
        .pclk_hz(16_000_000)
        .hsync_pulse_width(1)
        .hsync_back_porch(16)
        .hsync_front_porch(210)
        .vsync_pulse_width(1)
        .vsync_back_porch(10)
        .vsync_front_porch(22)
        .hsync_idle_low(false)
        .vsync_idle_low(false)
        .de_idle_high(false)
        .pclk_active_neg(true)
        .pclk_idle_high(false)
        .clk_src_ppl240m(true) // CONFIG_ESP_DEFAULT_CPU_FREQ_MHZ_240=y must be set in sdkconfig.defaults
        .data_width(16)
        .bits_per_pixel(0)
        .num_fbs(1)
        .bounce_buffer_size_px(3200)
        .sram_trans_align(8) // tried 16, 32, 64 but did not make a differnce
        .dma_burst_size(64)
        .hsync_gpio_num(39)
        .vsync_gpio_num(40)
        .de_gpio_num(41)
        .pclk_gpio_num(42)
        .disp_gpio_num(-1)
        // gpio's for RGB565 data lines - [B3, B4, B5, B6, B7,   G2, G3, G4, G5, G6, G7    R3, R4, R5, R6, R7]
        //                                [15,  7,  6,  5,  4,    9, 46,  3,  8, 16,  1,   14, 21, 47, 48, 45]
        .data_gpio_nums(&[15, 7, 6, 5, 4, 9, 46, 3, 8, 16, 1, 14, 21, 47, 48, 45])
        .disp_active_low(false)
        .refresh_on_demand(false)
        .fb_in_psram(true)
        .double_fb(false)
        .no_fb(false)
        .bb_invalidate_cache(false)
        .build();

    let mut lcd_panel = EspLcdRgbPanel::new(lcd_panel_config).unwrap();

    lvgl::init();

    info!("Registering Display");
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
    info!("Creating UI");
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

    // Create a periodic timer to call LVGL tick. LVGL needs a system tick to know elapsed time for animations and other tasks.
    info!("Creating Lvgl Tick Timer");
    let timer_service_01 = EspTaskTimerService::new().unwrap();
    let lvgl_tick_timer = timer_service_01
        .timer(move || {
            lvgl::tick_inc(Duration::from_millis(10));
        })
        .unwrap();

    // Let it trigger every 10 milliseconds
    lvgl_tick_timer.every(Duration::from_millis(10)).unwrap();

    loop {
        //let start = Instant::now();

        if i > 59 {
            i = 0;
        }

        let val = CString::new(format!("21:{:02}", i)).unwrap();
        time.set_text(&val);
        i += 1;

        lvgl::task_handler();

        // Simulate clock - so sleep for one second so time text is incremented in seconds
        FreeRtos::delay_ms(1000);

        //lvgl::tick_inc(Instant::now().duration_since(start));
    }
}
