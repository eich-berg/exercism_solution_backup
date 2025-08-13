use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // todo!("What time is a gigasecond later than {start}");
    let start_utc = start.assume_utc();
    // Add 1 billion seconds (gigasecond)
    let end = start_utc + time::Duration::seconds(1_000_000_000);
    // Convert back to PrimitiveDateTime
    DateTime::new(end.date(), end.time())
}