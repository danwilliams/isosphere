//! Language-related types.



//		Modules

#[cfg(test)]
#[path = "tests/language.rs"]
mod tests;



//		Packages

#[cfg_attr(    feature = "reasons",  allow(clippy::enum_glob_use, reason = "Brevity wins here"))]
#[cfg_attr(not(feature = "reasons"), allow(clippy::enum_glob_use))]
use crate::country::{CountryCode, CountryCode::*};
use core::{
	fmt::{Debug, Display, self},
	str::FromStr,
};
use once_cell::sync::Lazy;
use rubedo::{
	std::AsStr,
	sugar::s,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};
use std::collections::{HashMap, HashSet};
use utoipa::ToSchema;
use velcro::{hash_map, hash_set};



//		Constants

/// The possible languages.
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
/// * [`Language`]
/// 
pub static LANGUAGES: Lazy<HashMap<LanguageCode, Language>> = Lazy::new(|| {
	hash_map!{
		LanguageCode::AA: Language { code: LanguageCode::AA, name: s!("Afar"),              countries: hash_set![ ET ] },
		LanguageCode::AB: Language { code: LanguageCode::AB, name: s!("Abkhazian"),         countries: hash_set![] },
		LanguageCode::AE: Language { code: LanguageCode::AE, name: s!("Avestan"),           countries: hash_set![] },
		LanguageCode::AF: Language { code: LanguageCode::AF, name: s!("Afrikaans"),         countries: hash_set![ ZA ] },
		LanguageCode::AK: Language { code: LanguageCode::AK, name: s!("Akan"),              countries: hash_set![] },
		LanguageCode::AM: Language { code: LanguageCode::AM, name: s!("Amharic"),           countries: hash_set![ ET ] },
		LanguageCode::AN: Language { code: LanguageCode::AN, name: s!("Aragonese"),         countries: hash_set![] },
		LanguageCode::AR: Language { code: LanguageCode::AR, name: s!("Arabic"),            countries: hash_set![ AE, BH, DJ, DZ, EG, EH, IQ, JO, KM, KW, LB, LY, MA, MR, OM, PS, QA, SA, SD, SO, SY, TD, TN, YE ] },
		LanguageCode::AS: Language { code: LanguageCode::AS, name: s!("Assamese"),          countries: hash_set![] },
		LanguageCode::AV: Language { code: LanguageCode::AV, name: s!("Avaric"),            countries: hash_set![] },
		LanguageCode::AY: Language { code: LanguageCode::AY, name: s!("Aymara"),            countries: hash_set![ BO, PE ] },
		LanguageCode::AZ: Language { code: LanguageCode::AZ, name: s!("Azerbaijani"),       countries: hash_set![ AZ ] },
		LanguageCode::BA: Language { code: LanguageCode::BA, name: s!("Bashkir"),           countries: hash_set![] },
		LanguageCode::BE: Language { code: LanguageCode::BE, name: s!("Belarusian"),        countries: hash_set![ BY ] },
		LanguageCode::BG: Language { code: LanguageCode::BG, name: s!("Bulgarian"),         countries: hash_set![ BG ] },
		LanguageCode::BI: Language { code: LanguageCode::BI, name: s!("Bislama"),           countries: hash_set![ VU ] },
		LanguageCode::BM: Language { code: LanguageCode::BM, name: s!("Bambara"),           countries: hash_set![ ML ] },
		LanguageCode::BN: Language { code: LanguageCode::BN, name: s!("Bengali"),           countries: hash_set![ BD ] },
		LanguageCode::BO: Language { code: LanguageCode::BO, name: s!("Tibetan"),           countries: hash_set![] },
		LanguageCode::BR: Language { code: LanguageCode::BR, name: s!("Breton"),            countries: hash_set![] },
		LanguageCode::BS: Language { code: LanguageCode::BS, name: s!("Bosnian"),           countries: hash_set![ BA ] },
		LanguageCode::CA: Language { code: LanguageCode::CA, name: s!("Catalan"),           countries: hash_set![ AD ] },
		LanguageCode::CE: Language { code: LanguageCode::CE, name: s!("Chechen"),           countries: hash_set![] },
		LanguageCode::CH: Language { code: LanguageCode::CH, name: s!("Chamorro"),          countries: hash_set![ GU, MP ] },
		LanguageCode::CO: Language { code: LanguageCode::CO, name: s!("Corsican"),          countries: hash_set![] },
		LanguageCode::CR: Language { code: LanguageCode::CR, name: s!("Cree"),              countries: hash_set![] },
		LanguageCode::CS: Language { code: LanguageCode::CS, name: s!("Czech"),             countries: hash_set![ CZ ] },
		LanguageCode::CU: Language { code: LanguageCode::CU, name: s!("Church Slavonic"),   countries: hash_set![] },
		LanguageCode::CV: Language { code: LanguageCode::CV, name: s!("Chuvash"),           countries: hash_set![] },
		LanguageCode::CY: Language { code: LanguageCode::CY, name: s!("Welsh"),             countries: hash_set![] },
		LanguageCode::DA: Language { code: LanguageCode::DA, name: s!("Danish"),            countries: hash_set![ DK, FO, GL ] },
		LanguageCode::DE: Language { code: LanguageCode::DE, name: s!("German"),            countries: hash_set![ AT, BE, CH, DE, LI, LU ] },
		LanguageCode::DV: Language { code: LanguageCode::DV, name: s!("Divehi"),            countries: hash_set![ MV ] },
		LanguageCode::DZ: Language { code: LanguageCode::DZ, name: s!("Dzongkha"),          countries: hash_set![ BT ] },
		LanguageCode::EE: Language { code: LanguageCode::EE, name: s!("Ewe"),               countries: hash_set![] },
		LanguageCode::EL: Language { code: LanguageCode::EL, name: s!("Greek"),             countries: hash_set![ CY, GR ] },
		LanguageCode::EN: Language { code: LanguageCode::EN, name: s!("English"),           countries: hash_set![ AG, AI, AS, AU, BB, BI, BM, BS, BW, BZ, CA, CC, CK, CM, CW, CX, DM, FJ, FK, FM, GB, GD, GG, GH, GI, GL, GM, GS, GU, GY, HK, HM, IE, IM, IN, IO, JE, JM, KE, KI, KN, KY, LC, LR, LS, MH, MP, MS, MT, MU, MW, NA, NF, NG, NR, NU, NZ, PG, PH, PK, PN, PR, PW, RW, SB, SC, SD, SG, SH, SL, SS, SX, SZ, TC, TK, TO, TT, TV, TZ, UG, UM, US, VC, VG, VI, VU, WS, ZA, ZM, ZW ] },
		LanguageCode::EO: Language { code: LanguageCode::EO, name: s!("Esperanto"),         countries: hash_set![] },
		LanguageCode::ES: Language { code: LanguageCode::ES, name: s!("Spanish"),           countries: hash_set![ AR, BO, CL, CO, CR, CU, DO, EC, EH, ES, GQ, GT, HN, MX, NI, PA, PE, PR, PY, SV, UY, VE ] },
		LanguageCode::ET: Language { code: LanguageCode::ET, name: s!("Estonian"),          countries: hash_set![ EE ] },
		LanguageCode::EU: Language { code: LanguageCode::EU, name: s!("Basque"),            countries: hash_set![] },
		LanguageCode::FA: Language { code: LanguageCode::FA, name: s!("Persian"),           countries: hash_set![ AF, IR ] },
		LanguageCode::FF: Language { code: LanguageCode::FF, name: s!("Fulah"),             countries: hash_set![ ML ] },
		LanguageCode::FI: Language { code: LanguageCode::FI, name: s!("Finnish"),           countries: hash_set![ FI ] },
		LanguageCode::FJ: Language { code: LanguageCode::FJ, name: s!("Fijian"),            countries: hash_set![ FJ ] },
		LanguageCode::FO: Language { code: LanguageCode::FO, name: s!("Faroese"),           countries: hash_set![ FO ] },
		LanguageCode::FR: Language { code: LanguageCode::FR, name: s!("French"),            countries: hash_set![ BE, BF, BI, BJ, BL, CA, CD, CF, CG, CH, CI, CM, DJ, FR, GA, GF, GN, GP, GQ, HT, JE, KM, LU, MC, MF, MG, MQ, NC, NE, PF, PM, RE, RW, SC, SN, TD, TF, TG, VU, WF, YT ] },
		LanguageCode::FY: Language { code: LanguageCode::FY, name: s!("Western Frisian"),   countries: hash_set![] },
		LanguageCode::GA: Language { code: LanguageCode::GA, name: s!("Irish"),             countries: hash_set![ IE ] },
		LanguageCode::GD: Language { code: LanguageCode::GD, name: s!("Gaelic"),            countries: hash_set![] },
		LanguageCode::GL: Language { code: LanguageCode::GL, name: s!("Galician"),          countries: hash_set![] },
		LanguageCode::GN: Language { code: LanguageCode::GN, name: s!("Guarani"),           countries: hash_set![ BO, PY ] },
		LanguageCode::GU: Language { code: LanguageCode::GU, name: s!("Gujarati"),          countries: hash_set![] },
		LanguageCode::GV: Language { code: LanguageCode::GV, name: s!("Manx"),              countries: hash_set![ IM ] },
		LanguageCode::HA: Language { code: LanguageCode::HA, name: s!("Hausa"),             countries: hash_set![] },
		LanguageCode::HE: Language { code: LanguageCode::HE, name: s!("Hebrew"),            countries: hash_set![ IL ] },
		LanguageCode::HI: Language { code: LanguageCode::HI, name: s!("Hindi"),             countries: hash_set![ IN ] },
		LanguageCode::HO: Language { code: LanguageCode::HO, name: s!("Hiri Motu"),         countries: hash_set![ PG ] },
		LanguageCode::HR: Language { code: LanguageCode::HR, name: s!("Croatian"),          countries: hash_set![ BA, HR, ME ] },
		LanguageCode::HT: Language { code: LanguageCode::HT, name: s!("Haitian"),           countries: hash_set![ HT ] },
		LanguageCode::HU: Language { code: LanguageCode::HU, name: s!("Hungarian"),         countries: hash_set![ HU ] },
		LanguageCode::HY: Language { code: LanguageCode::HY, name: s!("Armenian"),          countries: hash_set![ AM ] },
		LanguageCode::HZ: Language { code: LanguageCode::HZ, name: s!("Herero"),            countries: hash_set![] },
		LanguageCode::IA: Language { code: LanguageCode::IA, name: s!("Interlingua"),       countries: hash_set![] },
		LanguageCode::ID: Language { code: LanguageCode::ID, name: s!("Indonesian"),        countries: hash_set![ ID ] },
		LanguageCode::IE: Language { code: LanguageCode::IE, name: s!("Interlingue"),       countries: hash_set![] },
		LanguageCode::IG: Language { code: LanguageCode::IG, name: s!("Igbo"),              countries: hash_set![] },
		LanguageCode::II: Language { code: LanguageCode::II, name: s!("Sichuan Yi"),        countries: hash_set![] },
		LanguageCode::IK: Language { code: LanguageCode::IK, name: s!("Inupiaq"),           countries: hash_set![] },
		LanguageCode::IO: Language { code: LanguageCode::IO, name: s!("Ido"),               countries: hash_set![] },
		LanguageCode::IS: Language { code: LanguageCode::IS, name: s!("Icelandic"),         countries: hash_set![ IS ] },
		LanguageCode::IT: Language { code: LanguageCode::IT, name: s!("Italian"),           countries: hash_set![ CH, IT, SM, VA ] },
		LanguageCode::IU: Language { code: LanguageCode::IU, name: s!("Inuktitut"),         countries: hash_set![] },
		LanguageCode::JA: Language { code: LanguageCode::JA, name: s!("Japanese"),          countries: hash_set![ JP ] },
		LanguageCode::JV: Language { code: LanguageCode::JV, name: s!("Javanese"),          countries: hash_set![] },
		LanguageCode::KA: Language { code: LanguageCode::KA, name: s!("Georgian"),          countries: hash_set![ GE ] },
		LanguageCode::KG: Language { code: LanguageCode::KG, name: s!("Kongo"),             countries: hash_set![] },
		LanguageCode::KI: Language { code: LanguageCode::KI, name: s!("Kikuyu"),            countries: hash_set![] },
		LanguageCode::KJ: Language { code: LanguageCode::KJ, name: s!("Kuanyama"),          countries: hash_set![] },
		LanguageCode::KK: Language { code: LanguageCode::KK, name: s!("Kazakh"),            countries: hash_set![ KZ ] },
		LanguageCode::KL: Language { code: LanguageCode::KL, name: s!("Kalaallisut"),       countries: hash_set![] },
		LanguageCode::KM: Language { code: LanguageCode::KM, name: s!("Central Khmer"),     countries: hash_set![ KH ] },
		LanguageCode::KN: Language { code: LanguageCode::KN, name: s!("Kannada"),           countries: hash_set![] },
		LanguageCode::KO: Language { code: LanguageCode::KO, name: s!("Korean"),            countries: hash_set![ KP, KR ] },
		LanguageCode::KR: Language { code: LanguageCode::KR, name: s!("Kanuri"),            countries: hash_set![] },
		LanguageCode::KS: Language { code: LanguageCode::KS, name: s!("Kashmiri"),          countries: hash_set![] },
		LanguageCode::KU: Language { code: LanguageCode::KU, name: s!("Kurdish"),           countries: hash_set![ IQ ] },
		LanguageCode::KV: Language { code: LanguageCode::KV, name: s!("Komi"),              countries: hash_set![] },
		LanguageCode::KW: Language { code: LanguageCode::KW, name: s!("Cornish"),           countries: hash_set![] },
		LanguageCode::KY: Language { code: LanguageCode::KY, name: s!("Kirghiz"),           countries: hash_set![ KG ] },
		LanguageCode::LA: Language { code: LanguageCode::LA, name: s!("Latin"),             countries: hash_set![ VA ] },
		LanguageCode::LB: Language { code: LanguageCode::LB, name: s!("Luxembourgish"),     countries: hash_set![ LU ] },
		LanguageCode::LG: Language { code: LanguageCode::LG, name: s!("Ganda"),             countries: hash_set![] },
		LanguageCode::LI: Language { code: LanguageCode::LI, name: s!("Limburgan"),         countries: hash_set![] },
		LanguageCode::LN: Language { code: LanguageCode::LN, name: s!("Lingala"),           countries: hash_set![] },
		LanguageCode::LO: Language { code: LanguageCode::LO, name: s!("Lao"),               countries: hash_set![ LA ] },
		LanguageCode::LT: Language { code: LanguageCode::LT, name: s!("Lithuanian"),        countries: hash_set![ LT ] },
		LanguageCode::LU: Language { code: LanguageCode::LU, name: s!("Luba-Katanga"),      countries: hash_set![] },
		LanguageCode::LV: Language { code: LanguageCode::LV, name: s!("Latvian"),           countries: hash_set![ LV ] },
		LanguageCode::MG: Language { code: LanguageCode::MG, name: s!("Malagasy"),          countries: hash_set![ MG ] },
		LanguageCode::MH: Language { code: LanguageCode::MH, name: s!("Marshallese"),       countries: hash_set![ MH ] },
		LanguageCode::MI: Language { code: LanguageCode::MI, name: s!("Maori"),             countries: hash_set![ NZ ] },
		LanguageCode::MK: Language { code: LanguageCode::MK, name: s!("Macedonian"),        countries: hash_set![ MK ] },
		LanguageCode::ML: Language { code: LanguageCode::ML, name: s!("Malayalam"),         countries: hash_set![] },
		LanguageCode::MN: Language { code: LanguageCode::MN, name: s!("Mongolian"),         countries: hash_set![ MN ] },
		LanguageCode::MR: Language { code: LanguageCode::MR, name: s!("Marathi"),           countries: hash_set![] },
		LanguageCode::MS: Language { code: LanguageCode::MS, name: s!("Malay"),             countries: hash_set![ BN, CC, CX, MY, SG ] },
		LanguageCode::MT: Language { code: LanguageCode::MT, name: s!("Maltese"),           countries: hash_set![ MT ] },
		LanguageCode::MY: Language { code: LanguageCode::MY, name: s!("Burmese"),           countries: hash_set![ MM ] },
		LanguageCode::NA: Language { code: LanguageCode::NA, name: s!("Nauru"),             countries: hash_set![ NR ] },
		LanguageCode::NB: Language { code: LanguageCode::NB, name: s!("Norwegian Bokmål"),  countries: hash_set![] },
		LanguageCode::ND: Language { code: LanguageCode::ND, name: s!("North Ndebele"),     countries: hash_set![] },
		LanguageCode::NE: Language { code: LanguageCode::NE, name: s!("Nepali"),            countries: hash_set![ NP ] },
		LanguageCode::NG: Language { code: LanguageCode::NG, name: s!("Ndonga"),            countries: hash_set![] },
		LanguageCode::NL: Language { code: LanguageCode::NL, name: s!("Dutch"),             countries: hash_set![ AW, BE, BQ, CW, NL, SR, SX ] },
		LanguageCode::NN: Language { code: LanguageCode::NN, name: s!("Norwegian Nynorsk"), countries: hash_set![] },
		LanguageCode::NO: Language { code: LanguageCode::NO, name: s!("Norwegian"),         countries: hash_set![ BV, NO, SJ ] },
		LanguageCode::NR: Language { code: LanguageCode::NR, name: s!("South Ndebele"),     countries: hash_set![ ZA, ZW ] },
		LanguageCode::NV: Language { code: LanguageCode::NV, name: s!("Navajo"),            countries: hash_set![] },
		LanguageCode::NY: Language { code: LanguageCode::NY, name: s!("Chichewa"),          countries: hash_set![ MW, ZW ] },
		LanguageCode::OC: Language { code: LanguageCode::OC, name: s!("Occitan"),           countries: hash_set![] },
		LanguageCode::OJ: Language { code: LanguageCode::OJ, name: s!("Ojibwa"),            countries: hash_set![] },
		LanguageCode::OM: Language { code: LanguageCode::OM, name: s!("Oromo"),             countries: hash_set![ ET ] },
		LanguageCode::OR: Language { code: LanguageCode::OR, name: s!("Oriya"),             countries: hash_set![] },
		LanguageCode::OS: Language { code: LanguageCode::OS, name: s!("Ossetian"),          countries: hash_set![] },
		LanguageCode::PA: Language { code: LanguageCode::PA, name: s!("Punjabi"),           countries: hash_set![] },
		LanguageCode::PI: Language { code: LanguageCode::PI, name: s!("Pali"),              countries: hash_set![] },
		LanguageCode::PL: Language { code: LanguageCode::PL, name: s!("Polish"),            countries: hash_set![ PL ] },
		LanguageCode::PS: Language { code: LanguageCode::PS, name: s!("Pashto"),            countries: hash_set![ AF ] },
		LanguageCode::PT: Language { code: LanguageCode::PT, name: s!("Portuguese"),        countries: hash_set![ AO, BR, CV, GW, MO, MZ, PT, ST, TL, GQ ] },
		LanguageCode::QU: Language { code: LanguageCode::QU, name: s!("Quechua"),           countries: hash_set![ BO, EC, PE ] },
		LanguageCode::RM: Language { code: LanguageCode::RM, name: s!("Romansh"),           countries: hash_set![ CH ] },
		LanguageCode::RN: Language { code: LanguageCode::RN, name: s!("Rundi"),             countries: hash_set![ BI ] },
		LanguageCode::RO: Language { code: LanguageCode::RO, name: s!("Romanian"),          countries: hash_set![ MD, RO ] },
		LanguageCode::RU: Language { code: LanguageCode::RU, name: s!("Russian"),           countries: hash_set![ BY, KG, KZ, RU ] },
		LanguageCode::RW: Language { code: LanguageCode::RW, name: s!("Kinyarwanda"),       countries: hash_set![ RW ] },
		LanguageCode::SA: Language { code: LanguageCode::SA, name: s!("Sanskrit"),          countries: hash_set![] },
		LanguageCode::SC: Language { code: LanguageCode::SC, name: s!("Sardinian"),         countries: hash_set![] },
		LanguageCode::SD: Language { code: LanguageCode::SD, name: s!("Sindhi"),            countries: hash_set![] },
		LanguageCode::SE: Language { code: LanguageCode::SE, name: s!("Northern Sami"),     countries: hash_set![] },
		LanguageCode::SG: Language { code: LanguageCode::SG, name: s!("Sango"),             countries: hash_set![ CF ] },
		LanguageCode::SI: Language { code: LanguageCode::SI, name: s!("Sinhala"),           countries: hash_set![ LK ] },
		LanguageCode::SK: Language { code: LanguageCode::SK, name: s!("Slovak"),            countries: hash_set![ CZ, SK ] },
		LanguageCode::SL: Language { code: LanguageCode::SL, name: s!("Slovenian"),         countries: hash_set![ SI ] },
		LanguageCode::SM: Language { code: LanguageCode::SM, name: s!("Samoan"),            countries: hash_set![ AS, WS ] },
		LanguageCode::SN: Language { code: LanguageCode::SN, name: s!("Shona"),             countries: hash_set![ ZW ] },
		LanguageCode::SO: Language { code: LanguageCode::SO, name: s!("Somali"),            countries: hash_set![ ET, SO ] },
		LanguageCode::SQ: Language { code: LanguageCode::SQ, name: s!("Albanian"),          countries: hash_set![ AL, MK ] },
		LanguageCode::SR: Language { code: LanguageCode::SR, name: s!("Serbian"),           countries: hash_set![ BA, ME, RS ] },
		LanguageCode::SS: Language { code: LanguageCode::SS, name: s!("Swati"),             countries: hash_set![ SZ, ZA ] },
		LanguageCode::ST: Language { code: LanguageCode::ST, name: s!("Southern Sotho"),    countries: hash_set![ LS, ZA, ZW ] },
		LanguageCode::SU: Language { code: LanguageCode::SU, name: s!("Sundanese"),         countries: hash_set![] },
		LanguageCode::SV: Language { code: LanguageCode::SV, name: s!("Swedish"),           countries: hash_set![ AX, FI, SE ] },
		LanguageCode::SW: Language { code: LanguageCode::SW, name: s!("Swahili"),           countries: hash_set![ KE, RW, TZ, UG ] },
		LanguageCode::TA: Language { code: LanguageCode::TA, name: s!("Tamil"),             countries: hash_set![ LK, SG ] },
		LanguageCode::TE: Language { code: LanguageCode::TE, name: s!("Telugu"),            countries: hash_set![] },
		LanguageCode::TG: Language { code: LanguageCode::TG, name: s!("Tajik"),             countries: hash_set![ TJ ] },
		LanguageCode::TH: Language { code: LanguageCode::TH, name: s!("Thai"),              countries: hash_set![ TH ] },
		LanguageCode::TI: Language { code: LanguageCode::TI, name: s!("Tigrinya"),          countries: hash_set![ ER, ET ] },
		LanguageCode::TK: Language { code: LanguageCode::TK, name: s!("Turkmen"),           countries: hash_set![ TM ] },
		LanguageCode::TL: Language { code: LanguageCode::TL, name: s!("Tagalog"),           countries: hash_set![ PH ] },
		LanguageCode::TN: Language { code: LanguageCode::TN, name: s!("Tswana"),            countries: hash_set![ ZA, ZW ] },
		LanguageCode::TO: Language { code: LanguageCode::TO, name: s!("Tonga"),             countries: hash_set![ TO ] },
		LanguageCode::TR: Language { code: LanguageCode::TR, name: s!("Turkish"),           countries: hash_set![ CY, TR ] },
		LanguageCode::TS: Language { code: LanguageCode::TS, name: s!("Tsonga"),            countries: hash_set![ ZA ] },
		LanguageCode::TT: Language { code: LanguageCode::TT, name: s!("Tatar"),             countries: hash_set![] },
		LanguageCode::TW: Language { code: LanguageCode::TW, name: s!("Twi"),               countries: hash_set![] },
		LanguageCode::TY: Language { code: LanguageCode::TY, name: s!("Tahitian"),          countries: hash_set![] },
		LanguageCode::UG: Language { code: LanguageCode::UG, name: s!("Uighur"),            countries: hash_set![] },
		LanguageCode::UK: Language { code: LanguageCode::UK, name: s!("Ukrainian"),         countries: hash_set![ UA ] },
		LanguageCode::UR: Language { code: LanguageCode::UR, name: s!("Urdu"),              countries: hash_set![ PK ] },
		LanguageCode::UZ: Language { code: LanguageCode::UZ, name: s!("Uzbek"),             countries: hash_set![ UZ ] },
		LanguageCode::VE: Language { code: LanguageCode::VE, name: s!("Venda"),             countries: hash_set![ ZA, ZW ] },
		LanguageCode::VI: Language { code: LanguageCode::VI, name: s!("Vietnamese"),        countries: hash_set![ VN ] },
		LanguageCode::VO: Language { code: LanguageCode::VO, name: s!("Volapük"),           countries: hash_set![] },
		LanguageCode::WA: Language { code: LanguageCode::WA, name: s!("Walloon"),           countries: hash_set![] },
		LanguageCode::WO: Language { code: LanguageCode::WO, name: s!("Wolof"),             countries: hash_set![] },
		LanguageCode::XH: Language { code: LanguageCode::XH, name: s!("Xhosa"),             countries: hash_set![ ZA, ZW ] },
		LanguageCode::YI: Language { code: LanguageCode::YI, name: s!("Yiddish"),           countries: hash_set![] },
		LanguageCode::YO: Language { code: LanguageCode::YO, name: s!("Yoruba"),            countries: hash_set![] },
		LanguageCode::ZA: Language { code: LanguageCode::ZA, name: s!("Zhuang"),            countries: hash_set![] },
		LanguageCode::ZH: Language { code: LanguageCode::ZH, name: s!("Chinese"),           countries: hash_set![ CN, CX, HK, MO, SG, TW ] },
		LanguageCode::ZU: Language { code: LanguageCode::ZU, name: s!("Zulu"),              countries: hash_set![ ZA ] },
	}
});



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
	//		language															
	/// Returns the `Language` instance corresponding to the `LanguageCode`.
	/// 
	/// This method provides an easy way to get to the associated `Language`
	/// instance from a `LanguageCode` enum variant.
	/// 
	#[cfg_attr(    feature = "reasons",  allow(clippy::missing_panics_doc, reason = "Infallible"))]
	#[cfg_attr(not(feature = "reasons"), allow(clippy::missing_panics_doc))]
	pub fn language(&self) -> &Language {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		//	This should be infallible. If it isn't, then the data is wrong, and one
		//	of the languages is missing from the list, which is a bug.
		LANGUAGES.get(self).unwrap()
	}
}

impl AsStr for LanguageCode {
	//		as_str																
	#[cfg_attr(    feature = "reasons",  allow(clippy::too_many_lines, reason = "Data not logic"))]
	#[cfg_attr(not(feature = "reasons"), allow(clippy::too_many_lines))]
	fn as_str(&self) -> &'static str {
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
#[derive(Clone, Eq, PartialEq, ToSchema)]
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

impl AsStr for Language {
	//		as_str																
	fn as_str(&self) -> &str {
		&self.name
	}
}

impl Debug for Language {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.code.as_str(), self.as_str())
	}
}

impl<'de> Deserialize<'de> for Language {
	//		deserialize															
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		String::deserialize(deserializer)?.parse().map_err(DeError::custom)
	}
}

impl Display for Language {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl From<Language> for String {
	//		from																
	fn from(language: Language) -> Self {
		language.to_string()
	}
}

impl FromStr for Language {
	type Err = String;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		LANGUAGES
			.values()
			.find(|language| language.name == s)
			.cloned()
			.ok_or_else(|| format!("Invalid Language: {s}"))
	}
}

impl Serialize for Language {
	//		serialize															
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(self.as_str())
	}
}

impl TryFrom<String> for Language {
	type Error = String;
	
	//		try_from															
	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.as_str().parse()
	}
}


