use std::error::Error;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    let input = download()?;

    let image = parse(input);

    let (min, max) = minmax(image);

    println!("{min},{max}");
    Ok(())
}

fn parse(input: String) -> Vec<Vec<u8>> {
    let mut image : Vec<Vec<u8>> = Vec::new();
    for input_line in input.trim().lines() {
        let mut line: Vec<u8> = Vec::new();
        for pixel_str in input_line.trim().split(" ") {
            let pixel = pixel_str.parse::<u8>().unwrap();
            line.push(pixel);
        }
        image.push(line);
    }
    image
}

fn download() -> Result<String, Box<dyn Error>> {
    let input = reqwest::blocking::get("https://raw.githubusercontent.com/dseres/rust_test_training/refs/heads/master/examples/image.txt")?.text()?;
    Ok(input)
}

fn minmax(image: Vec<Vec<u8>>) -> (u8, u8) {
    let min = image.iter().map(|line| line.iter().min().unwrap()).min().unwrap();
    let max = image.iter().map(|line| line.iter().max().unwrap()).min().unwrap();
    (*min, *max)
}

