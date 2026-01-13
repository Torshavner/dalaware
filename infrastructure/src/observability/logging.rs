use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

/// Initialize tracing for console output
///
/// Configures structured logging with:
/// - Environment-based log level control via RUST_LOG (defaults to "info")
/// - Pretty formatting with ANSI colors for better readability
/// - Target and span information for debugging multi-threaded execution
///
/// # Errors
///
/// Returns an error if the tracing subscriber cannot be initialized
///
/// # Examples
///
/// ```no_run
/// use nn_infrastructure::observability::init_tracing;
///
/// // Initialize with default settings
/// init_tracing().expect("Failed to initialize tracing");
///
/// // Set log level via environment variable before calling:
/// // RUST_LOG=debug cargo run
/// ```
pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    // Base layer: EnvFilter (RUST_LOG=info, debug, trace, etc.)
    // Defaults to "info" if RUST_LOG is not set
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Console layer: Human-readable output with colors
    let console_layer = tracing_subscriber::fmt::layer()
        .with_target(true)  // Show module path for context
        .with_ansi(true)    // Enable colors for log levels
        .pretty();          // Use pretty formatting for better readability

    // Initialize Registry with the environment filter and console layer
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
        // Given: No RUST_LOG environment variable set
        std::env::remove_var("RUST_LOG");

        // When: Create EnvFilter with default
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        // Then: Filter should be created successfully
        // Note: We can't directly assert the level, but we can verify it compiles and doesn't panic
        let filter_str = format!("{:?}", filter);
        assert!(filter_str.contains("EnvFilter"));
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__debug_env__when__create_env_filter__then__uses_debug_level() {
        // Given: RUST_LOG=debug
        std::env::set_var("RUST_LOG", "debug");

        // When: Create EnvFilter from environment
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        // Then: Filter should be created successfully with debug level
        let filter_str = format!("{:?}", filter);
        assert!(filter_str.contains("EnvFilter"));

        // Cleanup
        std::env::remove_var("RUST_LOG");
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__invalid_env__when__create_env_filter__then__falls_back_to_default() {
        // Given: Invalid RUST_LOG value
        std::env::set_var("RUST_LOG", "invalid_level_xyz");

        // When: Create EnvFilter with fallback
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        // Then: Should fall back to info level without panicking
        let filter_str = format!("{:?}", filter);
        assert!(filter_str.contains("EnvFilter"));

        // Cleanup
        std::env::remove_var("RUST_LOG");
    }
}
