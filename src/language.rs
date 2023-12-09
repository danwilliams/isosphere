//! Language-related types.



//		Modules

#[cfg(test)]
#[path = "tests/language.rs"]
mod tests;



//		Packages

#[cfg_attr(    feature = "reasons",  allow(clippy::enum_glob_use, reason = "Brevity wins here"))]
#[cfg_attr(not(feature = "reasons"), allow(clippy::enum_glob_use))]
use crate::country::CountryCode;
use core::{
	fmt::{Debug, Display, self},
	str::FromStr,
};
use once_cell::sync::Lazy;
use rubedo::{
	std::AsStr,
	sugar::{s, vh},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};
use std::collections::{HashMap, HashSet};
use utoipa::ToSchema;
use velcro::hash_map;



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
		LanguageCode::AA: Language { code: LanguageCode::AA, name: s!("Afar"),              countries: vh![ CountryCode: ET ] },
		LanguageCode::AB: Language { code: LanguageCode::AB, name: s!("Abkhazian"),         countries: vh![] },
		LanguageCode::AE: Language { code: LanguageCode::AE, name: s!("Avestan"),           countries: vh![] },
		LanguageCode::AF: Language { code: LanguageCode::AF, name: s!("Afrikaans"),         countries: vh![ CountryCode: ZA ] },
		LanguageCode::AK: Language { code: LanguageCode::AK, name: s!("Akan"),              countries: vh![] },
		LanguageCode::AM: Language { code: LanguageCode::AM, name: s!("Amharic"),           countries: vh![ CountryCode: ET ] },
		LanguageCode::AN: Language { code: LanguageCode::AN, name: s!("Aragonese"),         countries: vh![] },
		LanguageCode::AR: Language { code: LanguageCode::AR, name: s!("Arabic"),            countries: vh![ CountryCode: AE, BH, DJ, DZ, EG, EH, IQ, JO, KM, KW, LB, LY, MA, MR, OM, PS, QA, SA, SD, SO, SY, TD, TN, YE ] },
		LanguageCode::AS: Language { code: LanguageCode::AS, name: s!("Assamese"),          countries: vh![] },
		LanguageCode::AV: Language { code: LanguageCode::AV, name: s!("Avaric"),            countries: vh![] },
		LanguageCode::AY: Language { code: LanguageCode::AY, name: s!("Aymara"),            countries: vh![ CountryCode: BO, PE ] },
		LanguageCode::AZ: Language { code: LanguageCode::AZ, name: s!("Azerbaijani"),       countries: vh![ CountryCode: AZ ] },
		LanguageCode::BA: Language { code: LanguageCode::BA, name: s!("Bashkir"),           countries: vh![] },
		LanguageCode::BE: Language { code: LanguageCode::BE, name: s!("Belarusian"),        countries: vh![ CountryCode: BY ] },
		LanguageCode::BG: Language { code: LanguageCode::BG, name: s!("Bulgarian"),         countries: vh![ CountryCode: BG ] },
		LanguageCode::BI: Language { code: LanguageCode::BI, name: s!("Bislama"),           countries: vh![ CountryCode: VU ] },
		LanguageCode::BM: Language { code: LanguageCode::BM, name: s!("Bambara"),           countries: vh![ CountryCode: ML ] },
		LanguageCode::BN: Language { code: LanguageCode::BN, name: s!("Bengali"),           countries: vh![ CountryCode: BD ] },
		LanguageCode::BO: Language { code: LanguageCode::BO, name: s!("Tibetan"),           countries: vh![] },
		LanguageCode::BR: Language { code: LanguageCode::BR, name: s!("Breton"),            countries: vh![] },
		LanguageCode::BS: Language { code: LanguageCode::BS, name: s!("Bosnian"),           countries: vh![ CountryCode: BA ] },
		LanguageCode::CA: Language { code: LanguageCode::CA, name: s!("Catalan"),           countries: vh![ CountryCode: AD ] },
		LanguageCode::CE: Language { code: LanguageCode::CE, name: s!("Chechen"),           countries: vh![] },
		LanguageCode::CH: Language { code: LanguageCode::CH, name: s!("Chamorro"),          countries: vh![ CountryCode: GU, MP ] },
		LanguageCode::CO: Language { code: LanguageCode::CO, name: s!("Corsican"),          countries: vh![] },
		LanguageCode::CR: Language { code: LanguageCode::CR, name: s!("Cree"),              countries: vh![] },
		LanguageCode::CS: Language { code: LanguageCode::CS, name: s!("Czech"),             countries: vh![ CountryCode: CZ ] },
		LanguageCode::CU: Language { code: LanguageCode::CU, name: s!("Church Slavonic"),   countries: vh![] },
		LanguageCode::CV: Language { code: LanguageCode::CV, name: s!("Chuvash"),           countries: vh![] },
		LanguageCode::CY: Language { code: LanguageCode::CY, name: s!("Welsh"),             countries: vh![] },
		LanguageCode::DA: Language { code: LanguageCode::DA, name: s!("Danish"),            countries: vh![ CountryCode: DK, FO, GL ] },
		LanguageCode::DE: Language { code: LanguageCode::DE, name: s!("German"),            countries: vh![ CountryCode: AT, BE, CH, DE, LI, LU ] },
		LanguageCode::DV: Language { code: LanguageCode::DV, name: s!("Divehi"),            countries: vh![ CountryCode: MV ] },
		LanguageCode::DZ: Language { code: LanguageCode::DZ, name: s!("Dzongkha"),          countries: vh![ CountryCode: BT ] },
		LanguageCode::EE: Language { code: LanguageCode::EE, name: s!("Ewe"),               countries: vh![] },
		LanguageCode::EL: Language { code: LanguageCode::EL, name: s!("Greek"),             countries: vh![ CountryCode: CY, GR ] },
		LanguageCode::EN: Language { code: LanguageCode::EN, name: s!("English"),           countries: vh![ CountryCode: AG, AI, AS, AU, BB, BI, BM, BS, BW, BZ, CA, CC, CK, CM, CW, CX, DM, FJ, FK, FM, GB, GD, GG, GH, GI, GL, GM, GS, GU, GY, HK, HM, IE, IM, IN, IO, JE, JM, KE, KI, KN, KY, LC, LR, LS, MH, MP, MS, MT, MU, MW, NA, NF, NG, NR, NU, NZ, PG, PH, PK, PN, PR, PW, RW, SB, SC, SD, SG, SH, SL, SS, SX, SZ, TC, TK, TO, TT, TV, TZ, UG, UM, US, VC, VG, VI, VU, WS, ZA, ZM, ZW ] },
		LanguageCode::EO: Language { code: LanguageCode::EO, name: s!("Esperanto"),         countries: vh![] },
		LanguageCode::ES: Language { code: LanguageCode::ES, name: s!("Spanish"),           countries: vh![ CountryCode: AR, BO, CL, CO, CR, CU, DO, EC, EH, ES, GQ, GT, HN, MX, NI, PA, PE, PR, PY, SV, UY, VE ] },
		LanguageCode::ET: Language { code: LanguageCode::ET, name: s!("Estonian"),          countries: vh![ CountryCode: EE ] },
		LanguageCode::EU: Language { code: LanguageCode::EU, name: s!("Basque"),            countries: vh![] },
		LanguageCode::FA: Language { code: LanguageCode::FA, name: s!("Persian"),           countries: vh![ CountryCode: AF, IR ] },
		LanguageCode::FF: Language { code: LanguageCode::FF, name: s!("Fulah"),             countries: vh![ CountryCode: ML ] },
		LanguageCode::FI: Language { code: LanguageCode::FI, name: s!("Finnish"),           countries: vh![ CountryCode: FI ] },
		LanguageCode::FJ: Language { code: LanguageCode::FJ, name: s!("Fijian"),            countries: vh![ CountryCode: FJ ] },
		LanguageCode::FO: Language { code: LanguageCode::FO, name: s!("Faroese"),           countries: vh![ CountryCode: FO ] },
		LanguageCode::FR: Language { code: LanguageCode::FR, name: s!("French"),            countries: vh![ CountryCode: BE, BF, BI, BJ, BL, CA, CD, CF, CG, CH, CI, CM, DJ, FR, GA, GF, GN, GP, GQ, HT, JE, KM, LU, MC, MF, MG, MQ, NC, NE, PF, PM, RE, RW, SC, SN, TD, TF, TG, VU, WF, YT ] },
		LanguageCode::FY: Language { code: LanguageCode::FY, name: s!("Western Frisian"),   countries: vh![] },
		LanguageCode::GA: Language { code: LanguageCode::GA, name: s!("Irish"),             countries: vh![ CountryCode: IE ] },
		LanguageCode::GD: Language { code: LanguageCode::GD, name: s!("Gaelic"),            countries: vh![] },
		LanguageCode::GL: Language { code: LanguageCode::GL, name: s!("Galician"),          countries: vh![] },
		LanguageCode::GN: Language { code: LanguageCode::GN, name: s!("Guarani"),           countries: vh![ CountryCode: BO, PY ] },
		LanguageCode::GU: Language { code: LanguageCode::GU, name: s!("Gujarati"),          countries: vh![] },
		LanguageCode::GV: Language { code: LanguageCode::GV, name: s!("Manx"),              countries: vh![ CountryCode: IM ] },
		LanguageCode::HA: Language { code: LanguageCode::HA, name: s!("Hausa"),             countries: vh![] },
		LanguageCode::HE: Language { code: LanguageCode::HE, name: s!("Hebrew"),            countries: vh![ CountryCode: IL ] },
		LanguageCode::HI: Language { code: LanguageCode::HI, name: s!("Hindi"),             countries: vh![ CountryCode: IN ] },
		LanguageCode::HO: Language { code: LanguageCode::HO, name: s!("Hiri Motu"),         countries: vh![ CountryCode: PG ] },
		LanguageCode::HR: Language { code: LanguageCode::HR, name: s!("Croatian"),          countries: vh![ CountryCode: BA, HR, ME ] },
		LanguageCode::HT: Language { code: LanguageCode::HT, name: s!("Haitian"),           countries: vh![ CountryCode: HT ] },
		LanguageCode::HU: Language { code: LanguageCode::HU, name: s!("Hungarian"),         countries: vh![ CountryCode: HU ] },
		LanguageCode::HY: Language { code: LanguageCode::HY, name: s!("Armenian"),          countries: vh![ CountryCode: AM ] },
		LanguageCode::HZ: Language { code: LanguageCode::HZ, name: s!("Herero"),            countries: vh![] },
		LanguageCode::IA: Language { code: LanguageCode::IA, name: s!("Interlingua"),       countries: vh![] },
		LanguageCode::ID: Language { code: LanguageCode::ID, name: s!("Indonesian"),        countries: vh![ CountryCode: ID ] },
		LanguageCode::IE: Language { code: LanguageCode::IE, name: s!("Interlingue"),       countries: vh![] },
		LanguageCode::IG: Language { code: LanguageCode::IG, name: s!("Igbo"),              countries: vh![] },
		LanguageCode::II: Language { code: LanguageCode::II, name: s!("Sichuan Yi"),        countries: vh![] },
		LanguageCode::IK: Language { code: LanguageCode::IK, name: s!("Inupiaq"),           countries: vh![] },
		LanguageCode::IO: Language { code: LanguageCode::IO, name: s!("Ido"),               countries: vh![] },
		LanguageCode::IS: Language { code: LanguageCode::IS, name: s!("Icelandic"),         countries: vh![ CountryCode: IS ] },
		LanguageCode::IT: Language { code: LanguageCode::IT, name: s!("Italian"),           countries: vh![ CountryCode: CH, IT, SM, VA ] },
		LanguageCode::IU: Language { code: LanguageCode::IU, name: s!("Inuktitut"),         countries: vh![] },
		LanguageCode::JA: Language { code: LanguageCode::JA, name: s!("Japanese"),          countries: vh![ CountryCode: JP ] },
		LanguageCode::JV: Language { code: LanguageCode::JV, name: s!("Javanese"),          countries: vh![] },
		LanguageCode::KA: Language { code: LanguageCode::KA, name: s!("Georgian"),          countries: vh![ CountryCode: GE ] },
		LanguageCode::KG: Language { code: LanguageCode::KG, name: s!("Kongo"),             countries: vh![] },
		LanguageCode::KI: Language { code: LanguageCode::KI, name: s!("Kikuyu"),            countries: vh![] },
		LanguageCode::KJ: Language { code: LanguageCode::KJ, name: s!("Kuanyama"),          countries: vh![] },
		LanguageCode::KK: Language { code: LanguageCode::KK, name: s!("Kazakh"),            countries: vh![ CountryCode: KZ ] },
		LanguageCode::KL: Language { code: LanguageCode::KL, name: s!("Kalaallisut"),       countries: vh![] },
		LanguageCode::KM: Language { code: LanguageCode::KM, name: s!("Central Khmer"),     countries: vh![ CountryCode: KH ] },
		LanguageCode::KN: Language { code: LanguageCode::KN, name: s!("Kannada"),           countries: vh![] },
		LanguageCode::KO: Language { code: LanguageCode::KO, name: s!("Korean"),            countries: vh![ CountryCode: KP, KR ] },
		LanguageCode::KR: Language { code: LanguageCode::KR, name: s!("Kanuri"),            countries: vh![] },
		LanguageCode::KS: Language { code: LanguageCode::KS, name: s!("Kashmiri"),          countries: vh![] },
		LanguageCode::KU: Language { code: LanguageCode::KU, name: s!("Kurdish"),           countries: vh![ CountryCode: IQ ] },
		LanguageCode::KV: Language { code: LanguageCode::KV, name: s!("Komi"),              countries: vh![] },
		LanguageCode::KW: Language { code: LanguageCode::KW, name: s!("Cornish"),           countries: vh![] },
		LanguageCode::KY: Language { code: LanguageCode::KY, name: s!("Kirghiz"),           countries: vh![ CountryCode: KG ] },
		LanguageCode::LA: Language { code: LanguageCode::LA, name: s!("Latin"),             countries: vh![ CountryCode: VA ] },
		LanguageCode::LB: Language { code: LanguageCode::LB, name: s!("Luxembourgish"),     countries: vh![ CountryCode: LU ] },
		LanguageCode::LG: Language { code: LanguageCode::LG, name: s!("Ganda"),             countries: vh![] },
		LanguageCode::LI: Language { code: LanguageCode::LI, name: s!("Limburgan"),         countries: vh![] },
		LanguageCode::LN: Language { code: LanguageCode::LN, name: s!("Lingala"),           countries: vh![] },
		LanguageCode::LO: Language { code: LanguageCode::LO, name: s!("Lao"),               countries: vh![ CountryCode: LA ] },
		LanguageCode::LT: Language { code: LanguageCode::LT, name: s!("Lithuanian"),        countries: vh![ CountryCode: LT ] },
		LanguageCode::LU: Language { code: LanguageCode::LU, name: s!("Luba-Katanga"),      countries: vh![] },
		LanguageCode::LV: Language { code: LanguageCode::LV, name: s!("Latvian"),           countries: vh![ CountryCode: LV ] },
		LanguageCode::MG: Language { code: LanguageCode::MG, name: s!("Malagasy"),          countries: vh![ CountryCode: MG ] },
		LanguageCode::MH: Language { code: LanguageCode::MH, name: s!("Marshallese"),       countries: vh![ CountryCode: MH ] },
		LanguageCode::MI: Language { code: LanguageCode::MI, name: s!("Maori"),             countries: vh![ CountryCode: NZ ] },
		LanguageCode::MK: Language { code: LanguageCode::MK, name: s!("Macedonian"),        countries: vh![ CountryCode: MK ] },
		LanguageCode::ML: Language { code: LanguageCode::ML, name: s!("Malayalam"),         countries: vh![] },
		LanguageCode::MN: Language { code: LanguageCode::MN, name: s!("Mongolian"),         countries: vh![ CountryCode: MN ] },
		LanguageCode::MR: Language { code: LanguageCode::MR, name: s!("Marathi"),           countries: vh![] },
		LanguageCode::MS: Language { code: LanguageCode::MS, name: s!("Malay"),             countries: vh![ CountryCode: BN, CC, CX, MY, SG ] },
		LanguageCode::MT: Language { code: LanguageCode::MT, name: s!("Maltese"),           countries: vh![ CountryCode: MT ] },
		LanguageCode::MY: Language { code: LanguageCode::MY, name: s!("Burmese"),           countries: vh![ CountryCode: MM ] },
		LanguageCode::NA: Language { code: LanguageCode::NA, name: s!("Nauru"),             countries: vh![ CountryCode: NR ] },
		LanguageCode::NB: Language { code: LanguageCode::NB, name: s!("Norwegian Bokmål"),  countries: vh![] },
		LanguageCode::ND: Language { code: LanguageCode::ND, name: s!("North Ndebele"),     countries: vh![] },
		LanguageCode::NE: Language { code: LanguageCode::NE, name: s!("Nepali"),            countries: vh![ CountryCode: NP ] },
		LanguageCode::NG: Language { code: LanguageCode::NG, name: s!("Ndonga"),            countries: vh![] },
		LanguageCode::NL: Language { code: LanguageCode::NL, name: s!("Dutch"),             countries: vh![ CountryCode: AW, BE, BQ, CW, NL, SR, SX ] },
		LanguageCode::NN: Language { code: LanguageCode::NN, name: s!("Norwegian Nynorsk"), countries: vh![] },
		LanguageCode::NO: Language { code: LanguageCode::NO, name: s!("Norwegian"),         countries: vh![ CountryCode: BV, NO, SJ ] },
		LanguageCode::NR: Language { code: LanguageCode::NR, name: s!("South Ndebele"),     countries: vh![ CountryCode: ZA, ZW ] },
		LanguageCode::NV: Language { code: LanguageCode::NV, name: s!("Navajo"),            countries: vh![] },
		LanguageCode::NY: Language { code: LanguageCode::NY, name: s!("Chichewa"),          countries: vh![ CountryCode: MW, ZW ] },
		LanguageCode::OC: Language { code: LanguageCode::OC, name: s!("Occitan"),           countries: vh![] },
		LanguageCode::OJ: Language { code: LanguageCode::OJ, name: s!("Ojibwa"),            countries: vh![] },
		LanguageCode::OM: Language { code: LanguageCode::OM, name: s!("Oromo"),             countries: vh![ CountryCode: ET ] },
		LanguageCode::OR: Language { code: LanguageCode::OR, name: s!("Oriya"),             countries: vh![] },
		LanguageCode::OS: Language { code: LanguageCode::OS, name: s!("Ossetian"),          countries: vh![] },
		LanguageCode::PA: Language { code: LanguageCode::PA, name: s!("Punjabi"),           countries: vh![] },
		LanguageCode::PI: Language { code: LanguageCode::PI, name: s!("Pali"),              countries: vh![] },
		LanguageCode::PL: Language { code: LanguageCode::PL, name: s!("Polish"),            countries: vh![ CountryCode: PL ] },
		LanguageCode::PS: Language { code: LanguageCode::PS, name: s!("Pashto"),            countries: vh![ CountryCode: AF ] },
		LanguageCode::PT: Language { code: LanguageCode::PT, name: s!("Portuguese"),        countries: vh![ CountryCode: AO, BR, CV, GW, MO, MZ, PT, ST, TL, GQ ] },
		LanguageCode::QU: Language { code: LanguageCode::QU, name: s!("Quechua"),           countries: vh![ CountryCode: BO, EC, PE ] },
		LanguageCode::RM: Language { code: LanguageCode::RM, name: s!("Romansh"),           countries: vh![ CountryCode: CH ] },
		LanguageCode::RN: Language { code: LanguageCode::RN, name: s!("Rundi"),             countries: vh![ CountryCode: BI ] },
		LanguageCode::RO: Language { code: LanguageCode::RO, name: s!("Romanian"),          countries: vh![ CountryCode: MD, RO ] },
		LanguageCode::RU: Language { code: LanguageCode::RU, name: s!("Russian"),           countries: vh![ CountryCode: BY, KG, KZ, RU ] },
		LanguageCode::RW: Language { code: LanguageCode::RW, name: s!("Kinyarwanda"),       countries: vh![ CountryCode: RW ] },
		LanguageCode::SA: Language { code: LanguageCode::SA, name: s!("Sanskrit"),          countries: vh![] },
		LanguageCode::SC: Language { code: LanguageCode::SC, name: s!("Sardinian"),         countries: vh![] },
		LanguageCode::SD: Language { code: LanguageCode::SD, name: s!("Sindhi"),            countries: vh![] },
		LanguageCode::SE: Language { code: LanguageCode::SE, name: s!("Northern Sami"),     countries: vh![] },
		LanguageCode::SG: Language { code: LanguageCode::SG, name: s!("Sango"),             countries: vh![ CountryCode: CF ] },
		LanguageCode::SI: Language { code: LanguageCode::SI, name: s!("Sinhala"),           countries: vh![ CountryCode: LK ] },
		LanguageCode::SK: Language { code: LanguageCode::SK, name: s!("Slovak"),            countries: vh![ CountryCode: CZ, SK ] },
		LanguageCode::SL: Language { code: LanguageCode::SL, name: s!("Slovenian"),         countries: vh![ CountryCode: SI ] },
		LanguageCode::SM: Language { code: LanguageCode::SM, name: s!("Samoan"),            countries: vh![ CountryCode: AS, WS ] },
		LanguageCode::SN: Language { code: LanguageCode::SN, name: s!("Shona"),             countries: vh![ CountryCode: ZW ] },
		LanguageCode::SO: Language { code: LanguageCode::SO, name: s!("Somali"),            countries: vh![ CountryCode: ET, SO ] },
		LanguageCode::SQ: Language { code: LanguageCode::SQ, name: s!("Albanian"),          countries: vh![ CountryCode: AL, MK ] },
		LanguageCode::SR: Language { code: LanguageCode::SR, name: s!("Serbian"),           countries: vh![ CountryCode: BA, ME, RS ] },
		LanguageCode::SS: Language { code: LanguageCode::SS, name: s!("Swati"),             countries: vh![ CountryCode: SZ, ZA ] },
		LanguageCode::ST: Language { code: LanguageCode::ST, name: s!("Southern Sotho"),    countries: vh![ CountryCode: LS, ZA, ZW ] },
		LanguageCode::SU: Language { code: LanguageCode::SU, name: s!("Sundanese"),         countries: vh![] },
		LanguageCode::SV: Language { code: LanguageCode::SV, name: s!("Swedish"),           countries: vh![ CountryCode: AX, FI, SE ] },
		LanguageCode::SW: Language { code: LanguageCode::SW, name: s!("Swahili"),           countries: vh![ CountryCode: KE, RW, TZ, UG ] },
		LanguageCode::TA: Language { code: LanguageCode::TA, name: s!("Tamil"),             countries: vh![ CountryCode: LK, SG ] },
		LanguageCode::TE: Language { code: LanguageCode::TE, name: s!("Telugu"),            countries: vh![] },
		LanguageCode::TG: Language { code: LanguageCode::TG, name: s!("Tajik"),             countries: vh![ CountryCode: TJ ] },
		LanguageCode::TH: Language { code: LanguageCode::TH, name: s!("Thai"),              countries: vh![ CountryCode: TH ] },
		LanguageCode::TI: Language { code: LanguageCode::TI, name: s!("Tigrinya"),          countries: vh![ CountryCode: ER, ET ] },
		LanguageCode::TK: Language { code: LanguageCode::TK, name: s!("Turkmen"),           countries: vh![ CountryCode: TM ] },
		LanguageCode::TL: Language { code: LanguageCode::TL, name: s!("Tagalog"),           countries: vh![ CountryCode: PH ] },
		LanguageCode::TN: Language { code: LanguageCode::TN, name: s!("Tswana"),            countries: vh![ CountryCode: ZA, ZW ] },
		LanguageCode::TO: Language { code: LanguageCode::TO, name: s!("Tonga"),             countries: vh![ CountryCode: TO ] },
		LanguageCode::TR: Language { code: LanguageCode::TR, name: s!("Turkish"),           countries: vh![ CountryCode: CY, TR ] },
		LanguageCode::TS: Language { code: LanguageCode::TS, name: s!("Tsonga"),            countries: vh![ CountryCode: ZA ] },
		LanguageCode::TT: Language { code: LanguageCode::TT, name: s!("Tatar"),             countries: vh![] },
		LanguageCode::TW: Language { code: LanguageCode::TW, name: s!("Twi"),               countries: vh![] },
		LanguageCode::TY: Language { code: LanguageCode::TY, name: s!("Tahitian"),          countries: vh![] },
		LanguageCode::UG: Language { code: LanguageCode::UG, name: s!("Uighur"),            countries: vh![] },
		LanguageCode::UK: Language { code: LanguageCode::UK, name: s!("Ukrainian"),         countries: vh![ CountryCode: UA ] },
		LanguageCode::UR: Language { code: LanguageCode::UR, name: s!("Urdu"),              countries: vh![ CountryCode: PK ] },
		LanguageCode::UZ: Language { code: LanguageCode::UZ, name: s!("Uzbek"),             countries: vh![ CountryCode: UZ ] },
		LanguageCode::VE: Language { code: LanguageCode::VE, name: s!("Venda"),             countries: vh![ CountryCode: ZA, ZW ] },
		LanguageCode::VI: Language { code: LanguageCode::VI, name: s!("Vietnamese"),        countries: vh![ CountryCode: VN ] },
		LanguageCode::VO: Language { code: LanguageCode::VO, name: s!("Volapük"),           countries: vh![] },
		LanguageCode::WA: Language { code: LanguageCode::WA, name: s!("Walloon"),           countries: vh![] },
		LanguageCode::WO: Language { code: LanguageCode::WO, name: s!("Wolof"),             countries: vh![] },
		LanguageCode::XH: Language { code: LanguageCode::XH, name: s!("Xhosa"),             countries: vh![ CountryCode: ZA, ZW ] },
		LanguageCode::YI: Language { code: LanguageCode::YI, name: s!("Yiddish"),           countries: vh![] },
		LanguageCode::YO: Language { code: LanguageCode::YO, name: s!("Yoruba"),            countries: vh![] },
		LanguageCode::ZA: Language { code: LanguageCode::ZA, name: s!("Zhuang"),            countries: vh![] },
		LanguageCode::ZH: Language { code: LanguageCode::ZH, name: s!("Chinese"),           countries: vh![ CountryCode: CN, CX, HK, MO, SG, TW ] },
		LanguageCode::ZU: Language { code: LanguageCode::ZU, name: s!("Zulu"),              countries: vh![ CountryCode: ZA ] },
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


