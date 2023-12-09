#![allow(non_snake_case)]

//		Tests

//		CountryCode																
#[cfg(test)]
mod country_code__enum {
	use super::super::*;
	
	//		country																
	#[test]
	fn country() {
		let country = CountryCode::US.country();
		assert_eq!(country.name, "United States of America");
		assert_eq!(country.code, CountryCode::US);
	}
	#[test]
	fn country__all() {
		for code in COUNTRIES.keys() {
			assert_eq!(code.country().code, *code);
		}
	}
	#[test]
	fn country__relationships() {
		for country_code in COUNTRIES.keys() {
			let country = country_code.country();
			for currency_code in country.currencies.iter() {
				assert!(currency_code.currency().countries().contains(country_code));
			}
			for language_code in country.languages.iter() {
				assert!(language_code.language().countries().contains(country_code));
			}
		}
	}
}

#[cfg(test)]
mod country_code__traits {
	use super::super::*;
	use claims::assert_err;
	use serde_json;
	
	//		as_str																
	#[test]
	fn as_str() {
		assert_eq!(CountryCode::US.as_str(), "US");
	}
	
	//		debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", CountryCode::US), "US");
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let code: CountryCode = serde_json::from_str(r#""US""#).unwrap();
		assert_eq!(code, CountryCode::US);
	}
	
	//		display																
	#[test]
	fn display() {
		let code = CountryCode::US;
		assert_eq!(format!("{}", code), "US");
		assert_eq!(code.to_string(),    "US");
	}
	
	//		eq / partial_eq														
	#[test]
	fn eq() {
		assert_eq!(CountryCode::US, CountryCode::US);
	}
	#[test]
	fn ne() {
		assert_ne!(CountryCode::US, CountryCode::GB);
	}
	
	//		from																
	#[test]
	fn from__country_code_for_u16() {
		let code = CountryCode::US;
		assert_eq!(u16::from(code), 840);
		assert_eq!(code as u16,     840);
		let int: u16 = code.into();
		assert_eq!(int,             840);
	}
	#[test]
	fn from__country_code_for_string() {
		let code = CountryCode::US;
		assert_eq!(String::from(code), "US");
		let str: String = code.into();
		assert_eq!(str,                "US");
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_eq!(CountryCode::from_str("US").unwrap(), CountryCode::US);
		let err = CountryCode::from_str("FOO");
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid CountryCode: FOO");
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		assert_eq!(serde_json::to_string(&CountryCode::US).unwrap(), r#""US""#);
	}
	
	//		try_from															
	#[test]
	fn try_from__u16() {
		assert_eq!(CountryCode::try_from(840).unwrap(), CountryCode::US);
		let err = CountryCode::try_from(000);
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid CountryCode: 0");
	}
	#[test]
	fn try_from__string() {
		assert_eq!(CountryCode::try_from(s!("US")).unwrap(), CountryCode::US);
		let err = CountryCode::try_from(s!("FOO"));
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid CountryCode: FOO");
	}
}

//		Country																	
#[cfg(test)]
mod country__struct {
	use super::super::*;
	
	//		name																
	#[test]
	fn name() {
		assert_eq!(CountryCode::CH.country().name(), "Switzerland");
	}
	
	//		code																
	#[test]
	fn code() {
		assert_eq!(CountryCode::CH.country().code(), CountryCode::CH);
	}
	
	//		currencies															
	#[test]
	fn currencies() {
		assert_eq!(CountryCode::CH.country().currencies(), &vh![ CurrencyCode: CHE, CHF, CHW ]);
	}
	
	//		languages															
	#[test]
	fn languages() {
		assert_eq!(CountryCode::CH.country().languages(), &vh![ LanguageCode: DE, FR, IT, RM ]);
	}
}

#[cfg(test)]
mod country__traits {
	use super::super::*;
	use claims::assert_err;
	use serde_json;
	
	//		as_str																
	#[test]
	fn as_str() {
		assert_eq!(CountryCode::US.country().as_str(), "United States of America");
	}
	
	//		debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", CountryCode::US.country()), "US: United States of America");
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let country: Country = serde_json::from_str(r#""United States of America""#).unwrap();
		assert_eq!(country, *CountryCode::US.country());
	}
	
	//		display																
	#[test]
	fn display() {
		let country = CountryCode::US.country();
		assert_eq!(format!("{}", country), "United States of America");
		assert_eq!(country.to_string(),    "United States of America");
	}
	
	//		eq / partial_eq														
	#[test]
	fn eq() {
		assert_eq!(CountryCode::US.country(), CountryCode::US.country());
	}
	#[test]
	fn ne() {
		assert_ne!(CountryCode::US.country(), CountryCode::GB.country());
	}
	
	//		from																
	#[test]
	fn from__country_for_string() {
		let country = CountryCode::US.country();
		assert_eq!(String::from(country.clone()), "United States of America");
		let str: String = country.clone().into();
		assert_eq!(str,                           "United States of America");
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_eq!(Country::from_str("United States of America").unwrap(), *CountryCode::US.country());
		let err = Country::from_str("Fooland");
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid Country: Fooland");
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		assert_eq!(serde_json::to_string(CountryCode::US.country()).unwrap(), r#""United States of America""#);
	}
	
	//		try_from															
	#[test]
	fn try_from__string() {
		assert_eq!(Country::from_str("United States of America").unwrap(), *CountryCode::US.country());
		let err = Country::from_str("Fooland");
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid Country: Fooland");
	}
}


