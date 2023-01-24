use std::vec;
use clap::Parser;

use image::{Pixel};
use image::{ImageBuffer, Rgb, Luma};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use setvision::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Seed: random number to shuffle cards with
   #[arg(short, long)]
   seed: Option<u64>,

   /// Image path: where to load an image from?
   img_path: Option<String>
}

fn to_rgb(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(image.width(), image.height(),
        |x, y| image.get_pixel(x, y).to_rgb())
}

// #[cfg(feature = "display-window")]
fn main() {
    use imageproc::window::display_multiple_images;

    let args = Args::parse();

    let mut all_cards = generate_all_cards();

    let selected_cards: Vec<&Card> = if let Some(seed) = args.seed {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        // cards.choose_multiple(&mut rng, 12).collect()
        all_cards.shuffle(&mut rng);
        all_cards.iter().take(12).collect()
    }
    else {
        let mut rng = thread_rng();
        // cards.choose_multiple(&mut rng, 12).collect()
        all_cards.shuffle(&mut rng);
        all_cards.iter().take(12).collect()
    };

    println!("These are all the sets in this table:");
    println!("-------------------------------------");
    let sets = find_all_sets(selected_cards.to_vec());
    let solved_table = Table {
        cards: selected_cards,
        triples: sets.into(),
    };
    println!("{}", solved_table);

    if let Some(path) = args.img_path {
        let img = image::open(path).expect("No image found at provided path").to_rgb8();
        let grayscaled = image::imageops::grayscale(&img);
        let canny = imageproc::edges::canny(&grayscaled, 30.0, 50.0);
        // let opened = imageproc::morphology::close(&canny, Norm::LInf, 1);

        display_multiple_images("", &vec![
            &img,
            &to_rgb(&grayscaled),
            &to_rgb(&canny),
            // &to_rgb(&opened)
            ], 500, 500);
    }
    
}

