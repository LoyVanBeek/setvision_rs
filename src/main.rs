use std::{fmt, vec};

extern crate ansi_colors;
use ansi_colors::*;
use std::slice::Iter;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Count {
    One,
    Two,
    Three,
}

impl Count {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

fn all_different_color(a: &Card, b: &Card, c: &Card) -> bool {
    a.color != b.color && b.color != c.color && c.color != a.color
}

fn all_same_color(a: &Card, b: &Card, c: &Card) -> bool {
    a.color == b.color && b.color == c.color && c.color == a.color
}

fn all_different_shape(a: &Card, b: &Card, c: &Card) -> bool {
    a.shape != b.shape && b.shape != c.shape && c.shape != a.shape
}

fn all_same_shape(a: &Card, b: &Card, c: &Card) -> bool {
    a.shape == b.shape && b.shape == c.shape && c.shape == a.shape
}

fn all_different_count(a: &Card, b: &Card, c: &Card) -> bool {
    a.count != b.count && b.count != c.count && c.count != a.count
}

fn all_same_count(a: &Card, b: &Card, c: &Card) -> bool {
    a.count == b.count && b.count == c.count && c.count == a.count
}

fn all_different_shading(a: &Card, b: &Card, c: &Card) -> bool {
    a.shading != b.shading && b.shading != c.shading && c.shading != a.shading
}

fn all_same_shading(a: &Card, b: &Card, c: &Card) -> bool {
    a.shading == b.shading && b.shading == c.shading && c.shading == a.shading
}

fn is_set(a: &Card, b: &Card, c: &Card) -> bool {
    let color_same_or_diff = all_same_color(a, b, c) || all_different_color(a, b, c);
    let shape_same_or_diff = all_same_shape(a, b, c) || all_different_shape(a, b, c);
    let count_same_or_diff = all_same_count(a, b, c) || all_different_count(a, b, c);
    let shading_same_or_diff = all_same_shading(a, b, c) || all_different_shading(a, b, c);

    color_same_or_diff && shape_same_or_diff && count_same_or_diff && shading_same_or_diff
}

fn find_set(cards: Vec<&Card>) -> (&Card, &Card, &Card) {
    for triplet in combinations::Combinations::new(cards, 3) {
        if is_set(triplet[0], triplet[1], triplet[2]) {
            return (triplet[0], triplet[1], triplet[2]);
        }
    }
    todo!()
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
        assert_eq!(all_same_color(&K1, &K2, &K3), true);
    }

    #[test]
    fn test_different_color_false() {
        assert_eq!(all_different_color(&K1, &K2, &K3), false);
    }

    #[test]
    fn test_same_color_false() {
        assert_eq!(all_same_color(&K6, &K7, &K8), false);
    }

    #[test]
    fn test_different_color_true() {
        assert_eq!(all_different_color(&K6, &K7, &K8), true);
    }

    #[test]
    fn test_find_all_sets_1() {
        let all_cards: Vec<&Card> = vec![
            &C1, &C2, &C3, &C4, &K1, &K2, &K3, &K4, &K5, &K6, &K7, &K8, &K9, &K10, &K11, &K12,
        ];

        let set = find_set(all_cards);
        println!("Found a set: {:#?}", set);
        // assert!(set, (&C1, &C2, &C3))
    }

    #[test]
    fn test_is_set_1() {
        assert_eq!(is_set(&C1, &C2, &C3), true);
    }
}

fn main() {
    let mut all_cards = generate_all_cards();
    // fastrand::shuffle(&mut all_cards);

    // let card_iterator = all_cards.into_iter();
    // let table: Vec<&Card> = card_iterator.take(12).collect();
    // println!("{}, {}, {}", table[0], table[1], table[2]);
    // println!("{}, {}, {}", table[3], table[4], table[5]);
    // println!("{}, {}, {}", table[6], table[7], table[8]);
    // println!("{}, {}, {}", table[9], table[10], table[11]);
    // let set = find_set(table);
    // println!("{}, {}, {}", set.0, set.1, set.2);
}
