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

//	For an explanation of the following configuration, see:
//	https://github.com/danwilliams/standards-rs#code-linting

#![cfg_attr(feature = "reasons", feature(lint_reasons))]

//		Standard Rust compiler lints											
//	Future compatibility lints
#![deny(future_incompatible)]
//	Deprecated approach lints
#![deny(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]
//	Unused code lints
#![warn(unused)]
//	Cherry-picked lints
#![forbid(
	unsafe_code,
	unsafe_op_in_unsafe_fn,
)]
#![deny(
	deprecated,
	deprecated_where_clause_location,
	incomplete_features,
	internal_features,
	macro_use_extern_crate,
	unknown_lints,
	unnameable_test_items,
	unreachable_pub,
)]
#![warn(
	let_underscore_drop,
	meta_variable_misuse,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unused_crate_dependencies,
	unused_import_braces,
	unused_lifetimes,
	unused_qualifications,
	unused_results,
	variant_size_differences,
)]
//		Clippy lints															
//	Clippy lint categories
#![warn(
	clippy::cargo,
	clippy::nursery,
	clippy::pedantic,
)]
//	Clippy cargo lints
#![deny(
	clippy::negative_feature_names,
	clippy::wildcard_dependencies,
)]
//	Clippy pedantic lints
#![deny(
	clippy::pub_underscore_fields,
)]
#![cfg_attr(    feature = "reasons",  allow(clippy::module_name_repetitions, reason = "This is not required"))]
#![cfg_attr(not(feature = "reasons"), allow(clippy::module_name_repetitions))]
//	Clippy restriction lints
#![forbid(
	clippy::allow_attributes_without_reason,
	clippy::dbg_macro,
	clippy::exit,
	clippy::infinite_loop,
	clippy::missing_assert_message,
	clippy::missing_docs_in_private_items,
	clippy::mod_module_files,
	clippy::multiple_inherent_impl,
	clippy::panic_in_result_fn,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::tests_outside_test_module,
	clippy::unimplemented,
)]
#![deny(
	clippy::clone_on_ref_ptr,
	clippy::empty_enum_variants_with_brackets,
	clippy::empty_structs_with_brackets,
	clippy::error_impl_error,
	clippy::exhaustive_enums,
	clippy::exhaustive_structs,
	clippy::expect_used,
	clippy::float_cmp_const,
	clippy::fn_to_numeric_cast_any,
	clippy::format_push_string,
	clippy::get_unwrap,
	clippy::impl_trait_in_params,
	clippy::integer_division,
	clippy::lossy_float_literal,
	clippy::mem_forget,
	clippy::panic,
	clippy::print_stderr,
	clippy::print_stdout,
	clippy::rc_mutex,
	clippy::try_err,
	clippy::unwrap_in_result,
	clippy::unwrap_used,
	clippy::wildcard_enum_match_arm,
)]
#![warn(
	clippy::absolute_paths,
	clippy::arithmetic_side_effects,
	clippy::as_underscore,
	clippy::decimal_literal_representation,
	clippy::default_numeric_fallback,
	clippy::deref_by_slicing,
	clippy::empty_drop,
	clippy::filetype_is_file,
	clippy::if_then_some_else_none,
	clippy::indexing_slicing,
	clippy::iter_over_hash_type,
	clippy::let_underscore_must_use,
	clippy::let_underscore_untyped,
	clippy::map_err_ignore,
	clippy::missing_asserts_for_indexing,
	clippy::mixed_read_write_in_expression,
	clippy::mutex_atomic,
	clippy::pattern_type_mismatch,
	clippy::pub_without_shorthand,
	clippy::rc_buffer,
	clippy::redundant_type_annotations,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::same_name_method,
	clippy::semicolon_outside_block,
	clippy::shadow_reuse,
	clippy::shadow_same,
	clippy::shadow_unrelated,
	clippy::std_instead_of_core,
	clippy::string_lit_chars_any,
	clippy::string_slice,
	clippy::suspicious_xor_used_as_pow,
	clippy::todo,
	clippy::unnecessary_safety_comment,
	clippy::unnecessary_safety_doc,
	clippy::unneeded_field_pattern,
	clippy::unreachable,
	clippy::unseparated_literal_suffix,
	clippy::use_debug,
	clippy::verbose_file_reads,
)]
//	Clippy suspicious lints
#![deny(
	clippy::unconditional_recursion,
)]



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
