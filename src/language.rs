//! Language-related types.



//		Packages

use crate::country::CountryCode;
use core::{
	fmt::{Display, self},
	str::FromStr,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::ToSchema;



//		Enums

//		LanguageCode															
/// The possible languages.
#[cfg_attr(    feature = "reasons",  allow(clippy::upper_case_acronyms, reason = "Uppercase is suitable here"))]
#[cfg_attr(not(feature = "reasons"), allow(clippy::upper_case_acronyms))]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ToSchema)]
#[serde(into = "String", try_from = "String")]
#[non_exhaustive]
pub enum LanguageCode {
	/// English.
	EN,
}

impl Display for LanguageCode {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::EN => write!(f, "EN"),
		}
	}
}

impl FromStr for LanguageCode {
	type Err = String;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			"EN" => Ok(Self::EN),
			_    => Err(format!("Invalid LanguageCode: {s}")),
		}
	}
}



//		Structs

//		Language																
/// A language.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[non_exhaustive]
pub struct Language {
	//		Public properties													
	/// The name of the language.
	pub name:      String,
	
	/// The language code.
	pub code:      LanguageCode,
	
	/// The countries where the language is spoken.
	pub countries: HashSet<CountryCode>,
}


