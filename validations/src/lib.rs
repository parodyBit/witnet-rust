//! Validations

#![deny(rust_2018_idioms)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(missing_docs)]

/// Module containing general purpose validations
pub mod validations;

/// Module containing validations specific to witnessing
pub mod witnessing;

/// Module contaning validations specific to eligibility
pub mod eligibility;

#[cfg(test)]
mod tests;
