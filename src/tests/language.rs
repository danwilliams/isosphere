#![allow(non_snake_case)]

//		Tests

//		LanguageCode															
#[cfg(test)]
mod language_code__enum {
	use super::super::*;
	
	//		as_str																
	#[test]
	fn as_str() {
		assert_eq!(LanguageCode::EN.as_str(), "en");
	}
	
	//		language															
	#[test]
	fn language() {
		let language = LanguageCode::EN.language();
		assert_eq!(language.name, "English");
		assert_eq!(language.code, LanguageCode::EN);
	}
	#[test]
	fn language__all() {
		for code in LANGUAGES.keys() {
			assert_eq!(code.language().code, *code);
		}
	}
	#[test]
	fn language__relationships() {
		for language_code in LANGUAGES.keys() {
			let language = language_code.language();
			for country_code in language.countries.iter() {
				assert!(country_code.country().languages.contains(language_code));
			}
		}
	}
}

#[cfg(test)]
mod language_code__traits {
	use super::super::*;
	use claims::assert_err;
	use serde_json;
	
	//		debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", LanguageCode::EN), "EN");
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let code: LanguageCode = serde_json::from_str(r#""en""#).unwrap();
		assert_eq!(code, LanguageCode::EN);
	}
	
	//		display																
	#[test]
	fn display() {
		let code = LanguageCode::EN;
		assert_eq!(format!("{}", code), "en");
		assert_eq!(code.to_string(),    "en");
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
		let err = LanguageCode::from_str("foo");
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid LanguageCode: foo");
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
		let err = LanguageCode::try_from(s!("foo"));
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid LanguageCode: foo");
	}
}

//		Language																
#[cfg(test)]
mod language__struct {
	use super::super::*;
	
	//		as_str																
	#[test]
	fn as_str() {
		assert_eq!(LanguageCode::EN.language().as_str(), "English");
	}
}

#[cfg(test)]
mod language__traits {
	use super::super::*;
	use claims::assert_err;
	use serde_json;
	
	//		debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", LanguageCode::EN.language()), "en: English");
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let language: Language = serde_json::from_str(r#""English""#).unwrap();
		assert_eq!(language, *LanguageCode::EN.language());
	}
	
	//		display																
	#[test]
	fn display() {
		let language = LanguageCode::EN.language();
		assert_eq!(format!("{}", language), "English");
		assert_eq!(language.to_string(),    "English");
	}
	
	//		eq / partial_eq														
	#[test]
	fn eq() {
		assert_eq!(LanguageCode::EN.language(), LanguageCode::EN.language());
	}
	#[test]
	fn ne() {
		assert_ne!(LanguageCode::EN.language(), LanguageCode::FR.language());
	}
	
	//		from																
	#[test]
	fn from__language_for_string() {
		let language = LanguageCode::EN.language();
		assert_eq!(String::from(language.clone()), "English");
		let str: String = language.clone().into();
		assert_eq!(str,                            "English");
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_eq!(Language::from_str("English").unwrap(), *LanguageCode::EN.language());
		let err = Language::from_str("Fooish");
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid Language: Fooish");
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		assert_eq!(serde_json::to_string(LanguageCode::EN.language()).unwrap(), r#""English""#);
	}
	
	//		try_from															
	#[test]
	fn try_from__string() {
		assert_eq!(Language::from_str("English").unwrap(), *LanguageCode::EN.language());
		let err = Language::from_str("Fooish");
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid Language: Fooish");
	}
}


