//! Country-related types.



//		Packages

use crate::{
	currency::CurrencyCode,
	language::LanguageCode,
};
use core::{
	fmt::{Display, self},
	str::FromStr,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::ToSchema;



//		Enums

//		CountryCode																
/// The possible countries.
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ToSchema)]
#[non_exhaustive]
pub enum CountryCode {
	/// Great Britain.
	GBR,
}

impl Display for CountryCode {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::GBR => write!(f, "GBR"),
		}
	}
}

impl FromStr for CountryCode {
	type Err = String;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			"GBR" => Ok(Self::GBR),
			_     => Err(format!("Invalid CountryCode: {s}")),
		}
	}
}



//		Structs

//		Country																	
/// A country.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[non_exhaustive]
pub struct Country {
	//		Public properties													
	/// The name of the country.
	pub name:       String,
	
	/// The country code.
	pub code:       CountryCode,
	
	/// The currencies used in the country.
	pub currencies: HashSet<CurrencyCode>,
	
	/// The languages spoken in the country.
	pub languages:  HashSet<LanguageCode>,
}


