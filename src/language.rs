//! Language-related types.
//! 
//! This module provides ISO 639-1 languages with alpha2 codes and basic names.
//! The languages and codes are provided as enums, for ease of use and
//! performance.
//! 
//! The languages are related to countries, and vice versa, making lookups easy.
//! The information comes from the ISO and Wikipedia, but notably there is no
//! ISO list of languages used in each country, so this information is sourced
//! from Wikipedia alone.
//! 
//! The language codes only exist in alpha2 form, as ISO 639-1 does not provide
//! any numeric equivalent.
//! 



//		Modules																											

#[cfg(test)]
#[path = "tests/language.rs"]
mod tests;



//		Packages																										

use crate::country::CountryCode;
use core::{
	fmt::{Debug, Display, self},
	str::FromStr,
};
use rubedo::{
	std::AsStr,
	sugar::{s, vh},
};
use serde::{Deserialize, Serialize};
use std::{
	collections::{HashMap, HashSet},
	sync::LazyLock,
};
use velcro::hash_map;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;



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
static LANGUAGES: LazyLock<HashMap<Language, LanguageInfo>> = LazyLock::new(|| {
	hash_map!{
		Language::AA: LanguageInfo { code: LanguageCode::AA, name: s!("Afar"),              countries: vh![ CountryCode: ET ] },
		Language::AB: LanguageInfo { code: LanguageCode::AB, name: s!("Abkhazian"),         countries: vh![] },
		Language::AE: LanguageInfo { code: LanguageCode::AE, name: s!("Avestan"),           countries: vh![] },
		Language::AF: LanguageInfo { code: LanguageCode::AF, name: s!("Afrikaans"),         countries: vh![ CountryCode: ZA ] },
		Language::AK: LanguageInfo { code: LanguageCode::AK, name: s!("Akan"),              countries: vh![] },
		Language::AM: LanguageInfo { code: LanguageCode::AM, name: s!("Amharic"),           countries: vh![ CountryCode: ET ] },
		Language::AN: LanguageInfo { code: LanguageCode::AN, name: s!("Aragonese"),         countries: vh![] },
		Language::AR: LanguageInfo { code: LanguageCode::AR, name: s!("Arabic"),            countries: vh![ CountryCode: AE, BH, DJ, DZ, EG, EH, IQ, JO, KM, KW, LB, LY, MA, MR, OM, PS, QA, SA, SD, SO, SY, TD, TN, YE ] },
		Language::AS: LanguageInfo { code: LanguageCode::AS, name: s!("Assamese"),          countries: vh![] },
		Language::AV: LanguageInfo { code: LanguageCode::AV, name: s!("Avaric"),            countries: vh![] },
		Language::AY: LanguageInfo { code: LanguageCode::AY, name: s!("Aymara"),            countries: vh![ CountryCode: BO, PE ] },
		Language::AZ: LanguageInfo { code: LanguageCode::AZ, name: s!("Azerbaijani"),       countries: vh![ CountryCode: AZ ] },
		Language::BA: LanguageInfo { code: LanguageCode::BA, name: s!("Bashkir"),           countries: vh![] },
		Language::BE: LanguageInfo { code: LanguageCode::BE, name: s!("Belarusian"),        countries: vh![ CountryCode: BY ] },
		Language::BG: LanguageInfo { code: LanguageCode::BG, name: s!("Bulgarian"),         countries: vh![ CountryCode: BG ] },
		Language::BI: LanguageInfo { code: LanguageCode::BI, name: s!("Bislama"),           countries: vh![ CountryCode: VU ] },
		Language::BM: LanguageInfo { code: LanguageCode::BM, name: s!("Bambara"),           countries: vh![ CountryCode: ML ] },
		Language::BN: LanguageInfo { code: LanguageCode::BN, name: s!("Bengali"),           countries: vh![ CountryCode: BD ] },
		Language::BO: LanguageInfo { code: LanguageCode::BO, name: s!("Tibetan"),           countries: vh![] },
		Language::BR: LanguageInfo { code: LanguageCode::BR, name: s!("Breton"),            countries: vh![] },
		Language::BS: LanguageInfo { code: LanguageCode::BS, name: s!("Bosnian"),           countries: vh![ CountryCode: BA ] },
		Language::CA: LanguageInfo { code: LanguageCode::CA, name: s!("Catalan"),           countries: vh![ CountryCode: AD ] },
		Language::CE: LanguageInfo { code: LanguageCode::CE, name: s!("Chechen"),           countries: vh![] },
		Language::CH: LanguageInfo { code: LanguageCode::CH, name: s!("Chamorro"),          countries: vh![ CountryCode: GU, MP ] },
		Language::CO: LanguageInfo { code: LanguageCode::CO, name: s!("Corsican"),          countries: vh![] },
		Language::CR: LanguageInfo { code: LanguageCode::CR, name: s!("Cree"),              countries: vh![] },
		Language::CS: LanguageInfo { code: LanguageCode::CS, name: s!("Czech"),             countries: vh![ CountryCode: CZ ] },
		Language::CU: LanguageInfo { code: LanguageCode::CU, name: s!("Church Slavonic"),   countries: vh![] },
		Language::CV: LanguageInfo { code: LanguageCode::CV, name: s!("Chuvash"),           countries: vh![] },
		Language::CY: LanguageInfo { code: LanguageCode::CY, name: s!("Welsh"),             countries: vh![] },
		Language::DA: LanguageInfo { code: LanguageCode::DA, name: s!("Danish"),            countries: vh![ CountryCode: DK, FO, GL ] },
		Language::DE: LanguageInfo { code: LanguageCode::DE, name: s!("German"),            countries: vh![ CountryCode: AT, BE, CH, DE, LI, LU ] },
		Language::DV: LanguageInfo { code: LanguageCode::DV, name: s!("Divehi"),            countries: vh![ CountryCode: MV ] },
		Language::DZ: LanguageInfo { code: LanguageCode::DZ, name: s!("Dzongkha"),          countries: vh![ CountryCode: BT ] },
		Language::EE: LanguageInfo { code: LanguageCode::EE, name: s!("Ewe"),               countries: vh![] },
		Language::EL: LanguageInfo { code: LanguageCode::EL, name: s!("Greek"),             countries: vh![ CountryCode: CY, GR ] },
		Language::EN: LanguageInfo { code: LanguageCode::EN, name: s!("English"),           countries: vh![ CountryCode: AG, AI, AS, AU, BB, BI, BM, BS, BW, BZ, CA, CC, CK, CM, CW, CX, DM, FJ, FK, FM, GB, GD, GG, GH, GI, GL, GM, GS, GU, GY, HK, HM, IE, IM, IN, IO, JE, JM, KE, KI, KN, KY, LC, LR, LS, MH, MP, MS, MT, MU, MW, NA, NF, NG, NR, NU, NZ, PG, PH, PK, PN, PR, PW, RW, SB, SC, SD, SG, SH, SL, SS, SX, SZ, TC, TK, TO, TT, TV, TZ, UG, UM, US, VC, VG, VI, VU, WS, ZA, ZM, ZW ] },
		Language::EO: LanguageInfo { code: LanguageCode::EO, name: s!("Esperanto"),         countries: vh![] },
		Language::ES: LanguageInfo { code: LanguageCode::ES, name: s!("Spanish"),           countries: vh![ CountryCode: AR, BO, CL, CO, CR, CU, DO, EC, EH, ES, GQ, GT, HN, MX, NI, PA, PE, PR, PY, SV, UY, VE ] },
		Language::ET: LanguageInfo { code: LanguageCode::ET, name: s!("Estonian"),          countries: vh![ CountryCode: EE ] },
		Language::EU: LanguageInfo { code: LanguageCode::EU, name: s!("Basque"),            countries: vh![] },
		Language::FA: LanguageInfo { code: LanguageCode::FA, name: s!("Persian"),           countries: vh![ CountryCode: AF, IR ] },
		Language::FF: LanguageInfo { code: LanguageCode::FF, name: s!("Fulah"),             countries: vh![ CountryCode: ML ] },
		Language::FI: LanguageInfo { code: LanguageCode::FI, name: s!("Finnish"),           countries: vh![ CountryCode: FI ] },
		Language::FJ: LanguageInfo { code: LanguageCode::FJ, name: s!("Fijian"),            countries: vh![ CountryCode: FJ ] },
		Language::FO: LanguageInfo { code: LanguageCode::FO, name: s!("Faroese"),           countries: vh![ CountryCode: FO ] },
		Language::FR: LanguageInfo { code: LanguageCode::FR, name: s!("French"),            countries: vh![ CountryCode: BE, BF, BI, BJ, BL, CA, CD, CF, CG, CH, CI, CM, DJ, FR, GA, GF, GN, GP, GQ, HT, JE, KM, LU, MC, MF, MG, MQ, NC, NE, PF, PM, RE, RW, SC, SN, TD, TF, TG, VU, WF, YT ] },
		Language::FY: LanguageInfo { code: LanguageCode::FY, name: s!("Western Frisian"),   countries: vh![] },
		Language::GA: LanguageInfo { code: LanguageCode::GA, name: s!("Irish"),             countries: vh![ CountryCode: IE ] },
		Language::GD: LanguageInfo { code: LanguageCode::GD, name: s!("Gaelic"),            countries: vh![] },
		Language::GL: LanguageInfo { code: LanguageCode::GL, name: s!("Galician"),          countries: vh![] },
		Language::GN: LanguageInfo { code: LanguageCode::GN, name: s!("Guarani"),           countries: vh![ CountryCode: BO, PY ] },
		Language::GU: LanguageInfo { code: LanguageCode::GU, name: s!("Gujarati"),          countries: vh![] },
		Language::GV: LanguageInfo { code: LanguageCode::GV, name: s!("Manx"),              countries: vh![ CountryCode: IM ] },
		Language::HA: LanguageInfo { code: LanguageCode::HA, name: s!("Hausa"),             countries: vh![] },
		Language::HE: LanguageInfo { code: LanguageCode::HE, name: s!("Hebrew"),            countries: vh![ CountryCode: IL ] },
		Language::HI: LanguageInfo { code: LanguageCode::HI, name: s!("Hindi"),             countries: vh![ CountryCode: IN ] },
		Language::HO: LanguageInfo { code: LanguageCode::HO, name: s!("Hiri Motu"),         countries: vh![ CountryCode: PG ] },
		Language::HR: LanguageInfo { code: LanguageCode::HR, name: s!("Croatian"),          countries: vh![ CountryCode: BA, HR, ME ] },
		Language::HT: LanguageInfo { code: LanguageCode::HT, name: s!("Haitian"),           countries: vh![ CountryCode: HT ] },
		Language::HU: LanguageInfo { code: LanguageCode::HU, name: s!("Hungarian"),         countries: vh![ CountryCode: HU ] },
		Language::HY: LanguageInfo { code: LanguageCode::HY, name: s!("Armenian"),          countries: vh![ CountryCode: AM ] },
		Language::HZ: LanguageInfo { code: LanguageCode::HZ, name: s!("Herero"),            countries: vh![] },
		Language::IA: LanguageInfo { code: LanguageCode::IA, name: s!("Interlingua"),       countries: vh![] },
		Language::ID: LanguageInfo { code: LanguageCode::ID, name: s!("Indonesian"),        countries: vh![ CountryCode: ID ] },
		Language::IE: LanguageInfo { code: LanguageCode::IE, name: s!("Interlingue"),       countries: vh![] },
		Language::IG: LanguageInfo { code: LanguageCode::IG, name: s!("Igbo"),              countries: vh![] },
		Language::II: LanguageInfo { code: LanguageCode::II, name: s!("Sichuan Yi"),        countries: vh![] },
		Language::IK: LanguageInfo { code: LanguageCode::IK, name: s!("Inupiaq"),           countries: vh![] },
		Language::IO: LanguageInfo { code: LanguageCode::IO, name: s!("Ido"),               countries: vh![] },
		Language::IS: LanguageInfo { code: LanguageCode::IS, name: s!("Icelandic"),         countries: vh![ CountryCode: IS ] },
		Language::IT: LanguageInfo { code: LanguageCode::IT, name: s!("Italian"),           countries: vh![ CountryCode: CH, IT, SM, VA ] },
		Language::IU: LanguageInfo { code: LanguageCode::IU, name: s!("Inuktitut"),         countries: vh![] },
		Language::JA: LanguageInfo { code: LanguageCode::JA, name: s!("Japanese"),          countries: vh![ CountryCode: JP ] },
		Language::JV: LanguageInfo { code: LanguageCode::JV, name: s!("Javanese"),          countries: vh![] },
		Language::KA: LanguageInfo { code: LanguageCode::KA, name: s!("Georgian"),          countries: vh![ CountryCode: GE ] },
		Language::KG: LanguageInfo { code: LanguageCode::KG, name: s!("Kongo"),             countries: vh![] },
		Language::KI: LanguageInfo { code: LanguageCode::KI, name: s!("Kikuyu"),            countries: vh![] },
		Language::KJ: LanguageInfo { code: LanguageCode::KJ, name: s!("Kuanyama"),          countries: vh![] },
		Language::KK: LanguageInfo { code: LanguageCode::KK, name: s!("Kazakh"),            countries: vh![ CountryCode: KZ ] },
		Language::KL: LanguageInfo { code: LanguageCode::KL, name: s!("Kalaallisut"),       countries: vh![] },
		Language::KM: LanguageInfo { code: LanguageCode::KM, name: s!("Central Khmer"),     countries: vh![ CountryCode: KH ] },
		Language::KN: LanguageInfo { code: LanguageCode::KN, name: s!("Kannada"),           countries: vh![] },
		Language::KO: LanguageInfo { code: LanguageCode::KO, name: s!("Korean"),            countries: vh![ CountryCode: KP, KR ] },
		Language::KR: LanguageInfo { code: LanguageCode::KR, name: s!("Kanuri"),            countries: vh![] },
		Language::KS: LanguageInfo { code: LanguageCode::KS, name: s!("Kashmiri"),          countries: vh![] },
		Language::KU: LanguageInfo { code: LanguageCode::KU, name: s!("Kurdish"),           countries: vh![ CountryCode: IQ ] },
		Language::KV: LanguageInfo { code: LanguageCode::KV, name: s!("Komi"),              countries: vh![] },
		Language::KW: LanguageInfo { code: LanguageCode::KW, name: s!("Cornish"),           countries: vh![] },
		Language::KY: LanguageInfo { code: LanguageCode::KY, name: s!("Kirghiz"),           countries: vh![ CountryCode: KG ] },
		Language::LA: LanguageInfo { code: LanguageCode::LA, name: s!("Latin"),             countries: vh![ CountryCode: VA ] },
		Language::LB: LanguageInfo { code: LanguageCode::LB, name: s!("Luxembourgish"),     countries: vh![ CountryCode: LU ] },
		Language::LG: LanguageInfo { code: LanguageCode::LG, name: s!("Ganda"),             countries: vh![] },
		Language::LI: LanguageInfo { code: LanguageCode::LI, name: s!("Limburgan"),         countries: vh![] },
		Language::LN: LanguageInfo { code: LanguageCode::LN, name: s!("Lingala"),           countries: vh![] },
		Language::LO: LanguageInfo { code: LanguageCode::LO, name: s!("Lao"),               countries: vh![ CountryCode: LA ] },
		Language::LT: LanguageInfo { code: LanguageCode::LT, name: s!("Lithuanian"),        countries: vh![ CountryCode: LT ] },
		Language::LU: LanguageInfo { code: LanguageCode::LU, name: s!("Luba-Katanga"),      countries: vh![] },
		Language::LV: LanguageInfo { code: LanguageCode::LV, name: s!("Latvian"),           countries: vh![ CountryCode: LV ] },
		Language::MG: LanguageInfo { code: LanguageCode::MG, name: s!("Malagasy"),          countries: vh![ CountryCode: MG ] },
		Language::MH: LanguageInfo { code: LanguageCode::MH, name: s!("Marshallese"),       countries: vh![ CountryCode: MH ] },
		Language::MI: LanguageInfo { code: LanguageCode::MI, name: s!("Maori"),             countries: vh![ CountryCode: NZ ] },
		Language::MK: LanguageInfo { code: LanguageCode::MK, name: s!("Macedonian"),        countries: vh![ CountryCode: MK ] },
		Language::ML: LanguageInfo { code: LanguageCode::ML, name: s!("Malayalam"),         countries: vh![] },
		Language::MN: LanguageInfo { code: LanguageCode::MN, name: s!("Mongolian"),         countries: vh![ CountryCode: MN ] },
		Language::MR: LanguageInfo { code: LanguageCode::MR, name: s!("Marathi"),           countries: vh![] },
		Language::MS: LanguageInfo { code: LanguageCode::MS, name: s!("Malay"),             countries: vh![ CountryCode: BN, CC, CX, MY, SG ] },
		Language::MT: LanguageInfo { code: LanguageCode::MT, name: s!("Maltese"),           countries: vh![ CountryCode: MT ] },
		Language::MY: LanguageInfo { code: LanguageCode::MY, name: s!("Burmese"),           countries: vh![ CountryCode: MM ] },
		Language::NA: LanguageInfo { code: LanguageCode::NA, name: s!("Nauru"),             countries: vh![ CountryCode: NR ] },
		Language::NB: LanguageInfo { code: LanguageCode::NB, name: s!("Norwegian Bokmål"),  countries: vh![] },
		Language::ND: LanguageInfo { code: LanguageCode::ND, name: s!("North Ndebele"),     countries: vh![] },
		Language::NE: LanguageInfo { code: LanguageCode::NE, name: s!("Nepali"),            countries: vh![ CountryCode: NP ] },
		Language::NG: LanguageInfo { code: LanguageCode::NG, name: s!("Ndonga"),            countries: vh![] },
		Language::NL: LanguageInfo { code: LanguageCode::NL, name: s!("Dutch"),             countries: vh![ CountryCode: AW, BE, BQ, CW, NL, SR, SX ] },
		Language::NN: LanguageInfo { code: LanguageCode::NN, name: s!("Norwegian Nynorsk"), countries: vh![] },
		Language::NO: LanguageInfo { code: LanguageCode::NO, name: s!("Norwegian"),         countries: vh![ CountryCode: BV, NO, SJ ] },
		Language::NR: LanguageInfo { code: LanguageCode::NR, name: s!("South Ndebele"),     countries: vh![ CountryCode: ZA, ZW ] },
		Language::NV: LanguageInfo { code: LanguageCode::NV, name: s!("Navajo"),            countries: vh![] },
		Language::NY: LanguageInfo { code: LanguageCode::NY, name: s!("Chichewa"),          countries: vh![ CountryCode: MW, ZW ] },
		Language::OC: LanguageInfo { code: LanguageCode::OC, name: s!("Occitan"),           countries: vh![] },
		Language::OJ: LanguageInfo { code: LanguageCode::OJ, name: s!("Ojibwa"),            countries: vh![] },
		Language::OM: LanguageInfo { code: LanguageCode::OM, name: s!("Oromo"),             countries: vh![ CountryCode: ET ] },
		Language::OR: LanguageInfo { code: LanguageCode::OR, name: s!("Oriya"),             countries: vh![] },
		Language::OS: LanguageInfo { code: LanguageCode::OS, name: s!("Ossetian"),          countries: vh![] },
		Language::PA: LanguageInfo { code: LanguageCode::PA, name: s!("Punjabi"),           countries: vh![] },
		Language::PI: LanguageInfo { code: LanguageCode::PI, name: s!("Pali"),              countries: vh![] },
		Language::PL: LanguageInfo { code: LanguageCode::PL, name: s!("Polish"),            countries: vh![ CountryCode: PL ] },
		Language::PS: LanguageInfo { code: LanguageCode::PS, name: s!("Pashto"),            countries: vh![ CountryCode: AF ] },
		Language::PT: LanguageInfo { code: LanguageCode::PT, name: s!("Portuguese"),        countries: vh![ CountryCode: AO, BR, CV, GW, MO, MZ, PT, ST, TL, GQ ] },
		Language::QU: LanguageInfo { code: LanguageCode::QU, name: s!("Quechua"),           countries: vh![ CountryCode: BO, EC, PE ] },
		Language::RM: LanguageInfo { code: LanguageCode::RM, name: s!("Romansh"),           countries: vh![ CountryCode: CH ] },
		Language::RN: LanguageInfo { code: LanguageCode::RN, name: s!("Rundi"),             countries: vh![ CountryCode: BI ] },
		Language::RO: LanguageInfo { code: LanguageCode::RO, name: s!("Romanian"),          countries: vh![ CountryCode: MD, RO ] },
		Language::RU: LanguageInfo { code: LanguageCode::RU, name: s!("Russian"),           countries: vh![ CountryCode: BY, KG, KZ, RU ] },
		Language::RW: LanguageInfo { code: LanguageCode::RW, name: s!("Kinyarwanda"),       countries: vh![ CountryCode: RW ] },
		Language::SA: LanguageInfo { code: LanguageCode::SA, name: s!("Sanskrit"),          countries: vh![] },
		Language::SC: LanguageInfo { code: LanguageCode::SC, name: s!("Sardinian"),         countries: vh![] },
		Language::SD: LanguageInfo { code: LanguageCode::SD, name: s!("Sindhi"),            countries: vh![] },
		Language::SE: LanguageInfo { code: LanguageCode::SE, name: s!("Northern Sami"),     countries: vh![] },
		Language::SG: LanguageInfo { code: LanguageCode::SG, name: s!("Sango"),             countries: vh![ CountryCode: CF ] },
		Language::SI: LanguageInfo { code: LanguageCode::SI, name: s!("Sinhala"),           countries: vh![ CountryCode: LK ] },
		Language::SK: LanguageInfo { code: LanguageCode::SK, name: s!("Slovak"),            countries: vh![ CountryCode: CZ, SK ] },
		Language::SL: LanguageInfo { code: LanguageCode::SL, name: s!("Slovenian"),         countries: vh![ CountryCode: SI ] },
		Language::SM: LanguageInfo { code: LanguageCode::SM, name: s!("Samoan"),            countries: vh![ CountryCode: AS, WS ] },
		Language::SN: LanguageInfo { code: LanguageCode::SN, name: s!("Shona"),             countries: vh![ CountryCode: ZW ] },
		Language::SO: LanguageInfo { code: LanguageCode::SO, name: s!("Somali"),            countries: vh![ CountryCode: ET, SO ] },
		Language::SQ: LanguageInfo { code: LanguageCode::SQ, name: s!("Albanian"),          countries: vh![ CountryCode: AL, MK ] },
		Language::SR: LanguageInfo { code: LanguageCode::SR, name: s!("Serbian"),           countries: vh![ CountryCode: BA, ME, RS ] },
		Language::SS: LanguageInfo { code: LanguageCode::SS, name: s!("Swati"),             countries: vh![ CountryCode: SZ, ZA ] },
		Language::ST: LanguageInfo { code: LanguageCode::ST, name: s!("Southern Sotho"),    countries: vh![ CountryCode: LS, ZA, ZW ] },
		Language::SU: LanguageInfo { code: LanguageCode::SU, name: s!("Sundanese"),         countries: vh![] },
		Language::SV: LanguageInfo { code: LanguageCode::SV, name: s!("Swedish"),           countries: vh![ CountryCode: AX, FI, SE ] },
		Language::SW: LanguageInfo { code: LanguageCode::SW, name: s!("Swahili"),           countries: vh![ CountryCode: KE, RW, TZ, UG ] },
		Language::TA: LanguageInfo { code: LanguageCode::TA, name: s!("Tamil"),             countries: vh![ CountryCode: LK, SG ] },
		Language::TE: LanguageInfo { code: LanguageCode::TE, name: s!("Telugu"),            countries: vh![] },
		Language::TG: LanguageInfo { code: LanguageCode::TG, name: s!("Tajik"),             countries: vh![ CountryCode: TJ ] },
		Language::TH: LanguageInfo { code: LanguageCode::TH, name: s!("Thai"),              countries: vh![ CountryCode: TH ] },
		Language::TI: LanguageInfo { code: LanguageCode::TI, name: s!("Tigrinya"),          countries: vh![ CountryCode: ER, ET ] },
		Language::TK: LanguageInfo { code: LanguageCode::TK, name: s!("Turkmen"),           countries: vh![ CountryCode: TM ] },
		Language::TL: LanguageInfo { code: LanguageCode::TL, name: s!("Tagalog"),           countries: vh![ CountryCode: PH ] },
		Language::TN: LanguageInfo { code: LanguageCode::TN, name: s!("Tswana"),            countries: vh![ CountryCode: ZA, ZW ] },
		Language::TO: LanguageInfo { code: LanguageCode::TO, name: s!("Tonga"),             countries: vh![ CountryCode: TO ] },
		Language::TR: LanguageInfo { code: LanguageCode::TR, name: s!("Turkish"),           countries: vh![ CountryCode: CY, TR ] },
		Language::TS: LanguageInfo { code: LanguageCode::TS, name: s!("Tsonga"),            countries: vh![ CountryCode: ZA ] },
		Language::TT: LanguageInfo { code: LanguageCode::TT, name: s!("Tatar"),             countries: vh![] },
		Language::TW: LanguageInfo { code: LanguageCode::TW, name: s!("Twi"),               countries: vh![] },
		Language::TY: LanguageInfo { code: LanguageCode::TY, name: s!("Tahitian"),          countries: vh![] },
		Language::UG: LanguageInfo { code: LanguageCode::UG, name: s!("Uighur"),            countries: vh![] },
		Language::UK: LanguageInfo { code: LanguageCode::UK, name: s!("Ukrainian"),         countries: vh![ CountryCode: UA ] },
		Language::UR: LanguageInfo { code: LanguageCode::UR, name: s!("Urdu"),              countries: vh![ CountryCode: PK ] },
		Language::UZ: LanguageInfo { code: LanguageCode::UZ, name: s!("Uzbek"),             countries: vh![ CountryCode: UZ ] },
		Language::VE: LanguageInfo { code: LanguageCode::VE, name: s!("Venda"),             countries: vh![ CountryCode: ZA, ZW ] },
		Language::VI: LanguageInfo { code: LanguageCode::VI, name: s!("Vietnamese"),        countries: vh![ CountryCode: VN ] },
		Language::VO: LanguageInfo { code: LanguageCode::VO, name: s!("Volapük"),           countries: vh![] },
		Language::WA: LanguageInfo { code: LanguageCode::WA, name: s!("Walloon"),           countries: vh![] },
		Language::WO: LanguageInfo { code: LanguageCode::WO, name: s!("Wolof"),             countries: vh![] },
		Language::XH: LanguageInfo { code: LanguageCode::XH, name: s!("Xhosa"),             countries: vh![ CountryCode: ZA, ZW ] },
		Language::YI: LanguageInfo { code: LanguageCode::YI, name: s!("Yiddish"),           countries: vh![] },
		Language::YO: LanguageInfo { code: LanguageCode::YO, name: s!("Yoruba"),            countries: vh![] },
		Language::ZA: LanguageInfo { code: LanguageCode::ZA, name: s!("Zhuang"),            countries: vh![] },
		Language::ZH: LanguageInfo { code: LanguageCode::ZH, name: s!("Chinese"),           countries: vh![ CountryCode: CN, CX, HK, MO, SG, TW ] },
		Language::ZU: LanguageInfo { code: LanguageCode::ZU, name: s!("Zulu"),              countries: vh![ CountryCode: ZA ] },
	}
});



//		Enums																											

//		Language																
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
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[serde(into = "String", try_from = "String")]
#[non_exhaustive]
pub enum Language {
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

//󰭅		Language																
impl Language {
	//		all																	
	/// Returns all the languages.
	pub fn all() -> Vec<Self> {
		LANGUAGES.keys().copied().collect()
	}
	
	//		info																
	/// Returns the `LanguageInfo` instance corresponding to the `Language`.
	/// 
	/// This method provides an easy way to get to the associated `LanguageInfo`
	/// instance from a `Language` enum variant.
	/// 
	#[must_use]
	fn info(self) -> &'static LanguageInfo {
		#[expect(clippy::unwrap_used, reason = "Infallible")]
		//	This should be infallible. If it isn't, then the data is wrong, and one
		//	of the languages is missing from the list, which is a bug.
		LANGUAGES.get(&self).unwrap()
	}
	
	//		name																
	/// Returns the name of the language.
	#[cfg_attr(feature = "utoipa", expect(clippy::same_name_method, reason = "Doesn't matter"))]
	#[must_use]
	pub fn name(&self) -> &str {
		&self.info().name
	}
	
	//		code																
	/// Returns the language code.
	#[must_use]
	pub fn code(&self) -> LanguageCode {
		self.info().code
	}
	
	//		countries															
	/// Returns the countries where the language is used.
	#[must_use]
	pub fn countries(&self) -> &HashSet<CountryCode> {
		&self.info().countries
	}
}

//󰭅		AsStr																	
impl AsStr for Language {
	//		as_str																
	fn as_str(&self) -> &str {
		&self.info().name
	}
}

//󰭅		Debug																	
impl Debug for Language {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.info().code.as_str(), self.as_str())
	}
}

//󰭅		Display																	
impl Display for Language {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

//󰭅		From<Language> for String												
impl From<Language> for String {
	//		from																
	fn from(language: Language) -> Self {
		language.to_string()
	}
}

//󰭅		FromStr																	
impl FromStr for Language {
	type Err = String;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		LANGUAGES
			.values()
			.find(|info| info.name == s)
			.map_or_else(
				||     Err(format!("Invalid Language: {s}")),
				|info| Ok(info.code.language())
			)
	}
}

//󰭅		TryFrom<String>															
impl TryFrom<String> for Language {
	type Error = String;
	
	//		try_from															
	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.as_str().parse()
	}
}

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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
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

//󰭅		LanguageCode															
impl LanguageCode {
	//		all																	
	/// Returns all the language codes.
	pub fn all() -> Vec<Self> {
		LANGUAGES.values().map(|info| info.code).collect()
	}
	
	//		language															
	/// Returns the `Language` variant corresponding to the `LanguageCode`.
	/// 
	/// This method provides an easy way to get to the associated `Language`
	/// variant from a `LanguageCode` enum variant.
	/// 
	#[expect(clippy::too_many_lines, reason = "Data not logic")]
	#[must_use]
	pub const fn language(&self) -> Language {
		match *self {
			Self::AA => Language::AA,
			Self::AB => Language::AB,
			Self::AE => Language::AE,
			Self::AF => Language::AF,
			Self::AK => Language::AK,
			Self::AM => Language::AM,
			Self::AN => Language::AN,
			Self::AR => Language::AR,
			Self::AS => Language::AS,
			Self::AV => Language::AV,
			Self::AY => Language::AY,
			Self::AZ => Language::AZ,
			Self::BA => Language::BA,
			Self::BE => Language::BE,
			Self::BG => Language::BG,
			Self::BI => Language::BI,
			Self::BM => Language::BM,
			Self::BN => Language::BN,
			Self::BO => Language::BO,
			Self::BR => Language::BR,
			Self::BS => Language::BS,
			Self::CA => Language::CA,
			Self::CE => Language::CE,
			Self::CH => Language::CH,
			Self::CO => Language::CO,
			Self::CR => Language::CR,
			Self::CS => Language::CS,
			Self::CU => Language::CU,
			Self::CV => Language::CV,
			Self::CY => Language::CY,
			Self::DA => Language::DA,
			Self::DE => Language::DE,
			Self::DV => Language::DV,
			Self::DZ => Language::DZ,
			Self::EE => Language::EE,
			Self::EL => Language::EL,
			Self::EN => Language::EN,
			Self::EO => Language::EO,
			Self::ES => Language::ES,
			Self::ET => Language::ET,
			Self::EU => Language::EU,
			Self::FA => Language::FA,
			Self::FF => Language::FF,
			Self::FI => Language::FI,
			Self::FJ => Language::FJ,
			Self::FO => Language::FO,
			Self::FR => Language::FR,
			Self::FY => Language::FY,
			Self::GA => Language::GA,
			Self::GD => Language::GD,
			Self::GL => Language::GL,
			Self::GN => Language::GN,
			Self::GU => Language::GU,
			Self::GV => Language::GV,
			Self::HA => Language::HA,
			Self::HE => Language::HE,
			Self::HI => Language::HI,
			Self::HO => Language::HO,
			Self::HR => Language::HR,
			Self::HT => Language::HT,
			Self::HU => Language::HU,
			Self::HY => Language::HY,
			Self::HZ => Language::HZ,
			Self::IA => Language::IA,
			Self::ID => Language::ID,
			Self::IE => Language::IE,
			Self::IG => Language::IG,
			Self::II => Language::II,
			Self::IK => Language::IK,
			Self::IO => Language::IO,
			Self::IS => Language::IS,
			Self::IT => Language::IT,
			Self::IU => Language::IU,
			Self::JA => Language::JA,
			Self::JV => Language::JV,
			Self::KA => Language::KA,
			Self::KG => Language::KG,
			Self::KI => Language::KI,
			Self::KJ => Language::KJ,
			Self::KK => Language::KK,
			Self::KL => Language::KL,
			Self::KM => Language::KM,
			Self::KN => Language::KN,
			Self::KO => Language::KO,
			Self::KR => Language::KR,
			Self::KS => Language::KS,
			Self::KU => Language::KU,
			Self::KV => Language::KV,
			Self::KW => Language::KW,
			Self::KY => Language::KY,
			Self::LA => Language::LA,
			Self::LB => Language::LB,
			Self::LG => Language::LG,
			Self::LI => Language::LI,
			Self::LN => Language::LN,
			Self::LO => Language::LO,
			Self::LT => Language::LT,
			Self::LU => Language::LU,
			Self::LV => Language::LV,
			Self::MG => Language::MG,
			Self::MH => Language::MH,
			Self::MI => Language::MI,
			Self::MK => Language::MK,
			Self::ML => Language::ML,
			Self::MN => Language::MN,
			Self::MR => Language::MR,
			Self::MS => Language::MS,
			Self::MT => Language::MT,
			Self::MY => Language::MY,
			Self::NA => Language::NA,
			Self::NB => Language::NB,
			Self::ND => Language::ND,
			Self::NE => Language::NE,
			Self::NG => Language::NG,
			Self::NL => Language::NL,
			Self::NN => Language::NN,
			Self::NO => Language::NO,
			Self::NR => Language::NR,
			Self::NV => Language::NV,
			Self::NY => Language::NY,
			Self::OC => Language::OC,
			Self::OJ => Language::OJ,
			Self::OM => Language::OM,
			Self::OR => Language::OR,
			Self::OS => Language::OS,
			Self::PA => Language::PA,
			Self::PI => Language::PI,
			Self::PL => Language::PL,
			Self::PS => Language::PS,
			Self::PT => Language::PT,
			Self::QU => Language::QU,
			Self::RM => Language::RM,
			Self::RN => Language::RN,
			Self::RO => Language::RO,
			Self::RU => Language::RU,
			Self::RW => Language::RW,
			Self::SA => Language::SA,
			Self::SC => Language::SC,
			Self::SD => Language::SD,
			Self::SE => Language::SE,
			Self::SG => Language::SG,
			Self::SI => Language::SI,
			Self::SK => Language::SK,
			Self::SL => Language::SL,
			Self::SM => Language::SM,
			Self::SN => Language::SN,
			Self::SO => Language::SO,
			Self::SQ => Language::SQ,
			Self::SR => Language::SR,
			Self::SS => Language::SS,
			Self::ST => Language::ST,
			Self::SU => Language::SU,
			Self::SV => Language::SV,
			Self::SW => Language::SW,
			Self::TA => Language::TA,
			Self::TE => Language::TE,
			Self::TG => Language::TG,
			Self::TH => Language::TH,
			Self::TI => Language::TI,
			Self::TK => Language::TK,
			Self::TL => Language::TL,
			Self::TN => Language::TN,
			Self::TO => Language::TO,
			Self::TR => Language::TR,
			Self::TS => Language::TS,
			Self::TT => Language::TT,
			Self::TW => Language::TW,
			Self::TY => Language::TY,
			Self::UG => Language::UG,
			Self::UK => Language::UK,
			Self::UR => Language::UR,
			Self::UZ => Language::UZ,
			Self::VE => Language::VE,
			Self::VI => Language::VI,
			Self::VO => Language::VO,
			Self::WA => Language::WA,
			Self::WO => Language::WO,
			Self::XH => Language::XH,
			Self::YI => Language::YI,
			Self::YO => Language::YO,
			Self::ZA => Language::ZA,
			Self::ZH => Language::ZH,
			Self::ZU => Language::ZU,
		}
	}
}

//󰭅		AsStr																	
impl AsStr for LanguageCode {
	//		as_str																
	#[expect(clippy::too_many_lines, reason = "Data not logic")]
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

//󰭅		Display																	
impl Display for LanguageCode {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

//󰭅		From<LanguageCode> for String											
impl From<LanguageCode> for String {
	//		from																
	fn from(code: LanguageCode) -> Self {
		code.to_string()
	}
}

//󰭅		FromStr																	
impl FromStr for LanguageCode {
	type Err = String;
	
	//		from_str															
	#[expect(clippy::too_many_lines, reason = "Data not logic")]
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

//󰭅		TryFrom<String>															
impl TryFrom<String> for LanguageCode {
	type Error = String;
	
	//		try_from															
	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.as_str().parse()
	}
}



//		Structs																											

//		LanguageInfo															
/// Language information.
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
/// * [`Language`]
/// * [`LanguageCode`]
/// 
#[non_exhaustive]
struct LanguageInfo {
	//		Private properties													
	/// The name of the language.
	name:      String,
	
	/// The language code. For more information, see [`LanguageCode`].
	code:      LanguageCode,
	
	/// The countries where the language is used.
	countries: HashSet<CountryCode>,
}


