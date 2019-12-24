use std::collections::HashMap;

pub struct KyaniteUtility;

impl KyaniteUtility {
    pub fn get_sizes() -> HashMap<&'static str, &'static str> {
        let mut sizes = HashMap::new();
        sizes.insert("0", "B");
        sizes.insert("1", "KiB");
        sizes.insert("2", "MiB");
        sizes.insert("3", "GiB");
        sizes.insert("4", "TiB");
        sizes.insert("5", "PiB");
        sizes
    }

    pub fn human_size(size: f64) -> String {
        let sizes = Self::get_sizes();
        let mut power = 0f64;
        let mut metric = "B";
        for key in sizes.keys() {
            power = key.parse::<f64>().unwrap();
            metric = sizes.get(key).unwrap();
            if size < 1024f64.powf(power) {
                break;
            }
        }
        format!("{:.2} {}", size / 1024f64.powf(power), metric)
    }
}
