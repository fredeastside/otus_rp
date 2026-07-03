//! Reporting abstraction shared by all entities that can describe themselves.

/// Anything that can produce a human-readable report about itself.
pub trait Reporter {
    /// Builds and returns the report text for this entity.
    fn report(&self) -> String;
}

/// Accepts any object that can produce a report and prints it to standard output.
pub fn report(item: &impl Reporter) {
    println!("{}", item.report());
}
