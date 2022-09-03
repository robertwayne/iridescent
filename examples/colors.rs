use iridescent::{constants::*, Styled, StyledString};

fn main() {
    let mut foregrounds: Vec<StyledString> = Vec::new();
    let mut backgrounds: Vec<StyledString> = Vec::new();

    // Foreground Examples
    let black = "black".foreground(BLACK).background(WHITE);
    foregrounds.push(black);

    let red = "red".foreground(RED);
    foregrounds.push(red);

    let green = "green".foreground(GREEN);
    foregrounds.push(green);

    let yellow = "yellow".foreground(YELLOW);
    foregrounds.push(yellow);

    let blue = "blue".foreground(BLUE);
    foregrounds.push(blue);

    let magenta = "magenta".foreground(MAGENTA);
    foregrounds.push(magenta);

    let cyan = "cyan".foreground(CYAN);
    foregrounds.push(cyan);

    let white = "white".foreground(WHITE);
    foregrounds.push(white);

    // Background Examples
    let bg_black = "black".background(BLACK);
    backgrounds.push(bg_black);

    let bg_red = "red".background(RED);
    backgrounds.push(bg_red);

    let bg_green = "green".background(GREEN);
    backgrounds.push(bg_green);

    let bg_yellow = "yellow".background(YELLOW);
    backgrounds.push(bg_yellow);

    let bg_blue = "blue".background(BLUE);
    backgrounds.push(bg_blue);

    let bg_magenta = "magenta".background(MAGENTA);
    backgrounds.push(bg_magenta);

    let bg_cyan = "cyan".background(CYAN);
    backgrounds.push(bg_cyan);

    let bg_white = "white".foreground(BLACK).background(WHITE);
    backgrounds.push(bg_white);

    // Print the foreground examples
    println!("{}", foregrounds.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));

    // Print the background examples
    println!("{}", backgrounds.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));
}
