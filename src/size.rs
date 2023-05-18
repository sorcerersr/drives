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
    fn most_suitable_unit_for_raw_size(size: u64) -> Unit {
        if size < 2048 {
            Unit::KiloByte
        } else if size < 2097152 {
            Unit::MegaByte
        } else if size < 2147483648 {
            Unit::GigaByte
        } else {
            Unit::TeraByte
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

    pub fn get_size_in_unit(&self, unit: &Unit) -> f64 {
        let size = self.raw_size as f64 / unit.conversion_factor() as f64;
        // round to have max two decimal digits
        (size * 100.0).round() / 100.0
    }

    pub fn as_human_readable_string(&self) -> String {
        let unit = Unit::most_suitable_unit_for_raw_size(self.raw_size);
        let converted_size = self.get_size_in_unit(&unit);
        format!("{} {}", converted_size, unit.short_name())
    }
}

#[cfg(test)]
mod tests {
    use super::Size;
    use super::Unit;

    #[test]
    fn test_get_raw_size() {
        let size = Size::new(12345);
        assert_eq!(size.get_raw_size(), 12345);
    }

    #[test]
    fn test_get_size_in_unit() {
        let mut size = Size::new(8192);
        assert_eq!(size.get_size_in_unit(&Unit::KiloByte), 4096.0);
        assert_eq!(size.get_size_in_unit(&Unit::MegaByte), 4.0);

        size = Size::new(1050624);
        assert_eq!(size.get_size_in_unit(&Unit::MegaByte), 513.0);
        assert_eq!(size.get_size_in_unit(&Unit::GigaByte), 0.5);

        size = Size::new(999162511);
        assert_eq!(size.get_size_in_unit(&Unit::MegaByte), 487872.32);
        assert_eq!(size.get_size_in_unit(&Unit::GigaByte), 476.44);
        assert_eq!(size.get_size_in_unit(&Unit::TeraByte), 0.47);
    }

    #[test]
    fn test_as_human_readable_string() {
        let mut size = Size::new(8192);
        assert_eq!(size.as_human_readable_string(), "4 MB");

        size = Size::new(6532);
        assert_eq!(size.as_human_readable_string(), "3.19 MB");

        size = Size::new(1500);
        assert_eq!(size.as_human_readable_string(), "750 KB");

        size = Size::new(999162511);
        assert_eq!(size.as_human_readable_string(), "476.44 GB");

        size = Size::new(1050624);
        assert_eq!(size.as_human_readable_string(), "513 MB");
    }
}
