use std::fmt::Write;

#[derive(Debug)]
pub struct Chip8Display {
    screen_buffer_array: [[u8; Chip8Display::WIDTH / 8]; Chip8Display::HEIGHT],
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
                                if bit {
                                    write!(&mut acc_x, "⬜").unwrap();
                                } else {
                                    write!(&mut acc_x, "⬛").unwrap();
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
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub fn new() -> Chip8Display {
        Chip8Display {
            screen_buffer_array: [[0; 8]; 32],
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

    pub fn get_set_pixel_coords(&self) -> Vec<(f64, f64)> {
        let mut res: Vec<(f64, f64)> = Vec::new();
        for (y, data) in self.screen_buffer_array.iter().enumerate() {
            for (x_, x_byte) in data.iter().enumerate() {
                if *x_byte > 0 {
                    for i in 0..8 {
                        let bit_mask = 1 << i;
                        let bit = bit_mask & *x_byte > 0;
                        if bit {
                            res.push(((x_ * 8 + i) as f64, (Chip8Display::HEIGHT - y) as f64));
                        }
                    }
                }
            }
        }
        res
    }
}
