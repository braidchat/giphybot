use std::error::Error;
use std::fmt;

// Route error handler
#[derive(Debug)]
pub struct NoRoute;

impl fmt::Display for NoRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("No matching route found.")
    }
}

impl Error for NoRoute {
    fn description(&self) -> &str { "No Route" }
}

#[derive(Debug)]
pub struct MissingMac;

impl fmt::Display for MissingMac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Missing X-Braid-Signature header")
    }
}

impl Error for MissingMac {
    fn description(&self) -> &str { "Missing signature header" }
}

#[derive(Debug)]
pub struct BadMac;

impl fmt::Display for BadMac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Mac check failed")
    }
}

impl Error for BadMac {
    fn description(&self) -> &str { "Bad signature header" }
}
