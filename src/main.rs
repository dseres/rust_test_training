use rtt::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = download()?;

    let image = parse(&input);

    let (min, max) = minmax(&image);

    println!("{min},{max}");
    Ok(())
}
