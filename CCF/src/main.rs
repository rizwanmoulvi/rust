//if let syntax


fn main() {
    let config_max = Some(8u8);
    match config_max {
        Some(max) => println!("The maximum is set to be {}", max),
        _ => (),
    }

    let config_min = Some(3u8);
    if let Some(min) = config_min {
        println!("The minimum is set to be {}", min);
    } else {
        println!("Failed to set minimum");
    }
}