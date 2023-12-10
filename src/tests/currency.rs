#![allow(non_snake_case)]

//		Tests

//		CurrencyCode															
#[cfg(test)]
mod currency_code__enum {
	use super::super::*;
	
	//		currency															
	#[test]
	fn currency() {
		let currency = CurrencyCode::USD.currency();
		assert_eq!(currency.name(), "United States dollar");
		assert_eq!(currency.code(), CurrencyCode::USD);
	}
	#[test]
	fn currency__all() {
		for currency in CURRENCIES.keys() {
			assert_eq!(currency.code().currency(), *currency);
		}
	}
}

#[cfg(test)]
mod currency_code__traits {
	use super::super::*;
	use claims::assert_err;
	use serde_json;
	
	//		as_str																
	#[test]
	fn as_str() {
		assert_eq!(CurrencyCode::USD.as_str(), "USD");
	}
	
	//		debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", CurrencyCode::USD), "USD");
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let code: CurrencyCode = serde_json::from_str(r#""USD""#).unwrap();
		assert_eq!(code, CurrencyCode::USD);
		let code: CurrencyCode = serde_json::from_str(r#""usd""#).unwrap();
		assert_eq!(code, CurrencyCode::USD);
	}
	
	//		display																
	#[test]
	fn display() {
		let code = CurrencyCode::USD;
		assert_eq!(format!("{}", code), "USD");
		assert_eq!(code.to_string(),    "USD");
	}
	
	//		eq / partial_eq														
	#[test]
	fn eq() {
		assert_eq!(CurrencyCode::USD, CurrencyCode::USD);
	}
	#[test]
	fn ne() {
		assert_ne!(CurrencyCode::USD, CurrencyCode::EUR);
	}
	
	//		from																
	#[test]
	fn from__currency_code_for_u16() {
		let code = CurrencyCode::USD;
		assert_eq!(u16::from(code), 840);
		assert_eq!(code as u16,     840);
		let int: u16 = code.into();
		assert_eq!(int,             840);
	}
	#[test]
	fn from__currency_code_for_string() {
		let code = CurrencyCode::USD;
		assert_eq!(String::from(code), "USD");
		let str: String = code.into();
		assert_eq!(str,                "USD");
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_eq!(CurrencyCode::from_str("USD").unwrap(), CurrencyCode::USD);
		assert_eq!(CurrencyCode::from_str("usd").unwrap(), CurrencyCode::USD);
		let err = CurrencyCode::from_str("FOO");
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid CurrencyCode: FOO");
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		assert_eq!(serde_json::to_string(&CurrencyCode::USD).unwrap(), r#""USD""#);
	}
	
	//		try_from															
	#[test]
	fn try_from__u16() {
		assert_eq!(CurrencyCode::try_from(840).unwrap(), CurrencyCode::USD);
		let err = CurrencyCode::try_from(000);
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid CurrencyCode: 0");
	}
	#[test]
	fn try_from__string() {
		assert_eq!(CurrencyCode::try_from(s!("USD")).unwrap(), CurrencyCode::USD);
		assert_eq!(CurrencyCode::try_from(s!("usd")).unwrap(), CurrencyCode::USD);
		let err = CurrencyCode::try_from(s!("FOO"));
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid CurrencyCode: FOO");
	}
}

//		Currency																
#[cfg(test)]
mod currency__enum {
	use super::super::*;
	
	//		info																
	#[test]
	fn info() {
		let info = Currency::USD.info();
		assert_eq!(info.name, "United States dollar");
		assert_eq!(info.code, CurrencyCode::USD);
	}
	
	//		name																
	#[test]
	fn name() {
		assert_eq!(Currency::GBP.name(), "Pound sterling");
	}
	
	//		code																
	#[test]
	fn code() {
		assert_eq!(Currency::GBP.code(), CurrencyCode::GBP);
	}
	
	//		digits																
	#[test]
	fn digits() {
		assert_eq!(Currency::GBP.digits(), 2);
	}
	
	//		countries															
	#[test]
	fn countries() {
		assert_eq!(Currency::GBP.countries(), &vh![ CountryCode: GB, GG, IM, JE, SH ]);
	}
	#[test]
	fn countries__relationships() {
		for currency in CURRENCIES.keys() {
			for country_code in currency.countries().iter() {
				assert!(country_code.country().currencies().contains(&currency.code()));
			}
		}
	}
}

#[cfg(test)]
mod currency__traits {
	use super::super::*;
	use claims::assert_err;
	use serde_json;
	
	//		as_str																
	#[test]
	fn as_str() {
		assert_eq!(Currency::USD.as_str(), "United States dollar");
	}
	
	//		debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", Currency::USD), "USD: United States dollar");
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let currency: Currency = serde_json::from_str(r#""United States dollar""#).unwrap();
		assert_eq!(currency, Currency::USD);
	}
	
	//		display																
	#[test]
	fn display() {
		let currency = Currency::USD;
		assert_eq!(format!("{}", currency), "United States dollar");
		assert_eq!(currency.to_string(),    "United States dollar");
	}
	
	//		eq / partial_eq														
	#[test]
	fn eq() {
		assert_eq!(Currency::USD, Currency::USD);
	}
	#[test]
	fn ne() {
		assert_ne!(Currency::USD, Currency::EUR);
	}
	
	//		from																
	#[test]
	fn from__currency_for_string() {
		let currency = Currency::USD;
		assert_eq!(String::from(currency.clone()), "United States dollar");
		let str: String = currency.clone().into();
		assert_eq!(str,                            "United States dollar");
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_eq!(Currency::from_str("United States dollar").unwrap(), Currency::USD);
		let err = Currency::from_str("Foo dollar");
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid Currency: Foo dollar");
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		assert_eq!(serde_json::to_string(&Currency::USD).unwrap(), r#""United States dollar""#);
	}
	
	//		try_from															
	#[test]
	fn try_from__string() {
		assert_eq!(Currency::from_str("United States dollar").unwrap(), Currency::USD);
		let err = Currency::from_str("Foo dollar");
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid Currency: Foo dollar");
	}
}


