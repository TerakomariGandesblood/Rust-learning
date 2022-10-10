fn func(config_max: Option<u8>) {
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("No maximum");
    }
}

fn main() {
    func(Some(3_u8));
    func(None);
}
