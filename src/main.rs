use std::{fmt, vec};

extern crate ansi_colors;
use ansi_colors::*;
use std::slice::Iter;
use rand::seq::SliceRandom;
use rand::thread_rng;
// use rand::SeedableRng;
// use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        let mut repr = ColouredStr::new(shape_chr.as_str());
        repr.bold();
        match self.color {
            Color::Red => repr.red(),
            Color::Green => repr.green(),
            Color::Purple => repr.magenta(),
        };
        write!(f, "[{repr}]")
    }
}

struct HighlightedCard<'a> {
    card: &'a Card,
}

impl fmt::Display for HighlightedCard<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let card_repr = format!("{}", self.card);
        let mut repr = ColouredStr::new(card_repr.as_str());
        repr.back_white();
        write!(f, "[{repr}]")
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
struct Table<'a>(Vec<&'a Card>);

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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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
    }
}

fn main() {
    let cards = generate_all_cards();
    // let all_card_refs: Vec<&Card> = cards.iter().collect();

    let mut rng = thread_rng();//ChaCha8Rng::seed_from_u64(9);//
    let table: Vec<&Card> = cards.choose_multiple(&mut rng, 12).collect();

    let mut table_set = HashSet::with_capacity(12);
    for c in table.iter() {
        table_set.insert(*c);
    }
    println!("There are {} unique cards on the table", table_set.len());
    println!("{}, {}, {}", table[0], table[1], table[2]);
    println!("{}, {}, {}", table[3], table[4], table[5]);
    println!("{}, {}, {}", table[6], table[7], table[8]);
    println!("{}, {}, {}", table[9], table[10], table[11]);

    let set_result = find_set(table.to_vec());
    match set_result {
        Ok(set) => {
            println!("These form a set:");
            println!("{}, {}, {}", set.0, set.1, set.2);
        },
        Err(_) => println!("No set found"),
    }

    println!("These are all the sets in this table:");
    println!("-------------------------------------");
    let sets = find_all_sets(table.to_vec());
    for set in sets {
        println!("{}, {}, {}", set.0, set.1, set.2);
    }
}
