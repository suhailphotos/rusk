use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Implement FromStr so Point can be built from a &str
impl FromStr for Point {
    type Err = String; // you can use a custom error type too

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // expected format: "x,y"
        let (x_str, y_str) = s.split_once(',')
            .ok_or_else(|| "Missing comma".to_string())?;

        let x = x_str.trim().parse::<i32>()
            .map_err(|e| format!("Invalid x: {e}"))?;

        let y = y_str.trim().parse::<i32>()
            .map_err(|e| format!("Invalid y: {e}"))?;

        Ok(Point { x, y })
    }
}

fn main() {
    let p: Point = "10,20".parse().expect("Valid point");
    println!("{p:?}");

    // or explicit:
    let q = "30,40".parse::<Point>().unwrap();
    println!("{q:?}");
}
