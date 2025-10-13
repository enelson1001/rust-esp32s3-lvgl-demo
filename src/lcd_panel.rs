use core::cell::UnsafeCell;
use core::ffi::c_int;
use std::ptr::null_mut;

use esp_idf_svc::sys::{
    esp, esp_lcd_new_rgb_panel, esp_lcd_panel_del, esp_lcd_panel_draw_bitmap,
    esp_lcd_panel_handle_t, esp_lcd_panel_init, esp_lcd_panel_reset, esp_lcd_rgb_panel_config_t,
    esp_lcd_rgb_panel_config_t__bindgen_ty_1, esp_lcd_rgb_panel_config_t__bindgen_ty_2,
    esp_lcd_rgb_timing_t, esp_lcd_rgb_timing_t__bindgen_ty_1,
    soc_periph_lcd_clk_src_t_LCD_CLK_SRC_PLL160M, soc_periph_lcd_clk_src_t_LCD_CLK_SRC_PLL240M,
    EspError,
};

pub struct RgbPanelConfigBuilder {
    // esp_lcd_rgb_timing_t parameters
    pclk_hz: u32,
    h_res: u32,
    v_res: u32,
    hsync_pulse_width: u32,
    hsync_back_porch: u32,
    hsync_front_porch: u32,
    vsync_pulse_width: u32,
    vsync_back_porch: u32,
    vsync_front_porch: u32,
    hsync_idle_low: bool,
    vsync_idle_low: bool,
    de_idle_high: bool,
    pclk_active_neg: bool,
    pclk_idle_high: bool,

    // esp_lcd_rgb_panel_config_t parameters
    clk_src_ppl240m: bool,
    data_width: usize,
    bits_per_pixel: usize,
    num_fbs: usize,
    bounce_buffer_size_px: usize,
    sram_trans_align: usize,
    dma_burst_size: usize,
    hsync_gpio_num: c_int,
    vsync_gpio_num: c_int,
    de_gpio_num: c_int,
    pclk_gpio_num: c_int,
    disp_gpio_num: c_int,
    data_gpio_nums: [c_int; 16],
    disp_active_low: bool,
    refresh_on_demand: bool,
    fb_in_psram: bool,
    double_fb: bool,
    no_fb: bool,
    bb_invalidate_cache: bool,
}

impl Default for RgbPanelConfigBuilder {
    fn default() -> Self {
        Self {
            // esp_lcd_rgb_timing_t DEFAULT parameters
            pclk_hz: 16_000_000,
            h_res: 800,
            v_res: 480,
            hsync_pulse_width: 0,
            hsync_back_porch: 0,
            hsync_front_porch: 0,
            vsync_pulse_width: 0,
            vsync_back_porch: 0,
            vsync_front_porch: 0,
            hsync_idle_low: false,
            vsync_idle_low: false,
            de_idle_high: false,
            pclk_active_neg: false,
            pclk_idle_high: false,

            // esp_lcd_rgb_panel_config_t DEFAULT parameters
            clk_src_ppl240m: true,
            data_width: 0,
            bits_per_pixel: 0,
            num_fbs: 1, // Default to one frame buffer
            bounce_buffer_size_px: 0,
            sram_trans_align: 0,
            dma_burst_size: 64, // A common default, can be adjusted.
            hsync_gpio_num: -1, // -1 indicates not used
            vsync_gpio_num: -1,
            de_gpio_num: -1,
            pclk_gpio_num: -1,
            disp_gpio_num: -1,
            data_gpio_nums: [-1; 16],
            disp_active_low: false,
            refresh_on_demand: false,
            fb_in_psram: true,
            double_fb: false,
            no_fb: false,
            bb_invalidate_cache: false,
        }
    }
}

impl RgbPanelConfigBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /*
    ############################# Functions to set esp_lcd_rgb_timing_t parameters #############################
     */

    /// Sets the requency of pixel clock.
    pub fn pclk_hz(mut self, value: u32) -> Self {
        self.pclk_hz = value;
        self
    }

    /// Sets the horizontal resolution, i.e. the number of pixels in a line
    pub fn h_res(mut self, value: u32) -> Self {
        self.h_res = value;
        self
    }

    /// Sets the vertical resolution, i.e. the number of lines in the frame"
    pub fn v_res(mut self, value: u32) -> Self {
        self.v_res = value;
        self
    }

    /// Sets the horizontal sync width, unit: PCLK period
    pub fn hsync_pulse_width(mut self, value: u32) -> Self {
        self.hsync_pulse_width = value;
        self
    }

    /// Sets the horizontal back porch, number of PCLK between hsync and start of line active data
    pub fn hsync_back_porch(mut self, value: u32) -> Self {
        self.hsync_back_porch = value;
        self
    }

    /// Sets the horizontal front porch, number of PCLK between the end of active data and the next hsync
    pub fn hsync_front_porch(mut self, value: u32) -> Self {
        self.hsync_front_porch = value;
        self
    }

    /// Sets the vertical sync width, unit: number of lines
    pub fn vsync_pulse_width(mut self, value: u32) -> Self {
        self.vsync_pulse_width = value;
        self
    }

    /// Sets the vertical back porch, number of invalid lines between vsync and start of frame
    pub fn vsync_back_porch(mut self, value: u32) -> Self {
        self.vsync_back_porch = value;
        self
    }

    /// Sets the vertical front porch, number of invalid lines between the end of frame and the next vsync
    pub fn vsync_front_porch(mut self, value: u32) -> Self {
        self.vsync_front_porch = value;
        self
    }

    /// Sets hsync idle low flag. Set to true if hsync signal is LOW in idle state
    pub fn hsync_idle_low(mut self, flag: bool) -> Self {
        self.hsync_idle_low = flag;
        self
    }

    /// Sets the vsync idle low flag.  Set true if vsync signal is LOW in idle state
    pub fn vsync_idle_low(mut self, flag: bool) -> Self {
        self.vsync_idle_low = flag;
        self
    }

    /// Sets the de idle high flag.  Set to true if de dignal is HIGH in idle state
    pub fn de_idle_high(mut self, flag: bool) -> Self {
        self.de_idle_high = flag;
        self
    }

    /// Sets the pclk_active negative flag. Set true if the display data is clocked out on the falling edge of PCLK
    pub fn pclk_active_neg(mut self, flag: bool) -> Self {
        self.pclk_active_neg = flag;
        self
    }

    /// Sets the pclk idle high flag. Set true if The PCLK stays at high level in IDLE phase.
    pub fn pclk_idle_high(mut self, flag: bool) -> Self {
        self.pclk_idle_high = flag;
        self
    }

    /// Sets the clock source.  If true set clock source to PLL240M otherwise clock source is set to PLL160M
    pub fn clk_src_ppl240m(mut self, flag: bool) -> Self {
        self.clk_src_ppl240m = flag;
        self
    }

    /*
    ############################# Functions to set esp_lcd_rgb_panel_config_t parameters #############################
     */

    /// Sets the number of data lines.
    pub fn data_width(mut self, value: usize) -> Self {
        self.data_width = value;
        self
    }

    /// Sets the bits per pixel. Frame buffer color depth, in bpp, specially, if set to zero, it will default to data_width.
    pub fn bits_per_pixel(mut self, value: usize) -> Self {
        self.bits_per_pixel = value;
        self
    }

    /// Sets the number of frame buffers. Number of screen-sized frame buffers that allocated by the driver.
    /// By default (set to either 0 or 1) only one frame buffer will be used. Maximum number of buffers are 3
    pub fn num_fbs(mut self, value: usize) -> Self {
        self.num_fbs = value;
        self
    }

    /// Sets the bounce buffer size in pixels. If it's non-zero, the driver allocates two DRAM bounce buffers for DMA use.
    /// DMA fetching from DRAM bounce buffer is much faster than PSRAM frame buffer.
    pub fn bounce_buffer_size_px(mut self, value: usize) -> Self {
        self.bounce_buffer_size_px = value;
        self
    }

    /// Sets the SRAM transfer alignment. Alignment of buffers (frame buffer or bounce buffer) that allocated in SRAM
    pub fn sram_trans_align(mut self, value: usize) -> Self {
        self.sram_trans_align = value;
        self
    }

    /// Sets the DMA burst size in bytes
    pub fn dma_burst_size(mut self, value: usize) -> Self {
        self.dma_burst_size = value;
        self
    }

    /// Sets the HSYNC GPIO number.
    pub fn hsync_gpio_num(mut self, pin: c_int) -> Self {
        self.hsync_gpio_num = pin;
        self
    }

    /// Sets the VSYNC GPIO number.
    pub fn vsync_gpio_num(mut self, pin: c_int) -> Self {
        self.vsync_gpio_num = pin;
        self
    }

    /// Sets the DE GPIO number.
    pub fn de_gpio_num(mut self, pin: c_int) -> Self {
        self.de_gpio_num = pin;
        self
    }

    /// Sets the PCLK GPIO number.
    pub fn pclk_gpio_num(mut self, pin: c_int) -> Self {
        self.pclk_gpio_num = pin;
        self
    }

    /// Sets the DISP GPIO number.
    pub fn disp_gpio_num(mut self, pin: c_int) -> Self {
        self.disp_gpio_num = pin;
        self
    }

    /// Sets GPIO numbers for RGB565 data lines.
    /// [B3, B4, B5, B6, B7,  G2, G3, G4, G5, G6, G7   R3, R4, R5, R6, R7]
    pub fn data_gpio_nums(mut self, pins: &[c_int]) -> Self {
        let len = pins.len().min(self.data_gpio_nums.len());
        self.data_gpio_nums[0..len].copy_from_slice(&pins[0..len]);
        // Fill the rest with -1, though the default already does this.
        for i in len..self.data_gpio_nums.len() {
            self.data_gpio_nums[i] = -1;
        }
        self
    }

    /// Sets whether display is active. If true, a low level of display control signal can turn the screen on; vice versa
    pub fn disp_active_low(mut self, flag: bool) -> Self {
        self.disp_active_low = flag;
        self
    }

    /// Sets whether host only refreshes frame buffer.  If true, the host only refresh the frame buffer in esp_lcd_panel_draw_bitmap and esp_lcd_rgb_panel_refresh.
    pub fn refresh_on_demand(mut self, flag: bool) -> Self {
        self.refresh_on_demand = flag;
        self
    }

    /// Sets whether the frame buffer is in PSRAM.  If true then he frame buffer will be allocated from PSRAM, this is the prefered
    pub fn fb_in_psram(mut self, flag: bool) -> Self {
        self.fb_in_psram = flag;
        self
    }

    /// Sets whether the use a double frame buffer.  If true then the driver will allocate two screen sized frame buffer, same as num_fbs=2
    pub fn double_fb(mut self, flag: bool) -> Self {
        self.double_fb = flag;
        self
    }

    /// Sets wether to use frame buffer.  If true then the driver won't allocate frame buffer. Instead, user should fill in the bounce buffer manually in the on_bounce_empty callback
    pub fn no_fb(mut self, flag: bool) -> Self {
        self.no_fb = flag;
        self
    }

    /// Sets whether to invalidate cache. If this flag is enabled, in bounce back mode we'll do a cache invalidate on the read data,
    /// freeing the cache. Can be dangerous if data is written from other core(s).
    pub fn bb_invalidate_cache(mut self, flag: bool) -> Self {
        self.bb_invalidate_cache = flag;
        self
    }

    /// Builds the `esp_lcd_rgb_panel_panel_config_t` struct.
    pub fn build(&mut self) -> esp_lcd_rgb_panel_config_t {
        esp_lcd_rgb_panel_config_t {
            clk_src: {
                let mut clk_src = soc_periph_lcd_clk_src_t_LCD_CLK_SRC_PLL160M;
                if self.clk_src_ppl240m {
                    clk_src = soc_periph_lcd_clk_src_t_LCD_CLK_SRC_PLL240M;
                }

                clk_src
            },
            timings: esp_lcd_rgb_timing_t {
                pclk_hz: self.pclk_hz,
                h_res: self.h_res,
                v_res: self.v_res,
                hsync_pulse_width: self.hsync_pulse_width,
                hsync_back_porch: self.hsync_back_porch,
                hsync_front_porch: self.hsync_front_porch,
                vsync_pulse_width: self.vsync_pulse_width,
                vsync_back_porch: self.vsync_back_porch,
                vsync_front_porch: self.vsync_front_porch,

                // Set the bitfield using the provided bindgen setter function.
                flags: {
                    let mut flags = esp_lcd_rgb_timing_t__bindgen_ty_1::default();
                    flags.set_hsync_idle_low(self.hsync_idle_low as u32);
                    flags.set_vsync_idle_low(self.vsync_idle_low as u32);
                    flags.set_de_idle_high(self.de_idle_high as u32);
                    flags.set_pclk_active_neg(self.pclk_active_neg as u32);
                    flags.set_pclk_idle_high(self.pclk_idle_high as u32);

                    flags
                },
            },
            data_width: self.data_width,
            bits_per_pixel: self.bits_per_pixel,
            num_fbs: self.num_fbs,
            bounce_buffer_size_px: self.bounce_buffer_size_px,
            sram_trans_align: self.sram_trans_align,
            __bindgen_anon_1: esp_lcd_rgb_panel_config_t__bindgen_ty_1 {
                dma_burst_size: self.dma_burst_size,
            },
            hsync_gpio_num: self.hsync_gpio_num,
            vsync_gpio_num: self.vsync_gpio_num,
            de_gpio_num: self.de_gpio_num,
            pclk_gpio_num: self.pclk_gpio_num,
            disp_gpio_num: self.disp_gpio_num,
            data_gpio_nums: self.data_gpio_nums,

            // Set the bitfield using the provided bindgen setter function.
            flags: {
                let mut flags = esp_lcd_rgb_panel_config_t__bindgen_ty_2::default();
                flags.set_disp_active_low(self.disp_active_low as u32);
                flags.set_fb_in_psram(self.fb_in_psram as u32);
                flags.set_double_fb(self.double_fb as u32);
                flags.set_no_fb(self.no_fb as u32);
                flags.set_bb_invalidate_cache(self.bb_invalidate_cache as u32);

                flags
            },
        }
    }
}

pub struct EspLcdRgbPanel {
    pub panel_handle: esp_lcd_panel_handle_t,
}

impl EspLcdRgbPanel {
    pub fn new(panel_config: esp_lcd_rgb_panel_config_t) -> Result<Self, EspError> {
        let mut panel_handle = null_mut() as esp_lcd_panel_handle_t;

        unsafe {
            // create panel
            esp!(esp_lcd_new_rgb_panel(&panel_config, &mut panel_handle))?;

            // reset panel
            esp!(esp_lcd_panel_reset(panel_handle))?;

            // initialize panel
            esp!(esp_lcd_panel_init(panel_handle))?;
        };

        Ok(Self { panel_handle })
    }

    ///
    /// Sets pixel colors in a rectangular region.
    ///
    /// The color values from the `colors` iterator will be drawn to the given region starting
    /// at the top left corner and continuing, row first, to the bottom right corner. No bounds
    /// checking is performed on the `colors` iterator and drawing will wrap around if the
    /// iterator returns more color values than the number of pixels in the given region.
    ///
    /// # Arguments
    ///
    /// * `sx` - x coordinate start
    /// * `sy` - y coordinate start
    /// * `ex` - x coordinate end
    /// * `ey` - y coordinate end
    /// * `colors` - anything that can provide `IntoIterator<Item = lvgl::Color>` to iterate over pixel data
    pub fn set_pixels_lvgl_color<T>(
        &mut self,
        sx: c_int,
        sy: c_int,
        ex: c_int,
        ey: c_int,
        colors: T,
    ) -> Result<(), EspError>
    where
        T: IntoIterator<Item = lvgl::Color>,
    {
        let iter = UnsafeCell::new(colors);

        //let pixels = colors.as_ptr();

        unsafe {
            esp!(esp_lcd_panel_draw_bitmap(
                self.panel_handle,
                sx,
                sy,
                ex,
                ey,
                &iter as *const _ as _, //colors.as_ptr() as *const c_void,
            ))?;
        };

        Ok(())
    }
}

impl Drop for EspLcdRgbPanel {
    fn drop(&mut self) {
        esp!(unsafe { esp_lcd_panel_del(self.panel_handle) }).unwrap();
    }
}
