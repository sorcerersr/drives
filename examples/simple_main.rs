fn main() {
    let result = drives::get_devices();
    for device in result.unwrap() {
        println!("{:?}", device);
    }
}
