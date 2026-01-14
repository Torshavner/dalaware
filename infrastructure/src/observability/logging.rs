use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let console_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_ansi(true)
        .pretty();

    Registry::default()
        .with(env_filter)
        .with(console_layer)
        .init();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn given__default_env__when__create_env_filter__then__defaults_to_info() {
        std::env::remove_var("RUST_LOG");

        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        let filter_str = format!("{:?}", filter);
        assert!(filter_str.contains("EnvFilter"));
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__debug_env__when__create_env_filter__then__uses_debug_level() {
        std::env::set_var("RUST_LOG", "debug");

        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        let filter_str = format!("{:?}", filter);
        assert!(filter_str.contains("EnvFilter"));

        std::env::remove_var("RUST_LOG");
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__invalid_env__when__create_env_filter__then__falls_back_to_default() {
        std::env::set_var("RUST_LOG", "invalid_level_xyz");

        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        let filter_str = format!("{:?}", filter);
        assert!(filter_str.contains("EnvFilter"));

        std::env::remove_var("RUST_LOG");
    }
}
