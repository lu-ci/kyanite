pub struct KyaniteUtility;

impl KyaniteUtility {
    pub fn human_size(size: f64, power: f64, metric: &'static str) -> String {
        format!("{:.2} {}", size / 1024f64.powf(power), metric)
    }

    pub fn version() -> String {
        let mut version = "0.1.0";
        let toml = include_str!("../Cargo.toml");
        for line in toml.split('\n') {
            if line.starts_with("version") {
                let pieces = line.split('"').collect::<Vec<&str>>();
                version = pieces.get(1).unwrap_or(&"0.1.0");
                break;
            }
        }
        version.to_string()
    }
}
