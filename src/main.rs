use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use image::{self, ImageBuffer, Rgba};

fn set_avg_pixel_color(original: &ImageBuffer<Rgba<u8>, Vec<u8>>, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, start_x: u32, start_y: u32, factor: u32) -> u8 {
    let mut vec_colors: Vec<&Rgba<u8>> = Vec::new();

    for y in start_y..start_y+factor {
        for x in start_x..start_x+factor {
            match original.get_pixel_checked(x, y)  {
                Some(pixel) => {
                    vec_colors.push(pixel)
                }
                _ => {}
            }
        }
    }

    let vec_len = if vec_colors.len() == 0 {
        1
    } else {
        vec_colors.len()
    } as u32;

    let avg_pixel = (vec_colors.iter().map(|e| e.0[0]).collect::<Vec<u8>>().iter().fold(0u32, |acc, &x| acc + x as u32) / vec_len) as u8;
    let (hashmap_idx, min) = find_nearest_number(avg_pixel);

    for y in start_y..start_y+factor {
        for x in start_x..start_x+factor {
            if x < image.width() && y < image.height() {
                image.put_pixel(x, y, Rgba([avg_pixel, avg_pixel, avg_pixel, 255]));
            }
        }
    }
    hashmap_idx
}

fn find_nearest_number(color: u8) -> (u8, u8) {
    let mut dices_selector: HashMap<u8, u8> = HashMap::new();

    dices_selector.insert(1, 255 * (6 / 6));
    dices_selector.insert(2, 255 * (5 / 6));
    dices_selector.insert(3, 255 * (4 / 6));
    dices_selector.insert(4, 255 * (3 / 6));
    dices_selector.insert(5, 255 * (2 / 6));
    dices_selector.insert(6, 255 * (1 / 6));

    let mut min: u8 = u8::MAX;
    let mut hashmap_idx= 1;
    for (key, value) in dices_selector {
        let current_value: u8 = color.abs_diff(value);
        if min > current_value {
            min = current_value;
            hashmap_idx = key;
        }
    }
    (hashmap_idx, min)
}

fn print_dice(dice_value: u8) {
    let mut dices_selector: HashMap<u8, char> = HashMap::new();
    dices_selector.insert(1, '⚀');
    dices_selector.insert(2, '⚁');
    dices_selector.insert(3, '⚂');
    dices_selector.insert(4, '⚃');
    dices_selector.insert(5, '⚄');
    dices_selector.insert(6, '⚅');

    print!("{}", dices_selector.get(&dice_value).unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = args.get(1).unwrap();
    let factor = u32::from_str(&args.get(2).unwrap().as_str()).unwrap();
    let print_mode = args.get(3);
    let dice_print_mode = "dices".to_string();

    let img = image::open(image_path).unwrap().grayscale();
    let original: ImageBuffer::<Rgba<u8>, Vec<u8>> = img.to_rgba8();

    let mut buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(img.width(), img.height());

    let mut dices_sum = 0;
    for y in (0..=original.height()).step_by(factor as usize) {
        for x in (0..=original.width()).step_by(factor as usize) {
            let dice_value = set_avg_pixel_color(&original, &mut buffer, x, y, factor);
            match print_mode {
                Some(mode) if mode == &dice_print_mode => print_dice(dice_value),
                _ => print!("{dice_value}")
            }
            dices_sum += 1;
        }
        println!();
    }

    println!("Height: {}mm", buffer.height() / factor * 10);
    println!("Width: {}mm", buffer.width() / factor * 10);
    println!("Dices required: {dices_sum}");
    buffer.save("./output.png").expect("Failed to save image");
}
