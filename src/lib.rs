//! The Isosphere crate is a library of ISO standard data types, helpers, and
//! related utilities for Rust projects.
//! 
//! As a general approach, each module in this crate provides a primary type, to
//! be the main focus of interaction with the module and its data, and an
//! associated code enum, which has the purpose of being used for
//! (de)serialisation and other similar purposes.
//! 
//! Conversion, especially as regards (de)serialisation, is a key concern and
//! priority, and so the design of the various types is intended to make this as
//! easy, intuitive, and flexible as possible, with full support for [Serde](https://crates.io/crates/serde).
//! 
//! Notably, all types implement traits from [Utoipa](https://crates.io/crates/utoipa),
//! which provides Open&#8203;API functionality, meaning they will be compatible
//! for listing in Swagger and other Open&#8203;API documentation in your
//! applications.
//! 



//		Global configuration

//	Customisations of the standard linting configuration
#![allow(clippy::multiple_crate_versions, reason = "Cannot resolve all these")]
#![allow(clippy::items_after_test_module, reason = "Not needed with separated tests")]

//	Lints specifically disabled for unit tests
#![cfg_attr(test, allow(
	non_snake_case,
	clippy::arithmetic_side_effects,
	clippy::cast_lossless,
	clippy::cast_precision_loss,
	clippy::cognitive_complexity,
	clippy::default_numeric_fallback,
	clippy::exhaustive_enums,
	clippy::exhaustive_structs,
	clippy::expect_used,
	clippy::indexing_slicing,
	clippy::let_underscore_must_use,
	clippy::let_underscore_untyped,
	clippy::missing_assert_message,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::panic,
	clippy::print_stdout,
	clippy::too_many_lines,
	clippy::unwrap_in_result,
	clippy::unwrap_used,
	reason = "Not useful in unit tests"
))]



//		Modules

pub mod country;
pub mod currency;
pub mod language;

pub use {
	country::Country,
	country::CountryCode,
	currency::Currency,
	currency::CurrencyCode,
	language::Language,
	language::LanguageCode,
};


