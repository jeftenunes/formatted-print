#[derive(Debug)]
pub struct Bed {
    size: i32,
    count: u32
}

#[derive(Debug)]
pub enum Room {
    Lounge,
    Kitchen(i32),
    Bedroom(Bed)
}

fn main() {
    let e = Room::Bedroom(Bed { size: 50, count: 1 });
    println!("Hello from the {:?}", e);

    match e {
        Room::Kitchen(n) => println!("The room is a kitchen with {} rooms", n),
        d => println!("{:?}", d),
    }
}
