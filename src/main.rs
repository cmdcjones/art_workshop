use std::{io, process};

trait Draw {}
trait Paint {} 
trait Spray {}

#[derive(Debug)]
enum Implement {
    Pen,
    Pencil,
    Brush,
    SprayCan,
}

#[derive(Debug)]
enum Surface {
    Canvas,
    Paper,
    Wood,
    Glass,
}

#[derive(Debug)]
struct Artwork {
    art: String,
    price: u32,
}

impl Artwork {
    fn art(&self) -> &str {
        &self.art
    }
    fn price(&self) -> u32 {
        self.price
    }
}

impl Surface {
    fn draw(implement: impl Draw, surface: Surface) {}
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let implement_selection = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();
    let implement = match select_implement(implement_selection) {
        Ok(imp) => imp,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    println!("{:?}", implement);
}

fn select_implement(implement_num: usize) -> Result<Implement, String> {
    let implement = match implement_num {
        1 => Implement::Pen,
        2 => Implement::Pencil,
        3 => Implement::Brush,
        4 => Implement::SprayCan,
        _ => return Err(String::from("Not a valid implement selection!")),
    };
    Ok(implement)
}
