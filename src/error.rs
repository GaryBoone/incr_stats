pub type Result<T> = std::result::Result<T, StatsError>;

#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
pub enum StatsError {
    #[error("not enough data")]
    NotEnoughData,
    #[error("data produces undefined values")]
    Undefined,
    #[error("data contains NaNs or Infs")]
    InvalidData,
}
