use std::error::Error;

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut image: Vec<Vec<u8>> = Vec::new();
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

pub fn download() -> Result<String, Box<dyn Error>> {
    let input = reqwest::blocking::get("https://raw.githubusercontent.com/dseres/rust_test_training/refs/heads/master/examples/image.txt")?.text()?;
    Ok(input)
}

pub fn minmax(image: &[Vec<u8>]) -> (u8, u8) {
    let min = image
        .iter()
        .map(|line| line.iter().min().unwrap())
        .min()
        .unwrap();
    let max = image
        .iter()
        .map(|line| line.iter().max().unwrap())
        .max()
        .unwrap();
    (*min, *max)
}

#[cfg(test)]
mod test {

    static TXT: &str = "92 213 101\n224 197 198\n34 230 56";

    #[test]
    fn test_parse() {
        let image: Vec<Vec<u8>> = Vec::from([
            Vec::from([92_u8, 213_u8, 101_u8]),
            Vec::from([224_u8, 197_u8, 198_u8]),
            Vec::from([34_u8, 230_u8, 56_u8]),
        ]);
        assert_eq!(crate::parse(TXT), image)
    }

    #[test]
    fn test_minmax() {
        let image = crate::parse(TXT);
        assert_eq!(
            crate::minmax(&image),
            (34_u8, 230_u8),
            "Min and max should be 34 and 230 on the predefined matrix."
        )
    }

    #[test]
    #[ignore = "Unimplemented yet"]
    fn test_download() {
        unimplemented!()
    }
}
