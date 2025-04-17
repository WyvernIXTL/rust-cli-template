use std::fs::create_dir_all;
use std::path::PathBuf;
use std::{io, sync::Once};

use directories::BaseDirs;
use std::env::consts;
use tracing::debug;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_error::ErrorLayer;
use tracing_subscriber::registry;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt::{format::FmtSpan, layer},
    prelude::*,
};

/// Setup of pretty backtrace and spantrace printing.
///
/// Backtraces are printed in debug mode.
/// Spantraces are printed in both debug and release mode.
/// In release mode an url is generated for creating an issue with backtrace and error message included.
fn setup_color_eyre() {
    if cfg!(debug_assertions) {
        if std::env::var("RUST_BACKTRACE").is_err() {
            unsafe {
                std::env::set_var("RUST_BACKTRACE", "full");
            }
        }
    } else {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "0");
        }
    }

    let mut hook_builder = color_eyre::config::HookBuilder::new()
        .capture_span_trace_by_default(true)
        .display_env_section(cfg!(debug_assertions));

    if !cfg!(debug_assertions) {
        hook_builder = hook_builder.issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))
    }

    hook_builder
        .add_issue_metadata("version", env!("CARGO_PKG_VERSION"))
        .add_issue_metadata("os", consts::OS)
        .add_issue_metadata("arch", consts::ARCH)
        .install()
        .expect("Failed setting up color eyre (pretty console errors and printing of backtraces).");
}

/// Creates an app folder and a folder for logging within if those do not already exist.
/// Returns the path to the logging folder.
fn setup_app_folder() -> Option<PathBuf> {
    let base_dirs = BaseDirs::new()?;

    let app_dir = base_dirs
        .data_local_dir()
        .to_path_buf()
        .join(env!("CARGO_PKG_NAME"))
        .join("logging");

    if let Err(err) = create_dir_all(&app_dir) {
        eprintln!(
            "Failed creating directories needed for logging to file: {}\n Proceeding without logging to file.",
            err
        );

        return None;
    }

    Some(app_dir)
}

/// Setups a blocking logger for terminal that can be set with env variables.
///
/// See <https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>.
fn setup_debug_logger() {
    let env_filter_terminal = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();

    let terminal_layer = layer()
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::ACTIVE)
        .with_writer(io::stderr)
        .with_filter(env_filter_terminal);

    registry()
        .with(terminal_layer)
        .with(ErrorLayer::default())
        .init();
}

/// Setup a non blocking logger for terminal and a rolling file logger.
///
/// If the rolling file logger fails to initialize, an error message is displayed, but the program continues to execute.
fn setup_release_logger() -> Vec<WorkerGuard> {
    let (terminal_non_blocking, terminal_guard) = tracing_appender::non_blocking(io::stderr());

    let env_filter_terminal = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let terminal_layer = layer()
        .compact()
        .without_time()
        .with_target(false)
        .with_writer(terminal_non_blocking)
        .with_filter(env_filter_terminal);

    let file_appender_option: Option<RollingFileAppender> =  setup_app_folder().map_or(None, |app_dir| {
            RollingFileAppender::builder()
                .rotation(Rotation::DAILY)
                .filename_prefix(env!("CARGO_PKG_NAME"))
                .filename_suffix("log")
                .build(app_dir)
                .inspect_err(|e| {
                    eprintln!("Failed initializing file logger (rolling appender): {}\n Proceeding without logging to file.", e)
                })
                .ok()
        });

    if let Some(file_appender) = file_appender_option {
        let (file_non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);

        let file_layer = layer()
            .json()
            .with_span_list(true)
            .with_writer(file_non_blocking)
            .with_file(true)
            .with_line_number(true)
            .with_filter(LevelFilter::DEBUG);

        registry()
            .with(terminal_layer)
            .with(file_layer)
            .with(ErrorLayer::default())
            .init();
        vec![terminal_guard, file_guard]
    } else {
        registry()
            .with(terminal_layer)
            .with(ErrorLayer::default())
            .init();
        vec![terminal_guard]
    }
}

/// Setups a blocking logger (terminal) in debug mode
/// and 2 non blocking loggers (terminal, file) in release mode.
fn setup_logger() -> Vec<WorkerGuard> {
    if cfg!(debug_assertions) {
        setup_debug_logger();
        vec![]
    } else {
        setup_release_logger()
    }
}

/// Logs name and version of program, as well as the os and the architecture.
fn log_runtime_info() {
    debug!(
        name = %env!("CARGO_PKG_NAME"),
        version = %env!("CARGO_PKG_VERSION"),
        os = %consts::OS,
        arch = %consts::ARCH,
        "program_and_env_info"
    );
}

static SETUP_ONCE: Once = Once::new();

/// Setup logging to terminal and files and pretty error messages on program startup.
///
/// Behavior is different between debug and release mode.
/// In debug mode a blocking terminal logger is initialized. The backtrace is shown in full by default.
/// In release mode 2 non blocking loggers are used. One logs to the terminal and one logs to a file.
/// The terminal logger does not show verbose info like time or file. The file logger logs some extras
/// like span chain. On error no backtrace is shown, but a issue url is generated that has said
/// backtrace encoded in it.
pub fn setup() -> Vec<WorkerGuard> {
    let mut guard = vec![];

    SETUP_ONCE.call_once(|| {
        guard = setup_logger();

        log_runtime_info();

        setup_color_eyre();
    });

    guard
}
