use crate::loader::v1::LoaderConfig;

impl LoaderConfig {
    pub fn fetchers(&self) -> Vec<i32> {
        let mut entries: Vec<_> = self.fetchers.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries.into_iter().map(|(_, v)| *v).collect()
    }

    pub fn validators(&self) -> Vec<i32> {
        let mut entries: Vec<_> = self.validators.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries.into_iter().map(|(_, v)| *v).collect()
    }
}
