use iridescent::{Styled, StyledString};

fn main() {
    let mut modes: Vec<StyledString> = Vec::new();

    let bold = "bold".bold();
    modes.push(bold);

    let dim = "dim".dim();
    modes.push(dim);

    let italic = "italic".italic();
    modes.push(italic);

    let underline = "underline".underline();
    modes.push(underline);

    let blink = "blink".blink();
    modes.push(blink);

    let invert = "invert".invert();
    modes.push(invert);

    let hidden = "hidden".hidden();
    modes.push(hidden);

    let strike = "strike".strike();
    modes.push(strike);

    // Print all the examples
    println!("{}", modes.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));
}
