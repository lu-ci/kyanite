#[derive(Clone, Debug, Default)]
pub struct StatsContainer {
    pub ok: u64,
    pub failed: u64,
    pub skipped: u64,
    pub inherited: u64,
}

impl StatsContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn describe(&self) -> String {
        format!(
            "Ok: {} | Err: {} | Skp: {} | Cpd: {}",
            &self.ok, &self.failed, &self.skipped, &self.inherited
        )
    }

    pub fn count(&self) -> u64 {
        self.ok + self.failed + self.skipped + self.inherited
    }

    pub fn add_ok(&mut self) -> &'static str {
        self.ok += 1;
        "DOWNLOADED"
    }

    pub fn add_failed(&mut self) -> &'static str {
        self.failed += 1;
        "FAILED"
    }

    pub fn add_skipped(&mut self) -> &'static str {
        self.skipped += 1;
        "SKIPPED"
    }

    pub fn add_inherited(&mut self) -> &'static str {
        self.inherited += 1;
        "INHERITED"
    }
}
