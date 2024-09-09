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

#![cfg_attr(feature = "reasons", feature(lint_reasons))]

//	Customisations of the standard linting configuration
#![cfg_attr(    feature = "reasons",  allow(clippy::multiple_crate_versions, reason = "Cannot resolve all these"))]
#![cfg_attr(not(feature = "reasons"), allow(clippy::multiple_crate_versions))]



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


