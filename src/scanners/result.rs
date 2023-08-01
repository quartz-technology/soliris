use colored::*;
use std::fmt;

#[allow(dead_code)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Boost,
}

impl Severity {
    fn to_colored_string(&self) -> ColoredString {
        match self {
            Severity::Error => "ERROR".bold().red(),
            Severity::Warning => "WARNING".bold().yellow(),
            Severity::Info => "INFO".bold().blue(),
            Severity::Boost => "BOOST".bold().cyan(),
        }
    }
}

struct Report {
    line: usize,
    column: usize,
    severity: Severity,
    message: String,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            format!(
                "{:>4}:{:<4} {:10} {}",
                self.line,
                self.column,
                self.severity.to_colored_string(),
                self.message
            )
            .as_str(),
        )
    }
}

#[derive(Default)]
pub struct Reporter {
    reports: Vec<Report>,
}

impl Reporter {
    pub fn report(&mut self, line: usize, column: usize, severity: Severity, message: &str) {
        self.reports.push(Report {
            line,
            column,
            severity,
            message: message.to_string(),
        })
    }

    pub fn log(self, file_path: &str) {
        println!("{}", file_path.bold().underline());
        self.reports.iter().for_each(|r| println!("{}", r));
    }
}
