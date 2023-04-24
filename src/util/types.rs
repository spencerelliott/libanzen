/// Encapsulates the response of a call that requires an `Ok` response.
pub enum OkResult<T> {
    /// Returned on a successful message call.
    Ok(T),
    /// Returned when an error is encountered during a message call.
    Error(&'static str)
}

/// Encapsulates the response of a call that requires a `Valid` response.
pub enum ValidResult<T> {
    /// Returned when a valid object can be returned
    Valid(T),
    /// Returned when the object is invalid
    Invalid(&'static str)
}