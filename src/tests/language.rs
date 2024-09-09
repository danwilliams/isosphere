//		Tests

//		LanguageCode															
#[cfg(test)]
mod language_code__enum {
	use super::super::*;
	
	//		all																	
	#[test]
	fn all() {
		let codes = LanguageCode::all();
		assert_eq!(codes.len(), 183);
		assert!(codes.contains(&LanguageCode::EN));
		assert!(codes.contains(&LanguageCode::FR));
		assert!(codes.contains(&LanguageCode::ES));
	}
	
	//		language															
	#[test]
	fn language() {
		let language = LanguageCode::EN.language();
		assert_eq!(language.name(), "English");
		assert_eq!(language.code(), LanguageCode::EN);
	}
	#[test]
	fn language__all() {
		#[cfg_attr(    feature = "reasons",  allow(clippy::iter_over_hash_type, reason = "Order is not important here"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::iter_over_hash_type))]
		for language in LANGUAGES.keys() {
			assert_eq!(language.code().language(), *language);
		}
	}
}

#[cfg(test)]
mod language_code__traits {
	use super::super::*;
	use claims::assert_err;
	use serde_json;
	
	//		as_str																
	#[test]
	fn as_str() {
		assert_eq!(LanguageCode::EN.as_str(), "en");
	}
	
	//		debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", LanguageCode::EN), "EN");
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let code1: LanguageCode = serde_json::from_str(r#""en""#).unwrap();
		assert_eq!(code1, LanguageCode::EN);
		let code2: LanguageCode = serde_json::from_str(r#""EN""#).unwrap();
		assert_eq!(code2, LanguageCode::EN);
	}
	
	//		display																
	#[test]
	fn display() {
		let code = LanguageCode::EN;
		assert_eq!(format!("{code}"), "en");
		assert_eq!(code.to_string(),  "en");
	}
	
	//		eq / partial_eq														
	#[test]
	fn eq() {
		assert_eq!(LanguageCode::EN, LanguageCode::EN);
	}
	#[test]
	fn ne() {
		assert_ne!(LanguageCode::EN, LanguageCode::FR);
	}
	
	//		from																
	#[test]
	fn from__language_code_for_string() {
		let code = LanguageCode::EN;
		assert_eq!(String::from(code), "en");
		let str: String = code.into();
		assert_eq!(str,                "en");
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_eq!(LanguageCode::from_str("en").unwrap(), LanguageCode::EN);
		assert_eq!(LanguageCode::from_str("EN").unwrap(), LanguageCode::EN);
		let err = LanguageCode::from_str("foo");
		assert_err!(&err);
		assert_eq!(err.unwrap_err(), "Invalid LanguageCode: foo");
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		assert_eq!(serde_json::to_string(&LanguageCode::EN).unwrap(), r#""en""#);
	}
	
	//		try_from															
	#[test]
	fn try_from__string() {
		assert_eq!(LanguageCode::try_from(s!("en")).unwrap(), LanguageCode::EN);
		assert_eq!(LanguageCode::try_from(s!("EN")).unwrap(), LanguageCode::EN);
		let err = LanguageCode::try_from(s!("foo"));
		assert_err!(&err);
		assert_eq!(err.unwrap_err(), "Invalid LanguageCode: foo");
	}
}

//		Language																
#[cfg(test)]
mod language__enum {
	use super::super::*;
	
	//		all																	
	#[test]
	fn all() {
		let languages = Language::all();
		assert_eq!(languages.len(), 183);
		assert!(languages.contains(&Language::EN));
		assert!(languages.contains(&Language::FR));
		assert!(languages.contains(&Language::ES));
	}
	
	//		info																
	#[test]
	fn info() {
		let info = Language::EN.info();
		assert_eq!(info.name, "English");
		assert_eq!(info.code, LanguageCode::EN);
	}
	
	//		name																
	#[test]
	fn name() {
		assert_eq!(Language::NO.name(), "Norwegian");
	}
	
	//		code																
	#[test]
	fn code() {
		assert_eq!(Language::NO.code(), LanguageCode::NO);
	}
	
	//		countries															
	#[test]
	fn countries() {
		assert_eq!(Language::NO.countries(), &vh![ CountryCode: BV, NO, SJ ]);
	}
	#[test]
	fn countries__relationships() {
		#[cfg_attr(    feature = "reasons",  allow(clippy::iter_over_hash_type, reason = "Order is not important here"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::iter_over_hash_type))]
		for language in LANGUAGES.keys() {
			for country_code in language.countries() {
				assert!(country_code.country().languages().contains(&language.code()));
			}
		}
	}
}

#[cfg(test)]
mod language__traits {
	use super::super::*;
	use claims::assert_err;
	use serde_json;
	
	//		as_str																
	#[test]
	fn as_str() {
		assert_eq!(Language::EN.as_str(), "English");
	}
	
	//		debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", Language::EN), "en: English");
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let language: Language = serde_json::from_str(r#""English""#).unwrap();
		assert_eq!(language, Language::EN);
	}
	
	//		display																
	#[test]
	fn display() {
		let language = Language::EN;
		assert_eq!(format!("{language}"), "English");
		assert_eq!(language.to_string(),  "English");
	}
	
	//		eq / partial_eq														
	#[test]
	fn eq() {
		assert_eq!(Language::EN, Language::EN);
	}
	#[test]
	fn ne() {
		assert_ne!(Language::EN, Language::FR);
	}
	
	//		from																
	#[test]
	fn from__language_for_string() {
		let language = Language::EN;
		assert_eq!(String::from(language), "English");
		let str: String = language.into();
		assert_eq!(str,                    "English");
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_eq!(Language::from_str("English").unwrap(), Language::EN);
		let err = Language::from_str("Fooish");
		assert_err!(&err);
		assert_eq!(err.unwrap_err(), "Invalid Language: Fooish");
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		assert_eq!(serde_json::to_string(&Language::EN).unwrap(), r#""English""#);
	}
	
	//		try_from															
	#[test]
	fn try_from__string() {
		assert_eq!(Language::from_str("English").unwrap(), Language::EN);
		let err = Language::from_str("Fooish");
		assert_err!(&err);
		assert_eq!(err.unwrap_err(), "Invalid Language: Fooish");
	}
}


