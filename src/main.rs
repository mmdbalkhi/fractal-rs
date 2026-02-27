use crate::complex::Complex;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

mod complex;

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r_pre, g_pre, b_pre) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (r_pre + m, g_pre + m, b_pre + m)
}

fn main() -> std::io::Result<()> {
    let width = 1000;
    let height = 1000;
    let max_iter = 100;

    let file = File::create("output.ppm")?;
    let mut writer = BufWriter::with_capacity(1024 * 1024, file);
    let start_time = Instant::now();

    writeln!(writer, "P6\n{} {}\n255", width, height)?;

    for y in 0..height {
        let row_pixels: Vec<u8> = (0..width)
            .into_par_iter()
            .flat_map(|x| {
                let c = Complex::pixel_to_complex(x, y, width, height, -0.75, -0.74, 0.10, 0.11);
                let cx = c.re;
                let cy = c.im;

                let cy2 = cy * cy;
                let q = (cx - 0.25).powi(2) + cy2;

                if q * (q + (cx - 0.25)) < 0.25 * cy2 || (cx + 1.0).powi(2) + cy2 < 0.0625 {
                    return vec![0, 5, 25];
                }

                let mut z = Complex::new(0.0, 0.0);
                let mut iter = max_iter;
                let mut smooth = 0f64;
                let mut min_dist = 1e10;

                for i in 0..max_iter {
                    z = z.square().add(&c);
                    let mag_sq = z.magnitude_squared();

                    if mag_sq < min_dist {
                        min_dist = mag_sq;
                    }

                    if mag_sq > 256.0 {
                        iter = i;
                        smooth = (i as f64) + 1.0 - (mag_sq.sqrt().ln().ln() / 2.0_f64.ln());
                        break;
                    }
                }

                if iter == max_iter {
                    vec![0, 0, 0]
                } else {
                    let hue = 200.0 + (smooth * 2.0) % 120.0;

                    let saturation = 0.6 + (smooth * 0.1).sin().abs() * 0.3;
                    let lightness = 0.2 + (smooth * 0.05).cos().abs() * 0.4;

                    let (r, g, b) = hsl_to_rgb(hue, saturation, lightness);
                    vec![(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]
                }
            })
            .collect();

        writer.write_all(&row_pixels)?;
        if y % 100 == 0 && y > 0 {
            let elapsed = start_time.elapsed().as_secs_f64();
            let progress = y as f64 / height as f64;
            let remaining_time = (elapsed / progress) - elapsed;

            let hours = (remaining_time / 3600.0) as u32;
            let minutes = ((remaining_time % 3600.0) / 60.0) as u32;
            let seconds = (remaining_time % 60.0) as u32;

            print!(
                "\r[Progress: {:.2}%] | ETA: {:02}:{:02}:{:02} | Elapsed: {:.0}s   ",
                progress * 100.0,
                hours,
                minutes,
                seconds,
                elapsed
            );
            std::io::stdout().flush().unwrap();
        }
    }

    writer.flush()?;
    println!("\nDone!");
    Ok(())
}
