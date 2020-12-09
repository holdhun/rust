#[allow(dead_code)]
enum Color{
    White,
    Red,
    Green,
    Black,
}

struct Point{
    x:i64,y:i64,
}

fn main() {
    println!("match !");
    // simple example
    let mycolor1 = Color::Red;
    match mycolor1{
        Color::Red => println!("red"),
        Color::White | Color::Black => println!("white or black"),
        _ => println!("green"),
    };

    let point = Point{x:0,y:0};
    match  point {
        Point{x,y} => println!("({},{})",x,y),
    }

    match point {
        Point { x: x1, y: y1} => println!("({},{})", x1, y1),
    }

    let tuple:(u32,String) = (5, String::from("jkasdh"));
    //let (x,s)=tuple;
    //println!("Tuple is: {:?}", tuple);
    

}