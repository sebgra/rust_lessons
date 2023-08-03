fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Let if version

    let config_max_version_2 = Some(3u8);
    if let Some(max) = config_max_version_2 {
        println!("The maximum is configured to be {}", max),
        _ => (),

    }

    // Let if else version

    let config_max_version_3 = Some(3u8);
    if let Some(max) = config_max_version_3 {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("The maximum is not configured");
    }


    
}
