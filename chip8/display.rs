use std::fmt::Write;

#[derive(Debug)]
pub struct Chip8Display {
    screen_buffer_array: [[u8; 64 / 8]; 32],
    debug_x: u8,
    debug_y: u8,
}

impl std::fmt::Display for Chip8Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let tmp = self.raw_array[0][0].to_string();
        let result =
            self.screen_buffer_array
                .iter()
                .enumerate()
                .fold(String::new(), |mut acc_y, y| {
                    write!(
                        &mut acc_y,
                        "{}\n",
                        y.1.iter().enumerate().fold(String::new(), |mut acc_x, x| {
                            for i in 0..8 {
                                let bit_mask = 1 << i;
                                let bit = bit_mask & x.1 > 0;
                                if self.debug_x as usize == (x.0 * 8 + i)
                                    && self.debug_y as usize == y.0
                                {
                                    write!(&mut acc_x, "🟥").unwrap();
                                } else {
                                    if bit {
                                        write!(&mut acc_x, "⬜").unwrap();
                                    } else {
                                        write!(&mut acc_x, "⬛").unwrap();
                                    }
                                }
                            }
                            acc_x
                        })
                    )
                    .unwrap();
                    acc_y
                });

        f.write_fmt(format_args!("{}", result))
    }
}

impl Chip8Display {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 32;

    pub fn new() -> Chip8Display {
        Chip8Display {
            screen_buffer_array: [[0; 8]; 32],
            debug_x: 255,
            debug_y: 255,
        }
    }

    // Make set pixel return the XOR result and remove get/unset pixel
    pub fn set_pixel(&mut self, x: usize, y: usize) {
        let w_x = x % Chip8Display::WIDTH;
        let w_y = y % Chip8Display::HEIGHT;

        let sub_pixel_x = w_x / 8;
        // 0000 0110
        let pixel = 128 >> (7 - (w_x % 8));

        self.screen_buffer_array[w_y][sub_pixel_x] =
            self.screen_buffer_array[w_y][sub_pixel_x] | pixel;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        let w_x = x % Chip8Display::WIDTH;
        let w_y = y % Chip8Display::HEIGHT;

        let sub_pixel_x = w_x / 8;
        let pixel = 128 >> (7 - (w_x % 8));

        (self.screen_buffer_array[w_y][sub_pixel_x] & pixel) > 0
    }

    pub fn unset_pixel(&mut self, x: usize, y: usize) {
        let w_x = x % Chip8Display::WIDTH;
        let w_y = y % Chip8Display::HEIGHT;

        let sub_pixel_x = w_x / 8;
        let pixel = 128 >> (7 - (w_x % 8));

        self.screen_buffer_array[w_y][sub_pixel_x] =
            self.screen_buffer_array[w_y][sub_pixel_x] & !pixel;
    }

    pub fn clear(&mut self) {
        for data_row in self.screen_buffer_array.iter_mut() {
            for data in data_row.iter_mut() {
                *data = 0;
            }
        }
    }

    //Adds a red dot at the specified location (There can be only 1 debug pixel set in the screen buffer)
    pub fn _set_debug(&mut self, x: u8, y: u8) {
        self.debug_x = x;
        self.debug_y = y;
    }
}
