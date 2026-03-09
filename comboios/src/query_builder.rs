/// Builder for station search queries
#[derive(Debug, Clone, Default)]
pub struct StationQuery {
    name: Option<String>,
    limit: Option<usize>,
}

impl StationQuery {
    /// Create a new station query builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the station name to search for
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Limit the number of results
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Build the query string
    pub fn build(&self) -> String {
        self.name.clone().unwrap_or_default()
    }

    /// Get the limit
    pub fn get_limit(&self) -> Option<usize> {
        self.limit
    }
}

/// Builder for timetable filters
#[derive(Debug, Clone, Default)]
pub struct TimetableFilter {
    train_type: Option<String>,
    platform: Option<String>,
}

impl TimetableFilter {
    /// Create a new timetable filter builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by train type (e.g., "ALFA", "IC", "R")
    pub fn train_type(mut self, train_type: impl Into<String>) -> Self {
        self.train_type = Some(train_type.into());
        self
    }

    /// Filter by platform
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    /// Check if a timetable entry matches this filter
    pub fn matches(&self, entry: &crate::domain::station_timetable::Timetable) -> bool {
        if let Some(ref train_type) = self.train_type {
            let train_str = entry.train_number.to_string();
            if !train_str.starts_with(train_type) {
                return false;
            }
        }

        if let Some(ref platform) = self.platform
            && entry.platform.as_ref() != Some(platform)
        {
            return false;
        }

        true
    }
}
