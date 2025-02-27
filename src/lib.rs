use pgrx::prelude::*;

::pgrx::pg_module_magic!();
use pgrx::prelude::*;
use pgrx::PgSqlErrorCode;
use pgrx::{error, info, warning, PgRelation, FATAL, PANIC};

enum PgrxError {
    TestError,
}
impl core::error::Error for PgrxError {}
impl core::fmt::Debug for PgrxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::fmt::Display for PgrxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[pg_extern]
fn hello_pgrx_validation() -> &'static str {
    "Hello, pgrx_validation"
}

#[pg_extern]
fn strictly_positive(a: i32) -> bool {
    a > 0
}

#[pg_extern]
fn x_must_be_bigger_than_y(x: i32, y: i32) -> bool {
    if x > y {
        return true;
    }
    error!("X is smaller than Y")
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pgrx_validation() {
        assert_eq!("Hello, pgrx_validation", crate::hello_pgrx_validation());
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
