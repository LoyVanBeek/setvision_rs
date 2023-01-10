use std::{fmt, vec};
use clap::Parser;

extern crate ansi_colors;
use ansi_colors::*;
use image::{DynamicImage, Pixel};
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Rgb, Luma};
use imageproc::definitions::Image;
use std::slice::Iter;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Color {
    Red,
    Green,
    Purple,
}

impl Color {
    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Purple];
        COLORS.iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Count {
    One,
    Two,
    Three,
}

impl Count {
    #[allow(dead_code)]
    const fn from_int(count: u8) -> Self {
        match count {
            1 => Count::One,
            2 => Count::Two,
            3 => Count::Three,
            _ => {
                panic!("Cannot handle counts other than 1,2,3")
            }
        }
    }

    pub fn iterator() -> Iter<'static, Count> {
        static COUNTS: [Count; 3] = [Count::One, Count::Two, Count::Three];
        COUNTS.iter()
    }
}

impl Into<usize> for Count {
    fn into(self) -> usize {
        match self {
            Count::One => 1,
            Count::Two => 2,
            Count::Three => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Shading {
    Open,
    Solid,
    Striped,
}

impl Shading {
    pub fn iterator() -> Iter<'static, Shading> {
        static SHADINGS: [Shading; 3] = [Shading::Open, Shading::Solid, Shading::Striped];
        SHADINGS.iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Shape {
    Diamond,
    Oval,
    Squiggle,
}

impl fmt::Display for Shape {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let character = match self {
            Shape::Diamond => "^",
            Shape::Oval => "0",
            Shape::Squiggle => "~",
        };
        write!(f, "{}", character)
    }
}

impl Shape {
    pub fn iterator() -> Iter<'static, Shape> {
        static SHAPES: [Shape; 3] = [Shape::Diamond, Shape::Oval, Shape::Squiggle];
        SHAPES.iter()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Card {
    color: Color,
    count: Count,
    shading: Shading,
    shape: Shape,
}

impl fmt::Display for Card {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let shape_chr = format!("{}", self.shape).repeat(self.count.into());
        let padded = format!("{: ^3}", shape_chr);
        let mut repr = ColouredStr::new(padded.as_str());
        repr.bold();
        match self.color {
            Color::Red => match self.shading {
                Shading::Open => {
                    repr.red();
                },
                Shading::Solid => {
                    repr.black();
                    repr.back_red();
                },
                Shading::Striped => {
                    repr.red();
                    repr.underline();
                },
            },
            Color::Green => match self.shading {
                Shading::Open => {
                    repr.green();
                },
                Shading::Solid => {
                    repr.black();
                    repr.back_green();
                },
                Shading::Striped => {
                    repr.green();
                    repr.underline();
                },
            },
            Color::Purple => match self.shading {
                Shading::Open => {
                    repr.magenta();
                },
                Shading::Solid => {
                    repr.black();
                    repr.back_magenta();
                },
                Shading::Striped => {
                    repr.magenta();
                    repr.underline();
                },
            },
        };
        write!(f, "[{repr}]")
    }
}

struct HighlightedCard<'a> {
    card: &'a Card,
}

impl fmt::Display for HighlightedCard<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shape_chr = format!("{}", self.card.shape).repeat(self.card.count.into());        
        let padded = format!("{: ^3}", shape_chr);
        let mut repr = ColouredStr::new(padded.as_str());
        repr.bold();
        match self.card.color {
            Color::Red => match self.card.shading {
                Shading::Open => {
                    repr.red();
                },
                Shading::Solid => {
                    repr.black();
                    repr.back_red();
                },
                Shading::Striped => {
                    repr.red();
                    repr.underline();
                },
            },
            Color::Green => match self.card.shading {
                Shading::Open => {
                    repr.green();
                },
                Shading::Solid => {
                    repr.black();
                    repr.back_green();
                },
                Shading::Striped => {
                    repr.green();
                    repr.underline();
                },
            },
            Color::Purple => match self.card.shading {
                Shading::Open => {
                    repr.magenta();
                },
                Shading::Solid => {
                    repr.black();
                    repr.back_magenta();
                },
                Shading::Striped => {
                    repr.magenta();
                    repr.underline();
                },
            },
        };
        let mut colored_start_bracket = ColouredStr::new("[");
        colored_start_bracket.black();
        colored_start_bracket.back_white();
        let mut colored_end_bracket = ColouredStr::new("]");
        colored_end_bracket.black();
        colored_end_bracket.back_white();
        write!(f, "{colored_start_bracket}{repr}{colored_end_bracket}")
    }
}

#[derive(Debug)]
struct Triple<'a>(&'a Card, &'a Card, &'a Card);

impl Triple<'_> {
    fn all_different_color(&self) -> bool {
        self.0.color != self.1.color && self.1.color != self.2.color && self.2.color != self.0.color
    }

    fn all_same_color(&self) -> bool {
        self.0.color == self.1.color && self.1.color == self.2.color && self.2.color == self.0.color
    }

    fn all_different_shape(&self) -> bool {
        self.0.shape != self.1.shape && self.1.shape != self.2.shape && self.2.shape != self.0.shape
    }

    fn all_same_shape(&self) -> bool {
        self.0.shape == self.1.shape && self.1.shape == self.2.shape && self.2.shape == self.0.shape
    }

    fn all_different_count(&self) -> bool {
        self.0.count != self.1.count && self.1.count != self.2.count && self.2.count != self.0.count
    }

    fn all_same_count(&self) -> bool {
        self.0.count == self.1.count && self.1.count == self.2.count && self.2.count == self.0.count
    }

    fn all_different_shading(&self) -> bool {
        self.0.shading != self.1.shading && self.1.shading != self.2.shading && self.2.shading != self.0.shading
    }

    fn all_same_shading(&self) -> bool {
        self.0.shading == self.1.shading && self.1.shading == self.2.shading && self.2.shading == self.0.shading
    }

    fn is_set(&self) -> bool {
        let color_same_or_diff = self.all_same_color() || self.all_different_color();
        let shape_same_or_diff = self.all_same_shape() || self.all_different_shape();
        let count_same_or_diff = self.all_same_count() || self.all_different_count();
        let shading_same_or_diff = self.all_same_shading() || self.all_different_shading();

        color_same_or_diff && shape_same_or_diff && count_same_or_diff && shading_same_or_diff
    }
}

#[derive(Debug)]
struct Table<'a> {
    cards: Vec<&'a Card>,
    triples: Vec<Triple<'a>>,
}

impl Table<'_> {
    
}

#[derive(Debug, Clone)]
struct SetError;

fn find_set(cards: Vec<&Card>) -> Result<Triple, SetError> {
    for subset in combinations::Combinations::new(cards, 3) {
        let triple = Triple(
            subset[0],
            subset[1], 
            subset[2],
        );
        if triple.is_set() {
            return Ok(triple);
        }
    }
    return Err(SetError);
}

fn find_all_sets(cards: Vec<&Card>) -> Vec<Triple> {
    let mut sets: Vec<Triple> = vec![];
    for subset in combinations::Combinations::new(cards, 3) {
        let triple = Triple(
            subset[0],
            subset[1],
            subset[2],
        );
        if triple.is_set() {
            sets.push(triple);
        }
    }
    sets
}

fn generate_all_cards() -> Vec<Card> {
    let mut all_cards: Vec<Card> = vec![];
    for _color in Color::iterator() {
        for _count in Count::iterator() {
            for _shading in Shading::iterator() {
                for _shape in Shape::iterator() {
                    let card = Card {
                        color: *_color,
                        count: *_count,
                        shading: *_shading,
                        shape: *_shape,
                    };
                    all_cards.push(card);
                }
            }
        }
    }
    all_cards
}

impl fmt::Display for Table<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // A table should always have a multiple of 3 cards
        let remainder = self.cards.len() % 3;
        assert!(remainder == 0);
        let row_length = self.cards.len() / 3;

        // let self_iter = self.cards.iter();

        for triple in &self.triples {
            for x in 0..3 {
                for y in 0..row_length{
                    let index = (x * row_length) + y;
                    let card = self.cards[index];
                    if card == triple.0 || card == triple.1 || card == triple.2
                    {
                        write!(f, "{}", HighlightedCard{card: card}).unwrap();
                    }
                    else {
                        write!(f, "{}", card).unwrap();
                    }
                }
                write!(f, "\n").unwrap();
            }
            write!(f, "--------------------\n").unwrap();
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::collections::HashSet;

    // c1,2,3 together form a set, c1,2,4 do not
    const C1: Card = Card {
        color: Color::Green,
        count: Count::from_int(1),
        shading: Shading::Solid,
        shape: Shape::Squiggle,
    };
    const C2: Card = Card {
        color: Color::Green,
        shape: Shape::Oval,
        shading: Shading::Open,
        count: Count::from_int(2),
    };
    const C3: Card = Card {
        color: Color::Green,
        shape: Shape::Diamond,
        shading: Shading::Striped,
        count: Count::from_int(3),
    };
    const C4: Card = Card {
        color: Color::Green,
        shape: Shape::Diamond,
        shading: Shading::Solid,
        count: Count::from_int(3),
    };
    const K1: Card = Card {
        color: Color::Purple,
        shape: Shape::Squiggle,
        shading: Shading::Solid,
        count: Count::from_int(1),
    };
    const K2: Card = Card {
        color: Color::Purple,
        shape: Shape::Squiggle,
        shading: Shading::Striped,
        count: Count::from_int(2),
    };
    const K3: Card = Card {
        color: Color::Purple,
        shape: Shape::Oval,
        shading: Shading::Open,
        count: Count::from_int(3),
    };
    const K4: Card = Card {
        color: Color::Green,
        shape: Shape::Squiggle,
        shading: Shading::Open,
        count: Count::from_int(1),
    };
    const K5: Card = Card {
        color: Color::Purple,
        shape: Shape::Squiggle,
        shading: Shading::Open,
        count: Count::from_int(3),
    };
    const K6: Card = Card {
        color: Color::Green,
        shape: Shape::Diamond,
        shading: Shading::Solid,
        count: Count::from_int(2),
    };
    const K7: Card = Card {
        color: Color::Purple,
        shape: Shape::Diamond,
        shading: Shading::Solid,
        count: Count::from_int(3),
    };
    const K8: Card = Card {
        color: Color::Red,
        shape: Shape::Oval,
        shading: Shading::Open,
        count: Count::from_int(1),
    };
    const K9: Card = Card {
        color: Color::Red,
        shape: Shape::Oval,
        shading: Shading::Open,
        count: Count::from_int(2),
    };
    const K10: Card = Card {
        color: Color::Red,
        shape: Shape::Diamond,
        shading: Shading::Open,
        count: Count::from_int(3),
    };
    const K11: Card = Card {
        color: Color::Green,
        shape: Shape::Oval,
        shading: Shading::Open,
        count: Count::from_int(2),
    };
    const K12: Card = Card {
        color: Color::Purple,
        shape: Shape::Diamond,
        shading: Shading::Solid,
        count: Count::from_int(1),
    };

    #[test]
    fn test_cards_equal() {
        let a = Card {
            color: Color::Green,
            count: Count::from_int(1),
            shading: Shading::Solid,
            shape: Shape::Squiggle,
        };
        let b = a.clone();

        let c = Card {
            color: Color::Green,
            count: Count::from_int(1),
            shading: Shading::Solid,
            shape: Shape::Squiggle,
        };

        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(c, a);
    }

    #[test]
    fn test_hashset_keeps_only_unique() {
        let a = Card {
            color: Color::Green,
            count: Count::from_int(1),
            shading: Shading::Solid,
            shape: Shape::Squiggle,
        };
        let b = a.clone();

        let c = Card {
            color: Color::Green,
            count: Count::from_int(1),
            shading: Shading::Solid,
            shape: Shape::Squiggle,
        };

        let mut set = HashSet::new();
        set.insert(a);
        assert_eq!(set.len(), 1);
        set.insert(b);
        // b is the same thing as a, set should keep only unique things, so set doesn't increase in size
        assert_eq!(set.len(), 1);
        set.insert(c);
        assert_eq!(set.len(), 1);

        let collection = vec![a, b, c];
        assert_eq!(collection.len(), 3);
        let mut set2 = HashSet::new();
        set2.extend(collection);
        assert_eq!(set2.len(), 1);

        let collection = vec![&a, &b, &c];
        assert_eq!(collection.len(), 3);
        let mut set3: HashSet<&Card> = HashSet::new();
        set3.extend(collection.iter());
        assert_eq!(set3.len(), 1);
    }

    #[test]
    fn test_same_color_true() {
        assert_eq!(Triple(&K1, &K2, &K3).all_same_color(), true);
    }

    #[test]
    fn test_different_color_false() {
        assert_eq!(Triple(&K1, &K2, &K3).all_different_color(), false);
    }

    #[test]
    fn test_same_color_false() {
        assert_eq!(Triple(&K6, &K7, &K8).all_same_color(), false);
    }

    #[test]
    fn test_different_color_true() {
        assert_eq!(Triple(&K6, &K7, &K8).all_different_color(), true);
    }

    #[test]
    fn test_find_all_sets_1() {
        let all_cards: Vec<&Card> = vec![
            &C1, &C2, &C3, &C4, &K1, &K2, &K3, &K4, &K5, &K6, &K7, &K8, &K9, &K10, &K11, &K12,
        ];

        let set = find_set(all_cards);
        println!("Found a set: {:#?}", set);
    }

    #[test]
    fn test_is_set_1() {
        assert_eq!(Triple(&C1, &C2, &C3).is_set(), true);
    }

    #[test]
    fn test_generate_all_cards() {
        let cards = generate_all_cards();
        assert_eq!(81, cards.len());

        let mut set = HashSet::new();
        set.extend(cards);
        assert_eq!(set.len(), 81);
    }
}

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
        display_multiple_images("", &vec![&img, &to_rgb(&grayscaled), &to_rgb(&canny)], 500, 500);
    }
    
}
