#[cfg(feature = "random")]
fn main() {
    use iridescent::{Color, Styled, StyledString};

    let mut fg_low_bits: Vec<StyledString> = Vec::new();
    let mut bg_low_bits: Vec<StyledString> = Vec::new();
    let mut fg_high_bits: Vec<StyledString> = Vec::new();
    let mut bg_high_bits: Vec<StyledString> = Vec::new();

    for _ in 0..12 {
        let s = "rainbow".foreground(Color::random());
        fg_low_bits.push(s);
    }

    for _ in 0..12 {
        let s = "rainbow".background(Color::random());
        bg_low_bits.push(s);
    }

    for _ in 0..12 {
        let s = "rainbow".rgb_foreground(Color::random_rgb());
        fg_high_bits.push(s);
    }

    for _ in 0..12 {
        let s = "rainbow".rgb_background(Color::random_rgb());
        bg_high_bits.push(s);
    }

    // Print the foreground Xterm-256 examples
    println!("{}", fg_low_bits.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));

    // Print the background Xterm-256 examples
    println!("{}", bg_low_bits.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));

    // Print the foreground RGB examples
    println!("{}", fg_high_bits.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));

    // Print the background RGB examples
    println!("{}", bg_high_bits.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));
}

#[cfg(not(feature = "random"))]
fn main() {
    println!("This example requires the `random` feature to be enabled.\n\nTry running this command again as: cargo run --example rainbow --features random"
    );
}
