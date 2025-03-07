
/// This macro is to avoid repetition
/// while obtaining an IP address from
/// an `actix_web::web::HttpRequest`
/// object.
///
/// It takes the request as a parameter
/// and returns the sender IP address
/// or 'unknown'.
#[macro_export]
macro_rules! ip_address {
    ($req:expr) => {
        $req.connection_info().peer_addr().unwrap_or("unknown")
    };
}
