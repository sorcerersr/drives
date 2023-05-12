#[derive(Debug)]
pub struct Size {
    raw_size: u64,
}

pub enum Unit {
    Blocks,
    KiloByte,
    MegaByte,
    GigaByte,
    TeraByte,
}

impl Unit {
    fn short_name(&self) -> &str {
        match *self {
            Unit::Blocks => "Blocks",
            Unit::KiloByte => "KB",
            Unit::MegaByte => "MB",
            Unit::GigaByte => "GB",
            Unit::TeraByte => "TB",
        }
    }
    fn conversion_factor(&self) -> u32 {
        match *self {
            Unit::Blocks => 1,
            Unit::KiloByte => 2,
            Unit::MegaByte => 2048,
            Unit::GigaByte => 2097152,
            Unit::TeraByte => 2147483648,
        }
    }
}

impl Size {
    pub fn new(raw_size: u64) -> Size {
        Size { raw_size }
    }

    pub fn get_raw_size(&self) -> u64 {
        self.raw_size
    }

    pub fn get_size_in_unit(&self, unit: Unit) -> f64 {
        self.raw_size.) / unit.conversion_factor()
    }
}

#[cfg(test)]
mod tests {
    use super::Size;

    #[test]
    fn test_get_raw_size() {
        let size = Size::new(12345);
        assert_eq!(size.get_raw_size(), 12345);
    }
}
