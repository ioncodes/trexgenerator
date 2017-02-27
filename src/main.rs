extern crate raster;

use raster::Image;
use raster::Color;

fn main() {
    let color = Color::rgba(83, 83, 83, 255);
    let mut canvas = Image::blank(40, 43);
    for x in 22..38 {
        canvas.set_pixel(x, 0, color.clone()).unwrap();
    }
    for x in 22..38 {
        canvas.set_pixel(x, 1, color.clone()).unwrap();
    }
    for y in 2..11 {
        for x in 20..40 {
            if x == 24 && y == 3 || x == 24 && y == 4 || x == 25 && y == 3 || x == 25 && y == 4 {
                continue;
            }
            canvas.set_pixel(x, y, color.clone()).unwrap();
        }
    }
    for y in 11..13 {
        for x in 20..30 {
            canvas.set_pixel(x, y, color.clone()).unwrap();
        }
    }
    for y in 13..15 {
        for x in 20..36 {
            canvas.set_pixel(x, y, color.clone()).unwrap();
        }
    }
    for y in 15..17 {
        for x in 0..29 {
            if x == 0 || x == 1 || x >= 18 {
                canvas.set_pixel(x, y, color.clone()).unwrap();
            }
        }
    }
    for y in 17..19 {
        for x in 0..29 {
            if x == 0 || x == 1 || x >= 15 {
                canvas.set_pixel(x, y, color.clone()).unwrap();
            }
        }
    }
    for y in 19..21 {
        for x in 0..32 {
            if x == 0 || x == 1 || x == 2 || x == 3 || x >= 12 {
                canvas.set_pixel(x, y, color.clone()).unwrap();
            }
        }
    }
    for y in 21..23 {
        for x in 0..32 {
            if x == 0 || x == 1 || x == 2 || x == 3 || x == 4 || x == 5 || x >= 10 && x <= 27 || x == 30 || x == 31 {
                canvas.set_pixel(x, y, color.clone()).unwrap();
            }
        }
    }
    for y in 23..27 {
        for x in 0..28 {
            canvas.set_pixel(x, y, color.clone()).unwrap();
        }
    }
    for x in 2..28 {
        canvas.set_pixel(x, 27, color.clone()).unwrap();
    }
    for x in 2..26 {
        canvas.set_pixel(x, 28, color.clone()).unwrap();
    }
    for y in 29..31 {
        for x in 4..26 {
            canvas.set_pixel(x, y, color.clone()).unwrap();
        }
    }
    for y in 31..33 {
        for x in 6..24 {
            canvas.set_pixel(x, y, color.clone()).unwrap();
        }
    }
    for y in 33..35 {
        for x in 8..22 {
            canvas.set_pixel(x, y, color.clone()).unwrap();
        }
    }
    for y in 35..37 {
        for x in 10..22 {
            if x != 16 && x!= 17 {
                canvas.set_pixel(x, y, color.clone()).unwrap();
            }
        }
    }
    for y in 37..39 {
        for x in 10..22 {
            if x < 14 || x > 19 {
                canvas.set_pixel(x, y, color.clone()).unwrap();
            }
        }
    }
    for y in 39..41 {
        for x in 10..22 {
            if x == 10 || x == 11 || x == 20 || x == 21 {
                canvas.set_pixel(x, y, color.clone()).unwrap();
            }
        }
    }
    for y in 41..43 {
        for x in 10..24 {
            if x >= 10 && x <= 13 || x >= 20 && x <= 23 {
                canvas.set_pixel(x, y, color.clone()).unwrap();
            }
        }
    }
    raster::save(&canvas, "dino.png");
}
