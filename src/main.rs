use image::{Rgb, RgbImage};

const CM_PER_IN: f32 = 2.54;
const PPI: u32 = 300;
const PPCM: u32 = (PPI as f32 / CM_PER_IN) as u32;

const PAPER_SIZE: [u32; 2] = [(8.5 * PPI as f32) as u32, (11.0 * PPI as f32) as u32];

const LENGTH_PIXELS: u32 = PPCM * 41 / 2;
const LINE_WEIGHT: u32 = PPI / 100;

const INCH_SUBDIVISIONS: u32 = 8;
const INCH_SUB_DIST: f32 = PPI as f32 / INCH_SUBDIVISIONS as f32;
const CM_SUBDIVISIONS: u32 = 2;
const CM_SUB_DIST: f32 = PPCM as f32 / CM_SUBDIVISIONS as f32;

const INCH_MARK_LENGTH: u32 = PPI / 3;
const CM_MARK_LENGTH: u32 = PPI / 4;

const CENTER_X: u32 = PAPER_SIZE[0] / 2;

const TOP: u32 = (PAPER_SIZE[1] - LENGTH_PIXELS) / 2;

const BLACK: Rgb<u8> = Rgb([0, 0, 0]);

fn main() {
    let mut buffer = RgbImage::from_fn(PAPER_SIZE[0], PAPER_SIZE[1], |_, _| Rgb([255, 255, 255]));

    // Draw
    for j in 0..=LENGTH_PIXELS {
        let y = TOP + j;
        // Draw center line
        for m in 0..LINE_WEIGHT {
            buffer.put_pixel(CENTER_X + m, y, BLACK);
        }
        // Draw inch marks
        if j as f32 % INCH_SUB_DIST < 1.0 {
            let mark_length =
                INCH_MARK_LENGTH / mod_div((j as f32 / INCH_SUB_DIST) as u32, INCH_SUBDIVISIONS);
            for m in 0..LINE_WEIGHT {
                for i in 0..mark_length {
                    let x = CENTER_X + i;
                    buffer.put_pixel(x, y + m, BLACK);
                }
            }
        }
        // Draw cm marks
        if j as f32 % CM_SUB_DIST < 1.0 {
            let mark_length =
                CM_MARK_LENGTH / mod_div((j as f32 / CM_SUB_DIST) as u32, CM_SUBDIVISIONS);
            for m in 0..LINE_WEIGHT {
                for i in 0..mark_length {
                    let x = CENTER_X - i;
                    buffer.put_pixel(x, y + m, BLACK);
                }
            }
        }
    }

    // Save
    buffer.save("ruler_tattoo.png").unwrap();
}

fn mod_div(i: u32, div: u32) -> u32 {
    if i % div == 0 {
        1
    } else {
        1 + mod_div(i, div / 2)
    }
}
