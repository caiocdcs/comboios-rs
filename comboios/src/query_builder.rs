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

/// Builder for train entry filters
#[derive(Debug, Clone, Default)]
pub struct TrainEntryFilter {
    service_type: Option<String>,
    origin_station: Option<String>,
    destination_station: Option<String>,
}

impl TrainEntryFilter {
    /// Create a new train entry filter builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by service type (e.g., "ALFA", "IC", "IR", "R")
    pub fn service_type(mut self, service_type: impl Into<String>) -> Self {
        self.service_type = Some(service_type.into());
        self
    }

    /// Filter by origin station name
    pub fn origin_station(mut self, station: impl Into<String>) -> Self {
        self.origin_station = Some(station.into());
        self
    }

    /// Filter by destination station name
    pub fn destination_station(mut self, station: impl Into<String>) -> Self {
        self.destination_station = Some(station.into());
        self
    }

    /// Check if a train entry matches this filter (case-insensitive)
    pub fn matches(&self, entry: &crate::domain::station_timetable::TrainEntry) -> bool {
        if let Some(ref service) = self.service_type
            && entry.service_type.to_uppercase() != service.to_uppercase()
        {
            return false;
        }

        if let Some(ref origin) = self.origin_station
            && !entry
                .origin_station_name
                .to_uppercase()
                .contains(&origin.to_uppercase())
        {
            return false;
        }

        if let Some(ref dest) = self.destination_station
            && !entry
                .destination_station_name
                .to_uppercase()
                .contains(&dest.to_uppercase())
        {
            return false;
        }

        true
    }
}

/// Deprecated: Old filter for Timetable struct
#[deprecated(since = "0.2.0", note = "Use TrainEntryFilter instead")]
#[derive(Debug, Clone, Default)]
pub struct TimetableFilter {
    train_type: Option<String>,
    platform: Option<String>,
}

#[allow(deprecated)]
impl TimetableFilter {
    /// Create a new timetable filter builder
    #[deprecated(since = "0.2.0", note = "Use TrainEntryFilter instead")]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by train type
    #[deprecated(since = "0.2.0", note = "Use TrainEntryFilter::service_type instead")]
    pub fn train_type(mut self, train_type: impl Into<String>) -> Self {
        self.train_type = Some(train_type.into());
        self
    }

    /// Filter by platform
    #[deprecated(since = "0.2.0", note = "Platform filtering no longer supported")]
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
