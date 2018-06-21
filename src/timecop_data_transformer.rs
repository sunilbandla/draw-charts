#[derive(Debug)]
pub struct DataPoint {
    pub id: i64,
    pub ts_ms: i64,
    pub sched_us: i64,
    pub mono_drift_usps: i64,
    pub wallclock_drift_usps: i64
}
