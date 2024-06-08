use num::Complex;
use std::str::FromStr;
use image::ExtendedColorType;
use image::ImageEncoder;
use image::codecs::png::PngEncoder;
use std::fs::File;


fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

	if args.len() != 5 {
		eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
		eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
		std::process::exit(1);
	}

	let bounds = parse_pair(&args[2], 'x').expect("parse pixels failed!");
	let upper_left = parse_complex(&args[3]).expect("parse upper left failed!");
	let lower_right = parse_complex(&args[4],).expect("parse lower right failed!");
	let mut pixels = vec![0; bounds.0 * bounds.1];
	let threads = 8;
	let rows_per_bound = bounds.1 / threads + 1;
	let chunks: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_bound * bounds.0).collect();
	crossbeam::scope(
		|spawner| {
			for (i, band) in chunks.into_iter().enumerate() {
				let top = rows_per_bound * i;
				let height = band.len() / bounds.0;
				let band_bounds = (bounds.0, height);
				let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
				let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
				spawner.spawn(move |_| {
					render(band, band_bounds, band_upper_left, band_lower_right);
				});
			}
		}
	).unwrap();
	write_image(&args[1], &pixels, bounds).expect("failed to write image");
}


fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64, // 为什么这里要用减法？这是因为在屏幕坐标系中pixel.1是
                                                                       // 向下递增的，但复数的虚部是向上递增的
    }
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>, 
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}


fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), image::error::ImageError> {
	let output = File::create(filename)?;
	let encoder = PngEncoder::new(output);
	encoder.write_image(pixels, bounds.0 as u32, bounds.1 as u32, ExtendedColorType::L8)?;
	Ok(())
}


#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.234,-39.43"),
        Some(Complex {
            re: 1.234,
            im: -39.43
        })
    );
    assert_eq!(parse_complex(",9.333"), None);
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("1.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("1.5x2.5", 'x'), Some((1.5, 2.5)));
}
