#[derive(Debug, Default, Clone)]
pub struct TrafficMetrics {
    pub vehicles_active: u32,
    pub total_wait_secs: f32,
    pub max_queue_wait_secs: f32,
    pub trips_completed: u32,
}