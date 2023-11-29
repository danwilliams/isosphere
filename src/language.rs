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
/// The possible languages' codes.
/// 
/// These codes are based on the ISO 639 standard, specifically ISO 639-1, which
/// defines codes of two and three characters to represent languages. There are
/// only alphabetic codes, using two letters.
/// 
/// # Alphabetic codes
/// 
/// The alphabetic codes are defined by the ISO 639-1 set. They are the most
/// widely-used codes from the ISO 639 standard. ISO 639 also has three-letter
/// codes as part of ISO 639-2, 639-3, and 639-5, but these are not supported at
/// present.
/// 
/// # Data sources
/// 
/// The list of codes is available from [the ISO site](https://www.iso.org/iso-639-language-code),
/// and from [Wikipedia](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).
/// 
/// # See also
/// 
/// * [`Language`]
/// 

#[cfg_attr(    feature = "reasons",  allow(clippy::upper_case_acronyms, reason = "Uppercase is suitable here"))]
#[cfg_attr(not(feature = "reasons"), allow(clippy::upper_case_acronyms))]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ToSchema)]
#[serde(into = "String", try_from = "String")]
#[non_exhaustive]
pub enum LanguageCode {
	/// Afar
	AA,
	
	/// Abkhazian
	AB,
	
	/// Avestan
	AE,
	
	/// Afrikaans
	AF,
	
	/// Akan
	AK,
	
	/// Amharic
	AM,
	
	/// Aragonese
	AN,
	
	/// Arabic
	AR,
	
	/// Assamese
	AS,
	
	/// Avaric
	AV,
	
	/// Aymara
	AY,
	
	/// Azerbaijani
	AZ,
	
	/// Bashkir
	BA,
	
	/// Belarusian
	BE,
	
	/// Bulgarian
	BG,
	
	/// Bislama
	BI,
	
	/// Bambara
	BM,
	
	/// Bengali
	BN,
	
	/// Tibetan
	BO,
	
	/// Breton
	BR,
	
	/// Bosnian
	BS,
	
	/// Catalan, Valencian
	CA,
	
	/// Chechen
	CE,
	
	/// Chamorro
	CH,
	
	/// Corsican
	CO,
	
	/// Cree
	CR,
	
	/// Czech
	CS,
	
	/// Church Slavonic, Old Slavonic, Old Church Slavonic
	CU,
	
	/// Chuvash
	CV,
	
	/// Welsh
	CY,
	
	/// Danish
	DA,
	
	/// German
	DE,
	
	/// Divehi, Dhivehi, Maldivian
	DV,
	
	/// Dzongkha
	DZ,
	
	/// Ewe
	EE,
	
	/// Greek, Modern (1453–)
	EL,
	
	/// English
	EN,
	
	/// Esperanto
	EO,
	
	/// Spanish, Castilian
	ES,
	
	/// Estonian
	ET,
	
	/// Basque
	EU,
	
	/// Persian
	FA,
	
	/// Fulah
	FF,
	
	/// Finnish
	FI,
	
	/// Fijian
	FJ,
	
	/// Faroese
	FO,
	
	/// French
	FR,
	
	/// Western Frisian
	FY,
	
	/// Irish
	GA,
	
	/// Gaelic, Scottish Gaelic
	GD,
	
	/// Galician
	GL,
	
	/// Guarani
	GN,
	
	/// Gujarati
	GU,
	
	/// Manx
	GV,
	
	/// Hausa
	HA,
	
	/// Hebrew
	HE,
	
	/// Hindi
	HI,
	
	/// Hiri Motu
	HO,
	
	/// Croatian
	HR,
	
	/// Haitian, Haitian Creole
	HT,
	
	/// Hungarian
	HU,
	
	/// Armenian
	HY,
	
	/// Herero
	HZ,
	
	/// Interlingua (International Auxiliary Language Association)
	IA,
	
	/// Indonesian
	ID,
	
	/// Interlingue, Occidental
	IE,
	
	/// Igbo
	IG,
	
	/// Sichuan Yi, Nuosu
	II,
	
	/// Inupiaq
	IK,
	
	/// Ido
	IO,
	
	/// Icelandic
	IS,
	
	/// Italian
	IT,
	
	/// Inuktitut
	IU,
	
	/// Japanese
	JA,
	
	/// Javanese
	JV,
	
	/// Georgian
	KA,
	
	/// Kongo
	KG,
	
	/// Kikuyu, Gikuyu
	KI,
	
	/// Kuanyama, Kwanyama
	KJ,
	
	/// Kazakh
	KK,
	
	/// Kalaallisut, Greenlandic
	KL,
	
	/// Central Khmer
	KM,
	
	/// Kannada
	KN,
	
	/// Korean
	KO,
	
	/// Kanuri
	KR,
	
	/// Kashmiri
	KS,
	
	/// Kurdish
	KU,
	
	/// Komi
	KV,
	
	/// Cornish
	KW,
	
	/// Kirghiz, Kyrgyz
	KY,
	
	/// Latin
	LA,
	
	/// Luxembourgish, Letzeburgesch
	LB,
	
	/// Ganda
	LG,
	
	/// Limburgan, Limburger, Limburgish
	LI,
	
	/// Lingala
	LN,
	
	/// Lao
	LO,
	
	/// Lithuanian
	LT,
	
	/// Luba-Katanga
	LU,
	
	/// Latvian
	LV,
	
	/// Malagasy
	MG,
	
	/// Marshallese
	MH,
	
	/// Maori
	MI,
	
	/// Macedonian
	MK,
	
	/// Malayalam
	ML,
	
	/// Mongolian
	MN,
	
	/// Marathi
	MR,
	
	/// Malay
	MS,
	
	/// Maltese
	MT,
	
	/// Burmese
	MY,
	
	/// Nauru
	NA,
	
	/// Norwegian Bokmål
	NB,
	
	/// North Ndebele
	ND,
	
	/// Nepali
	NE,
	
	/// Ndonga
	NG,
	
	/// Dutch, Flemish
	NL,
	
	/// Norwegian Nynorsk
	NN,
	
	/// Norwegian
	NO,
	
	/// South Ndebele
	NR,
	
	/// Navajo, Navaho
	NV,
	
	/// Chichewa, Chewa, Nyanja
	NY,
	
	/// Occitan
	OC,
	
	/// Ojibwa
	OJ,
	
	/// Oromo
	OM,
	
	/// Oriya
	OR,
	
	/// Ossetian, Ossetic
	OS,
	
	/// Punjabi, Panjabi
	PA,
	
	/// Pali
	PI,
	
	/// Polish
	PL,
	
	/// Pashto, Pushto
	PS,
	
	/// Portuguese
	PT,
	
	/// Quechua
	QU,
	
	/// Romansh
	RM,
	
	/// Rundi
	RN,
	
	/// Romanian, Moldavian, Moldovan
	RO,
	
	/// Russian
	RU,
	
	/// Kinyarwanda
	RW,
	
	/// Sanskrit
	SA,
	
	/// Sardinian
	SC,
	
	/// Sindhi
	SD,
	
	/// Northern Sami
	SE,
	
	/// Sango
	SG,
	
	/// Sinhala, Sinhalese
	SI,
	
	/// Slovak
	SK,
	
	/// Slovenian
	SL,
	
	/// Samoan
	SM,
	
	/// Shona
	SN,
	
	/// Somali
	SO,
	
	/// Albanian
	SQ,
	
	/// Serbian
	SR,
	
	/// Swati
	SS,
	
	/// Southern Sotho
	ST,
	
	/// Sundanese
	SU,
	
	/// Swedish
	SV,
	
	/// Swahili
	SW,
	
	/// Tamil
	TA,
	
	/// Telugu
	TE,
	
	/// Tajik
	TG,
	
	/// Thai
	TH,
	
	/// Tigrinya
	TI,
	
	/// Turkmen
	TK,
	
	/// Tagalog
	TL,
	
	/// Tswana
	TN,
	
	/// Tonga (Tonga Islands)
	TO,
	
	/// Turkish
	TR,
	
	/// Tsonga
	TS,
	
	/// Tatar
	TT,
	
	/// Twi
	TW,
	
	/// Tahitian
	TY,
	
	/// Uighur, Uyghur
	UG,
	
	/// Ukrainian
	UK,
	
	/// Urdu
	UR,
	
	/// Uzbek
	UZ,
	
	/// Venda
	VE,
	
	/// Vietnamese
	VI,
	
	/// Volapük
	VO,
	
	/// Walloon
	WA,
	
	/// Wolof
	WO,
	
	/// Xhosa
	XH,
	
	/// Yiddish
	YI,
	
	/// Yoruba
	YO,
	
	/// Zhuang, Chuang
	ZA,
	
	/// Chinese
	ZH,
	
	/// Zulu
	ZU,
}

impl LanguageCode {
	//		as_str																
	/// Returns a string representation of the `LanguageCode` variant.
	/// 
	/// This method provides a way to obtain a static string slice corresponding
	/// to a variant of the `LanguageCode` enum. The returned string slice is
	/// suitable for use in scenarios where a string representation of the enum
	/// variant is needed, such as serialization or logging.
	/// 
	/// It is potentially different from the [`Display`] implementation, which
	/// returns a human-readable string representation of the enum variant, and
	/// the [`Debug`] implementation, which returns a string representation of
	/// the enum variant that is suitable for debugging purposes.
	/// 
	#[cfg_attr(    feature = "reasons",  allow(clippy::too_many_lines, reason = "Data not logic"))]
	#[cfg_attr(not(feature = "reasons"), allow(clippy::too_many_lines))]
	#[must_use]
	pub const fn as_str(&self) -> &'static str {
		match *self {
			Self::AA => "aa",
			Self::AB => "ab",
			Self::AE => "ae",
			Self::AF => "af",
			Self::AK => "ak",
			Self::AM => "am",
			Self::AN => "an",
			Self::AR => "ar",
			Self::AS => "as",
			Self::AV => "av",
			Self::AY => "ay",
			Self::AZ => "az",
			Self::BA => "ba",
			Self::BE => "be",
			Self::BG => "bg",
			Self::BI => "bi",
			Self::BM => "bm",
			Self::BN => "bn",
			Self::BO => "bo",
			Self::BR => "br",
			Self::BS => "bs",
			Self::CA => "ca",
			Self::CE => "ce",
			Self::CH => "ch",
			Self::CO => "co",
			Self::CR => "cr",
			Self::CS => "cs",
			Self::CU => "cu",
			Self::CV => "cv",
			Self::CY => "cy",
			Self::DA => "da",
			Self::DE => "de",
			Self::DV => "dv",
			Self::DZ => "dz",
			Self::EE => "ee",
			Self::EL => "el",
			Self::EN => "en",
			Self::EO => "eo",
			Self::ES => "es",
			Self::ET => "et",
			Self::EU => "eu",
			Self::FA => "fa",
			Self::FF => "ff",
			Self::FI => "fi",
			Self::FJ => "fj",
			Self::FO => "fo",
			Self::FR => "fr",
			Self::FY => "fy",
			Self::GA => "ga",
			Self::GD => "gd",
			Self::GL => "gl",
			Self::GN => "gn",
			Self::GU => "gu",
			Self::GV => "gv",
			Self::HA => "ha",
			Self::HE => "he",
			Self::HI => "hi",
			Self::HO => "ho",
			Self::HR => "hr",
			Self::HT => "ht",
			Self::HU => "hu",
			Self::HY => "hy",
			Self::HZ => "hz",
			Self::IA => "ia",
			Self::ID => "id",
			Self::IE => "ie",
			Self::IG => "ig",
			Self::II => "ii",
			Self::IK => "ik",
			Self::IO => "io",
			Self::IS => "is",
			Self::IT => "it",
			Self::IU => "iu",
			Self::JA => "ja",
			Self::JV => "jv",
			Self::KA => "ka",
			Self::KG => "kg",
			Self::KI => "ki",
			Self::KJ => "kj",
			Self::KK => "kk",
			Self::KL => "kl",
			Self::KM => "km",
			Self::KN => "kn",
			Self::KO => "ko",
			Self::KR => "kr",
			Self::KS => "ks",
			Self::KU => "ku",
			Self::KV => "kv",
			Self::KW => "kw",
			Self::KY => "ky",
			Self::LA => "la",
			Self::LB => "lb",
			Self::LG => "lg",
			Self::LI => "li",
			Self::LN => "ln",
			Self::LO => "lo",
			Self::LT => "lt",
			Self::LU => "lu",
			Self::LV => "lv",
			Self::MG => "mg",
			Self::MH => "mh",
			Self::MI => "mi",
			Self::MK => "mk",
			Self::ML => "ml",
			Self::MN => "mn",
			Self::MR => "mr",
			Self::MS => "ms",
			Self::MT => "mt",
			Self::MY => "my",
			Self::NA => "na",
			Self::NB => "nb",
			Self::ND => "nd",
			Self::NE => "ne",
			Self::NG => "ng",
			Self::NL => "nl",
			Self::NN => "nn",
			Self::NO => "no",
			Self::NR => "nr",
			Self::NV => "nv",
			Self::NY => "ny",
			Self::OC => "oc",
			Self::OJ => "oj",
			Self::OM => "om",
			Self::OR => "or",
			Self::OS => "os",
			Self::PA => "pa",
			Self::PI => "pi",
			Self::PL => "pl",
			Self::PS => "ps",
			Self::PT => "pt",
			Self::QU => "qu",
			Self::RM => "rm",
			Self::RN => "rn",
			Self::RO => "ro",
			Self::RU => "ru",
			Self::RW => "rw",
			Self::SA => "sa",
			Self::SC => "sc",
			Self::SD => "sd",
			Self::SE => "se",
			Self::SG => "sg",
			Self::SI => "si",
			Self::SK => "sk",
			Self::SL => "sl",
			Self::SM => "sm",
			Self::SN => "sn",
			Self::SO => "so",
			Self::SQ => "sq",
			Self::SR => "sr",
			Self::SS => "ss",
			Self::ST => "st",
			Self::SU => "su",
			Self::SV => "sv",
			Self::SW => "sw",
			Self::TA => "ta",
			Self::TE => "te",
			Self::TG => "tg",
			Self::TH => "th",
			Self::TI => "ti",
			Self::TK => "tk",
			Self::TL => "tl",
			Self::TN => "tn",
			Self::TO => "to",
			Self::TR => "tr",
			Self::TS => "ts",
			Self::TT => "tt",
			Self::TW => "tw",
			Self::TY => "ty",
			Self::UG => "ug",
			Self::UK => "uk",
			Self::UR => "ur",
			Self::UZ => "uz",
			Self::VE => "ve",
			Self::VI => "vi",
			Self::VO => "vo",
			Self::WA => "wa",
			Self::WO => "wo",
			Self::XH => "xh",
			Self::YI => "yi",
			Self::YO => "yo",
			Self::ZA => "za",
			Self::ZH => "zh",
			Self::ZU => "zu",
		}
	}
}

impl Display for LanguageCode {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl From<LanguageCode> for String {
	//		from																
	fn from(code: LanguageCode) -> Self {
		code.to_string()
	}
}

impl FromStr for LanguageCode {
	type Err = String;
	
	//		from_str															
	#[cfg_attr(    feature = "reasons",  allow(clippy::too_many_lines, reason = "Data not logic"))]
	#[cfg_attr(not(feature = "reasons"), allow(clippy::too_many_lines))]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"aa" => Ok(Self::AA),
			"ab" => Ok(Self::AB),
			"ae" => Ok(Self::AE),
			"af" => Ok(Self::AF),
			"ak" => Ok(Self::AK),
			"am" => Ok(Self::AM),
			"an" => Ok(Self::AN),
			"ar" => Ok(Self::AR),
			"as" => Ok(Self::AS),
			"av" => Ok(Self::AV),
			"ay" => Ok(Self::AY),
			"az" => Ok(Self::AZ),
			"ba" => Ok(Self::BA),
			"be" => Ok(Self::BE),
			"bg" => Ok(Self::BG),
			"bi" => Ok(Self::BI),
			"bm" => Ok(Self::BM),
			"bn" => Ok(Self::BN),
			"bo" => Ok(Self::BO),
			"br" => Ok(Self::BR),
			"bs" => Ok(Self::BS),
			"ca" => Ok(Self::CA),
			"ce" => Ok(Self::CE),
			"ch" => Ok(Self::CH),
			"co" => Ok(Self::CO),
			"cr" => Ok(Self::CR),
			"cs" => Ok(Self::CS),
			"cu" => Ok(Self::CU),
			"cv" => Ok(Self::CV),
			"cy" => Ok(Self::CY),
			"da" => Ok(Self::DA),
			"de" => Ok(Self::DE),
			"dv" => Ok(Self::DV),
			"dz" => Ok(Self::DZ),
			"ee" => Ok(Self::EE),
			"el" => Ok(Self::EL),
			"en" => Ok(Self::EN),
			"eo" => Ok(Self::EO),
			"es" => Ok(Self::ES),
			"et" => Ok(Self::ET),
			"eu" => Ok(Self::EU),
			"fa" => Ok(Self::FA),
			"ff" => Ok(Self::FF),
			"fi" => Ok(Self::FI),
			"fj" => Ok(Self::FJ),
			"fo" => Ok(Self::FO),
			"fr" => Ok(Self::FR),
			"fy" => Ok(Self::FY),
			"ga" => Ok(Self::GA),
			"gd" => Ok(Self::GD),
			"gl" => Ok(Self::GL),
			"gn" => Ok(Self::GN),
			"gu" => Ok(Self::GU),
			"gv" => Ok(Self::GV),
			"ha" => Ok(Self::HA),
			"he" => Ok(Self::HE),
			"hi" => Ok(Self::HI),
			"ho" => Ok(Self::HO),
			"hr" => Ok(Self::HR),
			"ht" => Ok(Self::HT),
			"hu" => Ok(Self::HU),
			"hy" => Ok(Self::HY),
			"hz" => Ok(Self::HZ),
			"ia" => Ok(Self::IA),
			"id" => Ok(Self::ID),
			"ie" => Ok(Self::IE),
			"ig" => Ok(Self::IG),
			"ii" => Ok(Self::II),
			"ik" => Ok(Self::IK),
			"io" => Ok(Self::IO),
			"is" => Ok(Self::IS),
			"it" => Ok(Self::IT),
			"iu" => Ok(Self::IU),
			"ja" => Ok(Self::JA),
			"jv" => Ok(Self::JV),
			"ka" => Ok(Self::KA),
			"kg" => Ok(Self::KG),
			"ki" => Ok(Self::KI),
			"kj" => Ok(Self::KJ),
			"kk" => Ok(Self::KK),
			"kl" => Ok(Self::KL),
			"km" => Ok(Self::KM),
			"kn" => Ok(Self::KN),
			"ko" => Ok(Self::KO),
			"kr" => Ok(Self::KR),
			"ks" => Ok(Self::KS),
			"ku" => Ok(Self::KU),
			"kv" => Ok(Self::KV),
			"kw" => Ok(Self::KW),
			"ky" => Ok(Self::KY),
			"la" => Ok(Self::LA),
			"lb" => Ok(Self::LB),
			"lg" => Ok(Self::LG),
			"li" => Ok(Self::LI),
			"ln" => Ok(Self::LN),
			"lo" => Ok(Self::LO),
			"lt" => Ok(Self::LT),
			"lu" => Ok(Self::LU),
			"lv" => Ok(Self::LV),
			"mg" => Ok(Self::MG),
			"mh" => Ok(Self::MH),
			"mi" => Ok(Self::MI),
			"mk" => Ok(Self::MK),
			"ml" => Ok(Self::ML),
			"mn" => Ok(Self::MN),
			"mr" => Ok(Self::MR),
			"ms" => Ok(Self::MS),
			"mt" => Ok(Self::MT),
			"my" => Ok(Self::MY),
			"na" => Ok(Self::NA),
			"nb" => Ok(Self::NB),
			"nd" => Ok(Self::ND),
			"ne" => Ok(Self::NE),
			"ng" => Ok(Self::NG),
			"nl" => Ok(Self::NL),
			"nn" => Ok(Self::NN),
			"no" => Ok(Self::NO),
			"nr" => Ok(Self::NR),
			"nv" => Ok(Self::NV),
			"ny" => Ok(Self::NY),
			"oc" => Ok(Self::OC),
			"oj" => Ok(Self::OJ),
			"om" => Ok(Self::OM),
			"or" => Ok(Self::OR),
			"os" => Ok(Self::OS),
			"pa" => Ok(Self::PA),
			"pi" => Ok(Self::PI),
			"pl" => Ok(Self::PL),
			"ps" => Ok(Self::PS),
			"pt" => Ok(Self::PT),
			"qu" => Ok(Self::QU),
			"rm" => Ok(Self::RM),
			"rn" => Ok(Self::RN),
			"ro" => Ok(Self::RO),
			"ru" => Ok(Self::RU),
			"rw" => Ok(Self::RW),
			"sa" => Ok(Self::SA),
			"sc" => Ok(Self::SC),
			"sd" => Ok(Self::SD),
			"se" => Ok(Self::SE),
			"sg" => Ok(Self::SG),
			"si" => Ok(Self::SI),
			"sk" => Ok(Self::SK),
			"sl" => Ok(Self::SL),
			"sm" => Ok(Self::SM),
			"sn" => Ok(Self::SN),
			"so" => Ok(Self::SO),
			"sq" => Ok(Self::SQ),
			"sr" => Ok(Self::SR),
			"ss" => Ok(Self::SS),
			"st" => Ok(Self::ST),
			"su" => Ok(Self::SU),
			"sv" => Ok(Self::SV),
			"sw" => Ok(Self::SW),
			"ta" => Ok(Self::TA),
			"te" => Ok(Self::TE),
			"tg" => Ok(Self::TG),
			"th" => Ok(Self::TH),
			"ti" => Ok(Self::TI),
			"tk" => Ok(Self::TK),
			"tl" => Ok(Self::TL),
			"tn" => Ok(Self::TN),
			"to" => Ok(Self::TO),
			"tr" => Ok(Self::TR),
			"ts" => Ok(Self::TS),
			"tt" => Ok(Self::TT),
			"tw" => Ok(Self::TW),
			"ty" => Ok(Self::TY),
			"ug" => Ok(Self::UG),
			"uk" => Ok(Self::UK),
			"ur" => Ok(Self::UR),
			"uz" => Ok(Self::UZ),
			"ve" => Ok(Self::VE),
			"vi" => Ok(Self::VI),
			"vo" => Ok(Self::VO),
			"wa" => Ok(Self::WA),
			"wo" => Ok(Self::WO),
			"xh" => Ok(Self::XH),
			"yi" => Ok(Self::YI),
			"yo" => Ok(Self::YO),
			"za" => Ok(Self::ZA),
			"zh" => Ok(Self::ZH),
			"zu" => Ok(Self::ZU),
			_     => Err(format!("Invalid LanguageCode: {s}")),
		}
	}
}

impl TryFrom<String> for LanguageCode {
	type Error = String;
	
	//		try_from															
	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.as_str().parse()
	}
}



//		Structs

//		Language																
/// A language.
/// 
/// A language has a number of properties, including a name, a language code,
/// and the countries where the language is used.
/// 
/// Each language is identified by a country code, which can be expressed as two
/// letters, as defined by the ISO 639-1 standard.
/// 
/// # Data sources
/// 
/// The list of codes and other country information is available from
/// [the ISO site](https://www.iso.org/iso-639-language-code), and from
/// [Wikipedia](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).
/// 
/// # See also
/// 
/// * [`LanguageCode`]
/// 
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[non_exhaustive]
pub struct Language {
	//		Public properties													
	/// The name of the language.
	pub name:      String,
	
	/// The language code. For more information, see [`LanguageCode`].
	pub code:      LanguageCode,
	
	/// The countries where the language is used.
	pub countries: HashSet<CountryCode>,
}


