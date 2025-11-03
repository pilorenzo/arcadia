use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct Env {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
    #[envconfig(nested)]
    pub tracker: TrackerConfig,
}

#[derive(Envconfig, Clone)]
pub struct TrackerConfig {
    #[envconfig(from = "ARCADIA_TRACKER_ANNOUNCE_INTERVAL")]
    pub announce_interval: u32,
}
