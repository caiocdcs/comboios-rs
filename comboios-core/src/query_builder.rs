//! Builder types for constructing API queries.
//!
//! These types offer a fluent interface for building queries before passing
//! them to the [`crate::Comboios`] client methods.

/// Builder for station search queries.
///
/// Construct a query string to pass to [`crate::Comboios::search_stations`].
///
/// # Examples
///
/// ```no_run
/// use comboios_core::query_builder::StationQuery;
///
/// let query = StationQuery::new()
///     .name("Lisboa")
///     .limit(10)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct StationQuery {
    name: Option<String>,
    limit: Option<usize>,
}

impl StationQuery {
    /// Create a new, empty station query builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the station name (or partial name) to search for.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Limit the maximum number of results to return.
    #[must_use]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Build the query string to pass to [`crate::Comboios::search_stations`].
    ///
    /// Returns an empty string when no name has been set.
    #[must_use]
    pub fn build(&self) -> String {
        self.name.clone().unwrap_or_default()
    }

    /// Return the configured result limit, if any.
    #[must_use]
    pub fn get_limit(&self) -> Option<usize> {
        self.limit
    }
}

/// Filter for [`crate::domain::station_timetable::TrainEntry`] results returned
/// by the CP timetable API.
///
/// All conditions are combined with AND semantics. Comparisons are
/// case-insensitive.
///
/// # Examples
///
/// ```no_run
/// use comboios_core::query_builder::TrainEntryFilter;
///
/// let filter = TrainEntryFilter::new()
///     .service_type("IC")
///     .destination_station("Porto");
/// ```
#[derive(Debug, Clone, Default)]
pub struct TrainEntryFilter {
    service_type: Option<String>,
    origin_station: Option<String>,
    destination_station: Option<String>,
}

impl TrainEntryFilter {
    /// Create a new, empty filter that matches all train entries.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict matches to entries with this service type (e.g. `"ALFA"`, `"IC"`,
    /// `"IR"`, `"REGIONAL"`). Comparison is case-insensitive.
    #[must_use]
    pub fn service_type(mut self, service_type: impl Into<String>) -> Self {
        self.service_type = Some(service_type.into());
        self
    }

    /// Restrict matches to entries whose origin station name contains `station`
    /// (case-insensitive substring match).
    #[must_use]
    pub fn origin_station(mut self, station: impl Into<String>) -> Self {
        self.origin_station = Some(station.into());
        self
    }

    /// Restrict matches to entries whose destination station name contains
    /// `station` (case-insensitive substring match).
    #[must_use]
    pub fn destination_station(mut self, station: impl Into<String>) -> Self {
        self.destination_station = Some(station.into());
        self
    }

    /// Returns `true` if `entry` satisfies every condition set on this filter.
    ///
    /// An empty filter (no conditions set) matches every entry.
    #[must_use]
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
    #[must_use]
    #[deprecated(since = "0.2.0", note = "Use TrainEntryFilter instead")]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by train type
    #[must_use]
    #[deprecated(since = "0.2.0", note = "Use TrainEntryFilter::service_type instead")]
    pub fn train_type(mut self, train_type: impl Into<String>) -> Self {
        self.train_type = Some(train_type.into());
        self
    }

    /// Filter by platform
    #[must_use]
    #[deprecated(since = "0.2.0", note = "Platform filtering no longer supported")]
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
