//! Currency-related types.



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

//		CurrencyCode															
/// The possible currencies.
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ToSchema)]
#[non_exhaustive]
pub enum CurrencyCode {
	/// Euro.
	EUR,
	
	/// British pound sterling.
	GBP,
	
	/// United States dollar.
	USD,
}

impl Display for CurrencyCode {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::EUR => write!(f, "EUR"),
			Self::USD => write!(f, "USD"),
			Self::GBP => write!(f, "GBP"),
		}
	}
}

impl FromStr for CurrencyCode {
	type Err = String;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			"EUR" => Ok(Self::EUR),
			"USD" => Ok(Self::USD),
			"GBP" => Ok(Self::GBP),
			_     => Err(format!("Invalid CurrencyCode: {s}")),
		}
	}
}


//		Structs

//		Currency																
/// A currency.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[non_exhaustive]
pub struct Currency {
	//		Public properties													
	/// The name of the currency.
	pub name:      String,
	
	/// The currency code.
	pub code:      CurrencyCode,
	
	/// The countries where the currency is used.
	pub countries: HashSet<CountryCode>,
}


