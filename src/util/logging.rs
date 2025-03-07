use std::{fmt::format, io::{Result, Write}};
use colored::Colorize;
use flexi_logger::DeferredNow;
use log::Record;


pub fn format_log(w: &mut dyn Write, now: &mut DeferredNow, record: &Record) -> Result<()> {
    let level = match record.level() {
        log::Level::Error => "ERROR".red(),
        log::Level::Warn => "WARN".yellow(),
        log::Level::Info => "INFO".green(),
        log::Level::Debug => "DEBUG".purple(),
        log::Level::Trace => "TRACE".cyan(),
    };

    write!(
        w,
        "[{} {}] › {}",
        now.format("%H:%M:%S %Y-%m-%d")
            .to_string()
            .white(),
        level,
        format(*record.args())
            .to_string()
            .white()
    )
}
