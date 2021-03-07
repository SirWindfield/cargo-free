#[cfg(feature = "colors")]
use colored::Colorize;
use std::{fmt, fmt::Formatter, time::Duration};

const TIMEOUT: u64 = 5;

/// The availability status of a crate name.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Availability {
    /// The crate name is available.
    Available,
    /// The crate name is unavailable.
    Unavailable,
    /// The crate name can't be resolved.
    Unknown,
}

impl fmt::Display for Availability {
    #[cfg(not(feature = "colors"))]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Availability::Available => write!(f, "Available"),
            Availability::Unavailable => write!(f, "Unavailable"),
            Availability::Unknown => write!(f, "Unknown"),
        }
    }

    #[cfg(feature = "colors")]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Availability::Available => write!(f, "{}", "Available".green()),
            Availability::Unavailable => write!(f, "{}", "Unavailable".red()),
            Availability::Unknown => write!(f, "{}", "Unknown".bright_black()),
        }
    }
}

/// Checks the availability for a given crate name.
///
/// # Arguments
///
/// - `name`: The crate name to check
///
/// # Returns
///
/// `Ok(Availability)` if the name could be resolved, an error otherwise.
///
/// # Note
///
/// The needed network request will timeout after five seconds.
pub fn check_availability(name: impl AsRef<str>) -> Result<Availability, ()> {
    check_availability_with_timeout(name, Duration::from_secs(TIMEOUT))
}

/// Checks the availability for a given crate name. Stops after the given timeout duration and returns `Availability::Unknown`.
///
/// # Arguments
///
/// - `name`: The crate name to check
/// - `timeout`: The timeout after which to stop trying to connect to the crates.io API.
///
/// # Returns
///
/// `Ok(Availability)` if the name could be resolved, an error otherwise.
pub fn check_availability_with_timeout(
    name: impl AsRef<str>,
    timeout: Duration,
) -> Result<Availability, ()> {
    let name = name.as_ref();
    if name.is_empty() {
        eprintln!("Crate name can't be empty");
        return Err(());
    }

    let url = format!("https://crates.io/api/v1/crates/{}", name);
    let resp = ureq::get(&url).timeout(timeout).call();
    let availability = match resp.status() {
        200 => Availability::Unavailable,
        404 => Availability::Available,
        _ => Availability::Unknown,
    };
    Ok(availability)
}
