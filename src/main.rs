#[derive(Debug)]
enum Color {
    Red,
    Green,
    Purple,
}


#[derive(Debug)]
enum Count {
    One,
    Two,
    Three,
}

#[derive(Debug)]
enum Shading {
    Open,
    Solid,
    Striped,
}

#[derive(Debug)]
enum Shape {
    Diamond,
    Oval,
    Squiggle,
}

#[derive(Debug)]
struct Card {
    color: Color,
    count: Count,
    shading: Shading,
    shape: Shape,
}


fn main() {
    let a = Card{
        color: Color::Purple,
        count: Count::One,
        shading: Shading::Open,
        shape: Shape::Diamond,
    };

    println!("Hello, Set: {:?}", a);
}
