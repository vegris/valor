// Poor man's anyhow
pub type AnyHow<T> = Result<T, Box<dyn std::error::Error>>;
pub type AnyWay = AnyHow<()>;
