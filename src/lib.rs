/// Converts a total in cents and an item count into an average price in dollars.
///
/// ```
/// use dw09::average_dollars;
///
/// assert_eq!(average_dollars(750, 4), 1.875);
/// ```
///
/// ```
/// use dw09::average_dollars;
///
/// assert_eq!(average_dollars(1000, 4), 2.5);
/// ```
/// Hint: Consider what happens to the decimal values before we 
/// cast from u32 to f64.
pub fn average_dollars(total_cents: u32, count: u32) -> f64 {
    (total_cents as f64/ 100.0 / count as f64) as f64 
}

/// Compares an average price against a spending limit (both in dollars).
///
/// ```
/// use dw09::budget_status;
///
/// assert_eq!(budget_status(1.875, 1.50), "Above budget");
/// ```
///
/// ```
/// use dw09::budget_status;
///
/// assert_eq!(budget_status(1.00, 1.50), "Within budget");
/// ```
pub fn budget_status(average: f64, limit: f64) -> &'static str {
    if average > limit {
        "Above budget"
    } else {
        "Within budget"
    }
}
