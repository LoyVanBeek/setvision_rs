use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::rc::Rc;
// use core::slice::SlicePattern;
use std::vec;
use clap::Parser;

use image::{Pixel, GenericImage};
use image::{ImageBuffer, Rgb, Luma};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::SeedableRng;
use imageproc::contours::Contour;
use rand_chacha::ChaCha8Rng;

use setvision::*;

use crate::tree::{add_child, TreeNode};
mod tree;

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
        let contours: Vec<Contour<i32>> = imageproc::contours::find_contours(&canny);
        // TODO: sort these contrours. Each contour has a parent that contains it, 
        //  so make some sort of graph and start at the top for drawing

        println!("There are {} contours found", contours.len());
        let mut contour_img = ImageBuffer::new(img.width(), img.height());
        contour_img.copy_from(&img, 0, 0).unwrap();

        let mut contour_mapping = HashMap::new(); //<usize, TreeNode<Contour<i32>>>

        for (index, contour) in contours.iter().enumerate() {
            let contour_treenode = Rc::new(TreeNode::new_childless(contour));
            contour_mapping.insert(index, contour_treenode);
        }

        for child_ref in contour_mapping.values() {
            let child = child_ref.to_owned();
            match child.value.parent {
                Some(parent_index) => {
                    let parent = contour_mapping.get(&parent_index).unwrap().borrow();
                    add_child(parent, &child);
                }
                None => ()
            }
        }

        let colors = vec![ 
            Rgb([0u8, 0u8, 0u8]),
            Rgb([0u8, 0u8, 255u8]),
            Rgb([0u8, 255u8, 0u8]),
            Rgb([0u8, 255u8, 255u8]),
            Rgb([255u8, 0u8, 0u8]),
            Rgb([255u8, 0u8, 255u8]),
            Rgb([255u8, 255u8, 0u8]),
            Rgb([255u8, 255u8, 255u8])];

        let mut levels = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        for contour_node in contour_mapping.values() {
            let level = contour_node.level();
            levels[level].push(contour_node);
        }

        for level in levels {
            for contour_ref in level {
                let node = contour_ref.to_owned();
                let color = colors[node.level()];
                let contour = node.value;
                // println!("There are {} points in this contour", contour.points.len());
                if contour.points.len() > 1 {
                    let polygon = contour.points.as_slice();
                    imageproc::drawing::draw_polygon_mut(&mut contour_img, polygon, color);
                }
            }
        }

        for contour_node in contour_mapping.values() {
            if contour_node.level() == 1 {
                // These are likely already the card outlines, but better be sure:
                // Does it have 1 to 3 children?
                let child_count = contour_node.children.borrow().len();
                if child_count >= 1 && child_count <= 3 {
                    let epsilon = 30.0;
                    let points = imageproc::geometry::approximate_polygon_dp(contour_node.value.points.as_slice(), epsilon, true);

                    println!("Card candidate has approx. polygon of {} points (using eps. {})", points.len(), epsilon);
                    if points.len() == 4 {
                        imageproc::drawing::draw_polygon_mut(&mut contour_img, points.as_slice(), colors[0]);
                    }
                    // imageproc::drawing::draw_polygon_mut(&mut contour_img, contour_node.value.points.as_slice(), colors[0]);
                }
            }
        }
        
        display_multiple_images("", &vec![
            &img,
            &to_rgb(&grayscaled),
            &to_rgb(&canny),
            // &to_rgb(&opened)
            &contour_img,
            ], 500, 500);
    }
    
}

