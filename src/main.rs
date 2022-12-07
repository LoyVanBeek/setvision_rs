use std::fmt;

extern crate ansi_colors;
use ansi_colors::*;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Purple,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
enum Shading {
    Open,
    Solid,
    Striped,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
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
        let prefix = match self.color {
            Color::Red => repr.red(),
            Color::Green => repr.green(),
            Color::Purple => repr.magenta(),
        };
        write!(f, "[{repr}]")
    }
}

// TODO: Could be Traits?
fn all_different_color(a: &Card, b: &Card, c: &Card) -> bool {
    a.color != b.color && b.color != c.color && c.color != a.color
}

fn all_same_color(a: &Card, b: &Card, c: &Card) -> bool {
    a.color == b.color && b.color == c.color && c.color == a.color
}

fn is_set(a: &Card, b: &Card, c: &Card) -> bool {
    todo!();
}

fn find_set(cards: Vec<&Card>) -> (&Card, &Card, &Card) {
    todo!();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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
    // c1,2,3 together form a set, c1,2,4 do not

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
}

fn main() {

    // let all_cards: Vec<&Card> = vec![
    //     &c1, &c2, &c3, &c4, &k1, &k2, &k3, &k4, &k5, &k6, &k7, &k8, &k9, &k10, &k11, &k12,
    // ];

    // let set = find_set(all_cards);
    // // println!("Found a set: {:#?}", set);
}
