fn main() {
    let result = drives::get_devices();
    for device in result.unwrap() {
        for partition in device.partitions {
            println!(
                "{} : {}",
                partition.name,
                partition.size.as_human_readable_string()
            );
        }
    }
}
