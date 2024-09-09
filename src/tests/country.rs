//		Tests

//		CountryCode																
#[cfg(test)]
mod country_code__enum {
	use super::super::*;
	
	//		all																	
	#[test]
	fn all() {
		let codes = CountryCode::all();
		assert_eq!(codes.len(), 249);
		assert!(codes.contains(&CountryCode::US));
		assert!(codes.contains(&CountryCode::FR));
		assert!(codes.contains(&CountryCode::GB));
	}
	
	//		country																
	#[test]
	fn country() {
		let country1 = CountryCode::US.country();
		assert_eq!(country1.name(), "United States of America");
		assert_eq!(country1.code(), CountryCode::US);
		
		let country2 = CountryCode::USA.country();
		assert_eq!(country2.name(), "United States of America");
		assert_eq!(country2.code(), CountryCode::US);
	}
	#[test]
	fn country__all() {
		#[cfg_attr(    feature = "reasons",  allow(clippy::iter_over_hash_type, reason = "Order is not important here"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::iter_over_hash_type))]
		for country in COUNTRIES.keys() {
			assert_eq!(country.code().country(), *country);
		}
	}
	
	//		is_alpha2															
	#[test]
	fn is_alpha2() {
		assert!( CountryCode::US .is_alpha2());
		assert!(!CountryCode::USA.is_alpha2());
	}
	
	//		is_alpha3															
	#[test]
	fn is_alpha3() {
		assert!(!CountryCode::US .is_alpha3());
		assert!( CountryCode::USA.is_alpha3());
	}
	
	//		to_alpha2															
	#[test]
	fn to_alpha2() {
		assert_eq!(CountryCode::US .to_alpha2(), CountryCode::US);
		assert_eq!(CountryCode::USA.to_alpha2(), CountryCode::US);
	}
	
	//		to_alpha3															
	#[test]
	fn to_alpha3() {
		assert_eq!(CountryCode::US .to_alpha3(), CountryCode::USA);
		assert_eq!(CountryCode::USA.to_alpha3(), CountryCode::USA);
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
		assert_eq!(CountryCode::US .as_str(), "US");
		assert_eq!(CountryCode::USA.as_str(), "USA");
	}
	
	//		debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", CountryCode::US),  "US");
		assert_eq!(format!("{:?}", CountryCode::USA), "USA");
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let code1: CountryCode = serde_json::from_str(r#""US""#).unwrap();
		assert_eq!(code1, CountryCode::US);
		let code2: CountryCode = serde_json::from_str(r#""us""#).unwrap();
		assert_eq!(code2, CountryCode::US);
		
		let code3: CountryCode = serde_json::from_str(r#""USA""#).unwrap();
		assert_eq!(code3, CountryCode::USA);
		let code4: CountryCode = serde_json::from_str(r#""usa""#).unwrap();
		assert_eq!(code4, CountryCode::USA);
	}
	
	//		display																
	#[test]
	fn display() {
		let code1 = CountryCode::US;
		assert_eq!(format!("{code1}"), "US");
		assert_eq!(code1.to_string(),  "US");
		
		let code2 = CountryCode::USA;
		assert_eq!(format!("{code2}"), "USA");
		assert_eq!(code2.to_string(),  "USA");
	}
	
	//		eq / partial_eq														
	#[test]
	fn eq() {
		assert_eq!(CountryCode::US, CountryCode::US);
	}
	#[test]
	fn ne() {
		assert_ne!(CountryCode::US, CountryCode::GB);
		assert_ne!(CountryCode::US, CountryCode::USA);
	}
	
	//		from																
	#[test]
	fn from__country_code_for_u16() {
		let code1 = CountryCode::US;
		assert_eq!(u16::from(code1), 840);
		assert_eq!(code1 as u16,     840);
		let int1: u16 = code1.into();
		assert_eq!(int1,             840);
		
		let code2 = CountryCode::USA;
		assert_eq!(u16::from(code2),   840);
		assert_eq!(code2 as u16,     1_840);
		let int2: u16 = code2.into();
		assert_eq!(int2,               840);
	}
	#[test]
	fn from__country_code_for_string() {
		let code1 = CountryCode::US;
		assert_eq!(String::from(code1), "US");
		let str1: String = code1.into();
		assert_eq!(str1,                "US");
		
		let code2 = CountryCode::USA;
		assert_eq!(String::from(code2), "USA");
		let str2: String = code2.into();
		assert_eq!(str2,                "USA");
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_eq!(CountryCode::from_str("US") .unwrap(), CountryCode::US);
		assert_eq!(CountryCode::from_str("us") .unwrap(), CountryCode::US);
		assert_eq!(CountryCode::from_str("USA").unwrap(), CountryCode::USA);
		assert_eq!(CountryCode::from_str("usa").unwrap(), CountryCode::USA);
		let err = CountryCode::from_str("FOO");
		assert_err!(&err);
		assert_eq!(err.unwrap_err(), "Invalid CountryCode: FOO");
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		assert_eq!(serde_json::to_string(&CountryCode::US) .unwrap(), r#""US""#);
		assert_eq!(serde_json::to_string(&CountryCode::USA).unwrap(), r#""USA""#);
	}
	
	//		try_from															
	#[test]
	fn try_from__u16() {
		assert_eq!(CountryCode::try_from(840).unwrap(), CountryCode::US);
		let err1 = CountryCode::try_from(000);
		assert_err!(&err1);
		assert_eq!(err1.unwrap_err(), "Invalid CountryCode: 0");
		
		let err2 = CountryCode::try_from(1840);
		assert_err!(&err2);
		assert_eq!(err2.unwrap_err(), "Invalid CountryCode: 1840");
	}
	#[test]
	fn try_from__string() {
		assert_eq!(CountryCode::try_from(s!("US")) .unwrap(), CountryCode::US);
		assert_eq!(CountryCode::try_from(s!("us")) .unwrap(), CountryCode::US);
		assert_eq!(CountryCode::try_from(s!("USA")).unwrap(), CountryCode::USA);
		assert_eq!(CountryCode::try_from(s!("usa")).unwrap(), CountryCode::USA);
		let err = CountryCode::try_from(s!("FOO"));
		assert_err!(&err);
		assert_eq!(err.unwrap_err(), "Invalid CountryCode: FOO");
	}
}

//		Country																	
#[cfg(test)]
mod country__enum {
	use super::super::*;
	
	//		all																	
	#[test]
	fn all() {
		let countries = Country::all();
		assert_eq!(countries.len(), 249);
		assert!(countries.contains(&Country::US));
		assert!(countries.contains(&Country::FR));
		assert!(countries.contains(&Country::GB));
	}
	
	//		info																
	#[test]
	fn info() {
		let info = Country::US.info();
		assert_eq!(info.name, "United States of America");
		assert_eq!(info.code, CountryCode::US);
	}
	
	//		name																
	#[test]
	fn name() {
		assert_eq!(Country::CH.name(), "Switzerland");
	}
	
	//		code																
	#[test]
	fn code() {
		assert_eq!(Country::CH.code(), CountryCode::CH);
	}
	
	//		currencies															
	#[test]
	fn currencies() {
		assert_eq!(Country::CH.currencies(), &vh![ CurrencyCode: CHE, CHF, CHW ]);
	}
	#[test]
	fn currencies__relationships() {
		#[cfg_attr(    feature = "reasons",  allow(clippy::iter_over_hash_type, reason = "Order is not important here"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::iter_over_hash_type))]
		for country in COUNTRIES.keys() {
			for currency_code in country.currencies() {
				assert!(currency_code.currency().countries().contains(&country.code()));
			}
		}
	}
	
	//		languages															
	#[test]
	fn languages() {
		assert_eq!(Country::CH.languages(), &vh![ LanguageCode: DE, FR, IT, RM ]);
	}
	#[test]
	fn languages__relationships() {
		#[cfg_attr(    feature = "reasons",  allow(clippy::iter_over_hash_type, reason = "Order is not important here"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::iter_over_hash_type))]
		for country in COUNTRIES.keys() {
			for language_code in country.languages() {
				assert!(language_code.language().countries().contains(&country.code()));
			}
		}
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
		assert_eq!(Country::US.as_str(), "United States of America");
	}
	
	//		debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", Country::US), "US: United States of America");
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let country: Country = serde_json::from_str(r#""United States of America""#).unwrap();
		assert_eq!(country, Country::US);
	}
	
	//		display																
	#[test]
	fn display() {
		let country = Country::US;
		assert_eq!(format!("{country}"), "United States of America");
		assert_eq!(country.to_string(),  "United States of America");
	}
	
	//		eq / partial_eq														
	#[test]
	fn eq() {
		assert_eq!(Country::US, Country::US);
	}
	#[test]
	fn ne() {
		assert_ne!(Country::US, Country::GB);
	}
	
	//		from																
	#[test]
	fn from__country_for_string() {
		let country = Country::US;
		assert_eq!(String::from(country), "United States of America");
		let str: String = country.into();
		assert_eq!(str,                   "United States of America");
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_eq!(Country::from_str("United States of America").unwrap(), Country::US);
		let err = Country::from_str("Fooland");
		assert_err!(&err);
		assert_eq!(err.unwrap_err(), "Invalid Country: Fooland");
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		assert_eq!(serde_json::to_string(&Country::US).unwrap(), r#""United States of America""#);
	}
	
	//		try_from															
	#[test]
	fn try_from__string() {
		assert_eq!(Country::from_str("United States of America").unwrap(), Country::US);
		let err = Country::from_str("Fooland");
		assert_err!(&err);
		assert_eq!(err.unwrap_err(), "Invalid Country: Fooland");
	}
}


