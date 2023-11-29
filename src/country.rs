//! Country-related types.



//		Packages

use crate::{
	currency::CurrencyCode,
	language::LanguageCode,
};
use core::{
	fmt::{Display, self},
	str::FromStr,
};
use once_cell::sync::Lazy;
use rubedo::sugar::s;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use utoipa::ToSchema;
use velcro::{hash_map, hash_set};



//		Constants

/// The possible countries.
/// 
/// # Data sources
///
/// The list of codes and other country information is available from
/// [the ISO site](https://www.iso.org/iso-3166-country-codes.html), and from
/// [Wikipedia](https://en.wikipedia.org/wiki/ISO_3166-1).
/// 
/// # See also
/// 
/// * [`CountryCode`]
/// * [`Country`]
/// 
pub static COUNTRIES: Lazy<HashMap<CountryCode, Country>> = Lazy::new(|| {
	hash_map!{
		CountryCode::AD: Country { code: CountryCode::AD, name: s!("Andorra"),                                              currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::CA ] },
		CountryCode::AE: Country { code: CountryCode::AE, name: s!("United Arab Emirates"),                                 currencies: hash_set![ CurrencyCode::AED ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::AF: Country { code: CountryCode::AF, name: s!("Afghanistan"),                                          currencies: hash_set![ CurrencyCode::AFN ],                                       languages: hash_set![ LanguageCode::FA, LanguageCode::PS ] },
		CountryCode::AG: Country { code: CountryCode::AG, name: s!("Antigua and Barbuda"),                                  currencies: hash_set![ CurrencyCode::XCD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::AI: Country { code: CountryCode::AI, name: s!("Anguilla"),                                             currencies: hash_set![ CurrencyCode::XCD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::AL: Country { code: CountryCode::AL, name: s!("Albania"),                                              currencies: hash_set![ CurrencyCode::ALL ],                                       languages: hash_set![ LanguageCode::SQ ] },
		CountryCode::AM: Country { code: CountryCode::AM, name: s!("Armenia"),                                              currencies: hash_set![ CurrencyCode::AMD ],                                       languages: hash_set![ LanguageCode::HY ] },
		CountryCode::AO: Country { code: CountryCode::AO, name: s!("Angola"),                                               currencies: hash_set![ CurrencyCode::AOA ],                                       languages: hash_set![ LanguageCode::PT ] },
		CountryCode::AQ: Country { code: CountryCode::AQ, name: s!("Antarctica"),                                           currencies: hash_set![],                                                          languages: hash_set![] },
		CountryCode::AR: Country { code: CountryCode::AR, name: s!("Argentina"),                                            currencies: hash_set![ CurrencyCode::ARS ],                                       languages: hash_set![ LanguageCode::ES ] },
		CountryCode::AS: Country { code: CountryCode::AS, name: s!("American Samoa"),                                       currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::SM ] },
		CountryCode::AT: Country { code: CountryCode::AT, name: s!("Austria"),                                              currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::DE ] },
		CountryCode::AU: Country { code: CountryCode::AU, name: s!("Australia"),                                            currencies: hash_set![ CurrencyCode::AUD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::AW: Country { code: CountryCode::AW, name: s!("Aruba"),                                                currencies: hash_set![ CurrencyCode::AWG ],                                       languages: hash_set![ LanguageCode::NL ] },
		CountryCode::AX: Country { code: CountryCode::AX, name: s!("Åland Islands"),                                        currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::SV ] },
		CountryCode::AZ: Country { code: CountryCode::AZ, name: s!("Azerbaijan"),                                           currencies: hash_set![ CurrencyCode::AZN ],                                       languages: hash_set![ LanguageCode::AZ ] },
		CountryCode::BA: Country { code: CountryCode::BA, name: s!("Bosnia and Herzegovina"),                               currencies: hash_set![ CurrencyCode::BAM ],                                       languages: hash_set![ LanguageCode::BS, LanguageCode::HR, LanguageCode::SR ] },
		CountryCode::BB: Country { code: CountryCode::BB, name: s!("Barbados"),                                             currencies: hash_set![ CurrencyCode::BBD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::BD: Country { code: CountryCode::BD, name: s!("Bangladesh"),                                           currencies: hash_set![ CurrencyCode::BDT ],                                       languages: hash_set![ LanguageCode::BN ] },
		CountryCode::BE: Country { code: CountryCode::BE, name: s!("Belgium"),                                              currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::DE, LanguageCode::FR, LanguageCode::NL ] },
		CountryCode::BF: Country { code: CountryCode::BF, name: s!("Burkina Faso"),                                         currencies: hash_set![ CurrencyCode::XOF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::BG: Country { code: CountryCode::BG, name: s!("Bulgaria"),                                             currencies: hash_set![ CurrencyCode::BGN ],                                       languages: hash_set![ LanguageCode::BG ] },
		CountryCode::BH: Country { code: CountryCode::BH, name: s!("Bahrain"),                                              currencies: hash_set![ CurrencyCode::BHD ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::BI: Country { code: CountryCode::BI, name: s!("Burundi"),                                              currencies: hash_set![ CurrencyCode::BIF ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::FR, LanguageCode::RN ] },
		CountryCode::BJ: Country { code: CountryCode::BJ, name: s!("Benin"),                                                currencies: hash_set![ CurrencyCode::XOF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::BL: Country { code: CountryCode::BL, name: s!("Saint Barthélemy"),                                     currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::BM: Country { code: CountryCode::BM, name: s!("Bermuda"),                                              currencies: hash_set![ CurrencyCode::BMD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::BN: Country { code: CountryCode::BN, name: s!("Brunei Darussalam"),                                    currencies: hash_set![ CurrencyCode::BND ],                                       languages: hash_set![ LanguageCode::MS ] },
		CountryCode::BO: Country { code: CountryCode::BO, name: s!("Bolivia (Plurinational State of)"),                     currencies: hash_set![ CurrencyCode::BOB, CurrencyCode::BOV ],                    languages: hash_set![ LanguageCode::AY, LanguageCode::ES, LanguageCode::GN, LanguageCode::QU ] },
		CountryCode::BQ: Country { code: CountryCode::BQ, name: s!("Bonaire, Sint Eustatius and Saba"),                     currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::NL ] },
		CountryCode::BR: Country { code: CountryCode::BR, name: s!("Brazil"),                                               currencies: hash_set![ CurrencyCode::BRL ],                                       languages: hash_set![ LanguageCode::PT ] },
		CountryCode::BS: Country { code: CountryCode::BS, name: s!("Bahamas"),                                              currencies: hash_set![ CurrencyCode::BSD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::BT: Country { code: CountryCode::BT, name: s!("Bhutan"),                                               currencies: hash_set![ CurrencyCode::BTN, CurrencyCode::INR ],                    languages: hash_set![ LanguageCode::DZ ] },
		CountryCode::BV: Country { code: CountryCode::BV, name: s!("Bouvet Island"),                                        currencies: hash_set![ CurrencyCode::NOK ],                                       languages: hash_set![ LanguageCode::NO ] },
		CountryCode::BW: Country { code: CountryCode::BW, name: s!("Botswana"),                                             currencies: hash_set![ CurrencyCode::BWP ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::BY: Country { code: CountryCode::BY, name: s!("Belarus"),                                              currencies: hash_set![ CurrencyCode::BYN ],                                       languages: hash_set![ LanguageCode::BE, LanguageCode::RU ] },
		CountryCode::BZ: Country { code: CountryCode::BZ, name: s!("Belize"),                                               currencies: hash_set![ CurrencyCode::BZD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::CA: Country { code: CountryCode::CA, name: s!("Canada"),                                               currencies: hash_set![ CurrencyCode::CAD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::FR ] },
		CountryCode::CC: Country { code: CountryCode::CC, name: s!("Cocos (Keeling) Islands"),                              currencies: hash_set![ CurrencyCode::AUD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::MS ] },
		CountryCode::CD: Country { code: CountryCode::CD, name: s!("Congo, Democratic Republic of the"),                    currencies: hash_set![ CurrencyCode::CDF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::CF: Country { code: CountryCode::CF, name: s!("Central African Republic"),                             currencies: hash_set![ CurrencyCode::XAF ],                                       languages: hash_set![ LanguageCode::FR, LanguageCode::SG ] },
		CountryCode::CG: Country { code: CountryCode::CG, name: s!("Congo"),                                                currencies: hash_set![ CurrencyCode::XAF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::CH: Country { code: CountryCode::CH, name: s!("Switzerland"),                                          currencies: hash_set![ CurrencyCode::CHE, CurrencyCode::CHF, CurrencyCode::CHW ], languages: hash_set![ LanguageCode::DE, LanguageCode::FR, LanguageCode::IT, LanguageCode::RM ] },
		CountryCode::CI: Country { code: CountryCode::CI, name: s!("Côte d'Ivoire"),                                        currencies: hash_set![ CurrencyCode::XOF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::CK: Country { code: CountryCode::CK, name: s!("Cook Islands"),                                         currencies: hash_set![ CurrencyCode::NZD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::CL: Country { code: CountryCode::CL, name: s!("Chile"),                                                currencies: hash_set![ CurrencyCode::CLF, CurrencyCode::CLP ],                    languages: hash_set![ LanguageCode::ES ] },
		CountryCode::CM: Country { code: CountryCode::CM, name: s!("Cameroon"),                                             currencies: hash_set![ CurrencyCode::XAF ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::FR ] },
		CountryCode::CN: Country { code: CountryCode::CN, name: s!("China"),                                                currencies: hash_set![ CurrencyCode::CNY ],                                       languages: hash_set![ LanguageCode::ZH ] },
		CountryCode::CO: Country { code: CountryCode::CO, name: s!("Colombia"),                                             currencies: hash_set![ CurrencyCode::COP, CurrencyCode::COU ],                    languages: hash_set![ LanguageCode::ES ] },
		CountryCode::CR: Country { code: CountryCode::CR, name: s!("Costa Rica"),                                           currencies: hash_set![ CurrencyCode::CRC ],                                       languages: hash_set![ LanguageCode::ES ] },
		CountryCode::CU: Country { code: CountryCode::CU, name: s!("Cuba"),                                                 currencies: hash_set![ CurrencyCode::CUP ],                                       languages: hash_set![ LanguageCode::ES ] },
		CountryCode::CV: Country { code: CountryCode::CV, name: s!("Cabo Verde"),                                           currencies: hash_set![ CurrencyCode::CVE ],                                       languages: hash_set![ LanguageCode::PT ] },
		CountryCode::CW: Country { code: CountryCode::CW, name: s!("Curaçao"),                                              currencies: hash_set![ CurrencyCode::ANG ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::NL ] },
		CountryCode::CX: Country { code: CountryCode::CX, name: s!("Christmas Island"),                                     currencies: hash_set![ CurrencyCode::AUD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::MS, LanguageCode::ZH ] },
		CountryCode::CY: Country { code: CountryCode::CY, name: s!("Cyprus"),                                               currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::EL, LanguageCode::TR ] },
		CountryCode::CZ: Country { code: CountryCode::CZ, name: s!("Czechia"),                                              currencies: hash_set![ CurrencyCode::CZK ],                                       languages: hash_set![ LanguageCode::CS, LanguageCode::SK ] },
		CountryCode::DE: Country { code: CountryCode::DE, name: s!("Germany"),                                              currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::DE ] },
		CountryCode::DJ: Country { code: CountryCode::DJ, name: s!("Djibouti"),                                             currencies: hash_set![ CurrencyCode::DJF ],                                       languages: hash_set![ LanguageCode::AR, LanguageCode::FR ] },
		CountryCode::DK: Country { code: CountryCode::DK, name: s!("Denmark"),                                              currencies: hash_set![ CurrencyCode::DKK ],                                       languages: hash_set![ LanguageCode::DA ] },
		CountryCode::DM: Country { code: CountryCode::DM, name: s!("Dominica"),                                             currencies: hash_set![ CurrencyCode::XCD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::DO: Country { code: CountryCode::DO, name: s!("Dominican Republic"),                                   currencies: hash_set![ CurrencyCode::DOP ],                                       languages: hash_set![ LanguageCode::ES ] },
		CountryCode::DZ: Country { code: CountryCode::DZ, name: s!("Algeria"),                                              currencies: hash_set![ CurrencyCode::DZD ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::EC: Country { code: CountryCode::EC, name: s!("Ecuador"),                                              currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::ES, LanguageCode::QU ] },
		CountryCode::EE: Country { code: CountryCode::EE, name: s!("Estonia"),                                              currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::ET ] },
		CountryCode::EG: Country { code: CountryCode::EG, name: s!("Egypt"),                                                currencies: hash_set![ CurrencyCode::EGP ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::EH: Country { code: CountryCode::EH, name: s!("Western Sahara"),                                       currencies: hash_set![ CurrencyCode::MAD ],                                       languages: hash_set![ LanguageCode::AR, LanguageCode::ES ] },
		CountryCode::ER: Country { code: CountryCode::ER, name: s!("Eritrea"),                                              currencies: hash_set![ CurrencyCode::ERN ],                                       languages: hash_set![ LanguageCode::TI ] },
		CountryCode::ES: Country { code: CountryCode::ES, name: s!("Spain"),                                                currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::ES ] },
		CountryCode::ET: Country { code: CountryCode::ET, name: s!("Ethiopia"),                                             currencies: hash_set![ CurrencyCode::ETB ],                                       languages: hash_set![ LanguageCode::AA, LanguageCode::AM, LanguageCode::OM, LanguageCode::SO, LanguageCode::TI ] },
		CountryCode::FI: Country { code: CountryCode::FI, name: s!("Finland"),                                              currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FI, LanguageCode::SV ] },
		CountryCode::FJ: Country { code: CountryCode::FJ, name: s!("Fiji"),                                                 currencies: hash_set![ CurrencyCode::FJD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::FJ ] },
		CountryCode::FK: Country { code: CountryCode::FK, name: s!("Falkland Islands (Malvinas)"),                          currencies: hash_set![ CurrencyCode::FKP ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::FM: Country { code: CountryCode::FM, name: s!("Micronesia (Federated States of)"),                     currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::FO: Country { code: CountryCode::FO, name: s!("Faroe Islands"),                                        currencies: hash_set![ CurrencyCode::DKK ],                                       languages: hash_set![ LanguageCode::DA, LanguageCode::FO ] },
		CountryCode::FR: Country { code: CountryCode::FR, name: s!("France"),                                               currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::GA: Country { code: CountryCode::GA, name: s!("Gabon"),                                                currencies: hash_set![ CurrencyCode::XAF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::GB: Country { code: CountryCode::GB, name: s!("United Kingdom of Great Britain and Northern Ireland"), currencies: hash_set![ CurrencyCode::GBP ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::GD: Country { code: CountryCode::GD, name: s!("Grenada"),                                              currencies: hash_set![ CurrencyCode::XCD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::GE: Country { code: CountryCode::GE, name: s!("Georgia"),                                              currencies: hash_set![ CurrencyCode::GEL ],                                       languages: hash_set![ LanguageCode::KA ] },
		CountryCode::GF: Country { code: CountryCode::GF, name: s!("French Guiana"),                                        currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::GG: Country { code: CountryCode::GG, name: s!("Guernsey"),                                             currencies: hash_set![ CurrencyCode::GBP ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::GH: Country { code: CountryCode::GH, name: s!("Ghana"),                                                currencies: hash_set![ CurrencyCode::GHS ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::GI: Country { code: CountryCode::GI, name: s!("Gibraltar"),                                            currencies: hash_set![ CurrencyCode::GIP ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::GL: Country { code: CountryCode::GL, name: s!("Greenland"),                                            currencies: hash_set![ CurrencyCode::DKK ],                                       languages: hash_set![ LanguageCode::DA, LanguageCode::EN ] },
		CountryCode::GM: Country { code: CountryCode::GM, name: s!("Gambia"),                                               currencies: hash_set![ CurrencyCode::GMD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::GN: Country { code: CountryCode::GN, name: s!("Guinea"),                                               currencies: hash_set![ CurrencyCode::GNF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::GP: Country { code: CountryCode::GP, name: s!("Guadeloupe"),                                           currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::GQ: Country { code: CountryCode::GQ, name: s!("Equatorial Guinea"),                                    currencies: hash_set![ CurrencyCode::XAF ],                                       languages: hash_set![ LanguageCode::ES, LanguageCode::FR, LanguageCode::PT ] },
		CountryCode::GR: Country { code: CountryCode::GR, name: s!("Greece"),                                               currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::EL ] },
		CountryCode::GS: Country { code: CountryCode::GS, name: s!("South Georgia and the South Sandwich Islands"),         currencies: hash_set![],                                                          languages: hash_set![ LanguageCode::EN ] },
		CountryCode::GT: Country { code: CountryCode::GT, name: s!("Guatemala"),                                            currencies: hash_set![ CurrencyCode::GTQ ],                                       languages: hash_set![ LanguageCode::ES ] },
		CountryCode::GU: Country { code: CountryCode::GU, name: s!("Guam"),                                                 currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::CH, LanguageCode::EN ] },
		CountryCode::GW: Country { code: CountryCode::GW, name: s!("Guinea-Bissau"),                                        currencies: hash_set![ CurrencyCode::XOF ],                                       languages: hash_set![ LanguageCode::PT ] },
		CountryCode::GY: Country { code: CountryCode::GY, name: s!("Guyana"),                                               currencies: hash_set![ CurrencyCode::GYD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::HK: Country { code: CountryCode::HK, name: s!("Hong Kong"),                                            currencies: hash_set![ CurrencyCode::HKD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::ZH ] },
		CountryCode::HM: Country { code: CountryCode::HM, name: s!("Heard Island and McDonald Islands"),                    currencies: hash_set![ CurrencyCode::AUD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::HN: Country { code: CountryCode::HN, name: s!("Honduras"),                                             currencies: hash_set![ CurrencyCode::HNL ],                                       languages: hash_set![ LanguageCode::ES ] },
		CountryCode::HR: Country { code: CountryCode::HR, name: s!("Croatia"),                                              currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::HR ] },
		CountryCode::HT: Country { code: CountryCode::HT, name: s!("Haiti"),                                                currencies: hash_set![ CurrencyCode::HTG ],                                       languages: hash_set![ LanguageCode::FR, LanguageCode::HT ] },
		CountryCode::HU: Country { code: CountryCode::HU, name: s!("Hungary"),                                              currencies: hash_set![ CurrencyCode::HUF ],                                       languages: hash_set![ LanguageCode::HU ] },
		CountryCode::ID: Country { code: CountryCode::ID, name: s!("Indonesia"),                                            currencies: hash_set![ CurrencyCode::IDR ],                                       languages: hash_set![ LanguageCode::ID ] },
		CountryCode::IE: Country { code: CountryCode::IE, name: s!("Ireland"),                                              currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::GA ] },
		CountryCode::IL: Country { code: CountryCode::IL, name: s!("Israel"),                                               currencies: hash_set![ CurrencyCode::ILS ],                                       languages: hash_set![ LanguageCode::HE ] },
		CountryCode::IM: Country { code: CountryCode::IM, name: s!("Isle of Man"),                                          currencies: hash_set![ CurrencyCode::GBP ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::GV ] },
		CountryCode::IN: Country { code: CountryCode::IN, name: s!("India"),                                                currencies: hash_set![ CurrencyCode::INR ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::HI ] },
		CountryCode::IO: Country { code: CountryCode::IO, name: s!("British Indian Ocean Territory"),                       currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::IQ: Country { code: CountryCode::IQ, name: s!("Iraq"),                                                 currencies: hash_set![ CurrencyCode::IQD ],                                       languages: hash_set![ LanguageCode::AR, LanguageCode::KU ] },
		CountryCode::IR: Country { code: CountryCode::IR, name: s!("Iran (Islamic Republic of)"),                           currencies: hash_set![ CurrencyCode::IRR ],                                       languages: hash_set![ LanguageCode::FA ] },
		CountryCode::IS: Country { code: CountryCode::IS, name: s!("Iceland"),                                              currencies: hash_set![ CurrencyCode::ISK ],                                       languages: hash_set![ LanguageCode::IS ] },
		CountryCode::IT: Country { code: CountryCode::IT, name: s!("Italy"),                                                currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::IT ] },
		CountryCode::JE: Country { code: CountryCode::JE, name: s!("Jersey"),                                               currencies: hash_set![ CurrencyCode::GBP ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::FR ] },
		CountryCode::JM: Country { code: CountryCode::JM, name: s!("Jamaica"),                                              currencies: hash_set![ CurrencyCode::JMD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::JO: Country { code: CountryCode::JO, name: s!("Jordan"),                                               currencies: hash_set![ CurrencyCode::JOD ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::JP: Country { code: CountryCode::JP, name: s!("Japan"),                                                currencies: hash_set![ CurrencyCode::JPY ],                                       languages: hash_set![ LanguageCode::JA ] },
		CountryCode::KE: Country { code: CountryCode::KE, name: s!("Kenya"),                                                currencies: hash_set![ CurrencyCode::KES ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::SW ] },
		CountryCode::KG: Country { code: CountryCode::KG, name: s!("Kyrgyzstan"),                                           currencies: hash_set![ CurrencyCode::KGS ],                                       languages: hash_set![ LanguageCode::KY, LanguageCode::RU ] },
		CountryCode::KH: Country { code: CountryCode::KH, name: s!("Cambodia"),                                             currencies: hash_set![ CurrencyCode::KHR ],                                       languages: hash_set![ LanguageCode::KM ] },
		CountryCode::KI: Country { code: CountryCode::KI, name: s!("Kiribati"),                                             currencies: hash_set![ CurrencyCode::AUD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::KM: Country { code: CountryCode::KM, name: s!("Comoros"),                                              currencies: hash_set![ CurrencyCode::KMF ],                                       languages: hash_set![ LanguageCode::AR, LanguageCode::FR ] },
		CountryCode::KN: Country { code: CountryCode::KN, name: s!("Saint Kitts and Nevis"),                                currencies: hash_set![ CurrencyCode::XCD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::KP: Country { code: CountryCode::KP, name: s!("Korea (Democratic People's Republic of)"),              currencies: hash_set![ CurrencyCode::KPW ],                                       languages: hash_set![ LanguageCode::KO ] },
		CountryCode::KR: Country { code: CountryCode::KR, name: s!("Korea, Republic of"),                                   currencies: hash_set![ CurrencyCode::KRW ],                                       languages: hash_set![ LanguageCode::KO ] },
		CountryCode::KW: Country { code: CountryCode::KW, name: s!("Kuwait"),                                               currencies: hash_set![ CurrencyCode::KWD ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::KY: Country { code: CountryCode::KY, name: s!("Cayman Islands"),                                       currencies: hash_set![ CurrencyCode::KYD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::KZ: Country { code: CountryCode::KZ, name: s!("Kazakhstan"),                                           currencies: hash_set![ CurrencyCode::KZT ],                                       languages: hash_set![ LanguageCode::KK, LanguageCode::RU ] },
		CountryCode::LA: Country { code: CountryCode::LA, name: s!("Lao People's Democratic Republic"),                     currencies: hash_set![ CurrencyCode::LAK ],                                       languages: hash_set![ LanguageCode::LO ] },
		CountryCode::LB: Country { code: CountryCode::LB, name: s!("Lebanon"),                                              currencies: hash_set![ CurrencyCode::LBP ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::LC: Country { code: CountryCode::LC, name: s!("Saint Lucia"),                                          currencies: hash_set![ CurrencyCode::XCD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::LI: Country { code: CountryCode::LI, name: s!("Liechtenstein"),                                        currencies: hash_set![ CurrencyCode::CHF ],                                       languages: hash_set![ LanguageCode::DE ] },
		CountryCode::LK: Country { code: CountryCode::LK, name: s!("Sri Lanka"),                                            currencies: hash_set![ CurrencyCode::LKR ],                                       languages: hash_set![ LanguageCode::SI, LanguageCode::TA ] },
		CountryCode::LR: Country { code: CountryCode::LR, name: s!("Liberia"),                                              currencies: hash_set![ CurrencyCode::LRD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::LS: Country { code: CountryCode::LS, name: s!("Lesotho"),                                              currencies: hash_set![ CurrencyCode::LSL, CurrencyCode::ZAR ],                    languages: hash_set![ LanguageCode::EN, LanguageCode::ST ] },
		CountryCode::LT: Country { code: CountryCode::LT, name: s!("Lithuania"),                                            currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::LT ] },
		CountryCode::LU: Country { code: CountryCode::LU, name: s!("Luxembourg"),                                           currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::DE, LanguageCode::FR, LanguageCode::LB ] },
		CountryCode::LV: Country { code: CountryCode::LV, name: s!("Latvia"),                                               currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::LV ] },
		CountryCode::LY: Country { code: CountryCode::LY, name: s!("Libya"),                                                currencies: hash_set![ CurrencyCode::LYD ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::MA: Country { code: CountryCode::MA, name: s!("Morocco"),                                              currencies: hash_set![ CurrencyCode::MAD ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::MC: Country { code: CountryCode::MC, name: s!("Monaco"),                                               currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::MD: Country { code: CountryCode::MD, name: s!("Moldova, Republic of"),                                 currencies: hash_set![ CurrencyCode::MDL ],                                       languages: hash_set![ LanguageCode::RO ] },
		CountryCode::ME: Country { code: CountryCode::ME, name: s!("Montenegro"),                                           currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::HR, LanguageCode::SR ] },
		CountryCode::MF: Country { code: CountryCode::MF, name: s!("Saint Martin (French part)"),                           currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::MG: Country { code: CountryCode::MG, name: s!("Madagascar"),                                           currencies: hash_set![ CurrencyCode::MGA ],                                       languages: hash_set![ LanguageCode::FR, LanguageCode::MG ] },
		CountryCode::MH: Country { code: CountryCode::MH, name: s!("Marshall Islands"),                                     currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::MH ] },
		CountryCode::MK: Country { code: CountryCode::MK, name: s!("North Macedonia"),                                      currencies: hash_set![ CurrencyCode::MKD ],                                       languages: hash_set![ LanguageCode::MK, LanguageCode::SQ ] },
		CountryCode::ML: Country { code: CountryCode::ML, name: s!("Mali"),                                                 currencies: hash_set![ CurrencyCode::XOF ],                                       languages: hash_set![ LanguageCode::BM, LanguageCode::FF ] },
		CountryCode::MM: Country { code: CountryCode::MM, name: s!("Myanmar"),                                              currencies: hash_set![ CurrencyCode::MMK ],                                       languages: hash_set![ LanguageCode::MY ] },
		CountryCode::MN: Country { code: CountryCode::MN, name: s!("Mongolia"),                                             currencies: hash_set![ CurrencyCode::MNT ],                                       languages: hash_set![ LanguageCode::MN ] },
		CountryCode::MO: Country { code: CountryCode::MO, name: s!("Macao"),                                                currencies: hash_set![ CurrencyCode::MOP ],                                       languages: hash_set![ LanguageCode::PT, LanguageCode::ZH ] },
		CountryCode::MP: Country { code: CountryCode::MP, name: s!("Northern Mariana Islands"),                             currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::CH, LanguageCode::EN ] },
		CountryCode::MQ: Country { code: CountryCode::MQ, name: s!("Martinique"),                                           currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::MR: Country { code: CountryCode::MR, name: s!("Mauritania"),                                           currencies: hash_set![ CurrencyCode::MRU ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::MS: Country { code: CountryCode::MS, name: s!("Montserrat"),                                           currencies: hash_set![ CurrencyCode::XCD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::MT: Country { code: CountryCode::MT, name: s!("Malta"),                                                currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::MT ] },
		CountryCode::MU: Country { code: CountryCode::MU, name: s!("Mauritius"),                                            currencies: hash_set![ CurrencyCode::MUR ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::MV: Country { code: CountryCode::MV, name: s!("Maldives"),                                             currencies: hash_set![ CurrencyCode::MVR ],                                       languages: hash_set![ LanguageCode::DV ] },
		CountryCode::MW: Country { code: CountryCode::MW, name: s!("Malawi"),                                               currencies: hash_set![ CurrencyCode::MWK ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::NY ] },
		CountryCode::MX: Country { code: CountryCode::MX, name: s!("Mexico"),                                               currencies: hash_set![ CurrencyCode::MXN, CurrencyCode::MXV ],                    languages: hash_set![ LanguageCode::ES ] },
		CountryCode::MY: Country { code: CountryCode::MY, name: s!("Malaysia"),                                             currencies: hash_set![ CurrencyCode::MYR ],                                       languages: hash_set![ LanguageCode::MS ] },
		CountryCode::MZ: Country { code: CountryCode::MZ, name: s!("Mozambique"),                                           currencies: hash_set![ CurrencyCode::MZN ],                                       languages: hash_set![ LanguageCode::PT ] },
		CountryCode::NA: Country { code: CountryCode::NA, name: s!("Namibia"),                                              currencies: hash_set![ CurrencyCode::NAD, CurrencyCode::ZAR ],                    languages: hash_set![ LanguageCode::EN ] },
		CountryCode::NC: Country { code: CountryCode::NC, name: s!("New Caledonia"),                                        currencies: hash_set![ CurrencyCode::XPF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::NE: Country { code: CountryCode::NE, name: s!("Niger"),                                                currencies: hash_set![ CurrencyCode::XOF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::NF: Country { code: CountryCode::NF, name: s!("Norfolk Island"),                                       currencies: hash_set![ CurrencyCode::AUD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::NG: Country { code: CountryCode::NG, name: s!("Nigeria"),                                              currencies: hash_set![ CurrencyCode::NGN ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::NI: Country { code: CountryCode::NI, name: s!("Nicaragua"),                                            currencies: hash_set![ CurrencyCode::NIO ],                                       languages: hash_set![ LanguageCode::ES ] },
		CountryCode::NL: Country { code: CountryCode::NL, name: s!("Netherlands, Kingdom of the"),                          currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::NL ] },
		CountryCode::NO: Country { code: CountryCode::NO, name: s!("Norway"),                                               currencies: hash_set![ CurrencyCode::NOK ],                                       languages: hash_set![ LanguageCode::NO ] },
		CountryCode::NP: Country { code: CountryCode::NP, name: s!("Nepal"),                                                currencies: hash_set![ CurrencyCode::NPR ],                                       languages: hash_set![ LanguageCode::NE ] },
		CountryCode::NR: Country { code: CountryCode::NR, name: s!("Nauru"),                                                currencies: hash_set![ CurrencyCode::AUD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::NA ] },
		CountryCode::NU: Country { code: CountryCode::NU, name: s!("Niue"),                                                 currencies: hash_set![ CurrencyCode::NZD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::NZ: Country { code: CountryCode::NZ, name: s!("New Zealand"),                                          currencies: hash_set![ CurrencyCode::NZD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::MI ] },
		CountryCode::OM: Country { code: CountryCode::OM, name: s!("Oman"),                                                 currencies: hash_set![ CurrencyCode::OMR ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::PA: Country { code: CountryCode::PA, name: s!("Panama"),                                               currencies: hash_set![ CurrencyCode::PAB, CurrencyCode::USD ],                    languages: hash_set![ LanguageCode::ES ] },
		CountryCode::PE: Country { code: CountryCode::PE, name: s!("Peru"),                                                 currencies: hash_set![ CurrencyCode::PEN ],                                       languages: hash_set![ LanguageCode::AY, LanguageCode::ES, LanguageCode::QU ] },
		CountryCode::PF: Country { code: CountryCode::PF, name: s!("French Polynesia"),                                     currencies: hash_set![ CurrencyCode::XPF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::PG: Country { code: CountryCode::PG, name: s!("Papua New Guinea"),                                     currencies: hash_set![ CurrencyCode::PGK ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::HO ] },
		CountryCode::PH: Country { code: CountryCode::PH, name: s!("Philippines"),                                          currencies: hash_set![ CurrencyCode::PHP ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::TL ] },
		CountryCode::PK: Country { code: CountryCode::PK, name: s!("Pakistan"),                                             currencies: hash_set![ CurrencyCode::PKR ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::UR ] },
		CountryCode::PL: Country { code: CountryCode::PL, name: s!("Poland"),                                               currencies: hash_set![ CurrencyCode::PLN ],                                       languages: hash_set![ LanguageCode::PL ] },
		CountryCode::PM: Country { code: CountryCode::PM, name: s!("Saint Pierre and Miquelon"),                            currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::PN: Country { code: CountryCode::PN, name: s!("Pitcairn"),                                             currencies: hash_set![ CurrencyCode::NZD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::PR: Country { code: CountryCode::PR, name: s!("Puerto Rico"),                                          currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::ES ] },
		CountryCode::PS: Country { code: CountryCode::PS, name: s!("Palestine, State of"),                                  currencies: hash_set![],                                                          languages: hash_set![ LanguageCode::AR ] },
		CountryCode::PT: Country { code: CountryCode::PT, name: s!("Portugal"),                                             currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::PT ] },
		CountryCode::PW: Country { code: CountryCode::PW, name: s!("Palau"),                                                currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::PY: Country { code: CountryCode::PY, name: s!("Paraguay"),                                             currencies: hash_set![ CurrencyCode::PYG ],                                       languages: hash_set![ LanguageCode::ES, LanguageCode::GN ] },
		CountryCode::QA: Country { code: CountryCode::QA, name: s!("Qatar"),                                                currencies: hash_set![ CurrencyCode::QAR ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::RE: Country { code: CountryCode::RE, name: s!("Réunion"),                                              currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::RO: Country { code: CountryCode::RO, name: s!("Romania"),                                              currencies: hash_set![ CurrencyCode::RON ],                                       languages: hash_set![ LanguageCode::RO ] },
		CountryCode::RS: Country { code: CountryCode::RS, name: s!("Serbia"),                                               currencies: hash_set![ CurrencyCode::RSD ],                                       languages: hash_set![ LanguageCode::SR ] },
		CountryCode::RU: Country { code: CountryCode::RU, name: s!("Russian Federation"),                                   currencies: hash_set![ CurrencyCode::RUB ],                                       languages: hash_set![ LanguageCode::RU ] },
		CountryCode::RW: Country { code: CountryCode::RW, name: s!("Rwanda"),                                               currencies: hash_set![ CurrencyCode::RWF ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::FR, LanguageCode::RW, LanguageCode::SW ] },
		CountryCode::SA: Country { code: CountryCode::SA, name: s!("Saudi Arabia"),                                         currencies: hash_set![ CurrencyCode::SAR ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::SB: Country { code: CountryCode::SB, name: s!("Solomon Islands"),                                      currencies: hash_set![ CurrencyCode::SBD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::SC: Country { code: CountryCode::SC, name: s!("Seychelles"),                                           currencies: hash_set![ CurrencyCode::SCR ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::FR ] },
		CountryCode::SD: Country { code: CountryCode::SD, name: s!("Sudan"),                                                currencies: hash_set![ CurrencyCode::SDG ],                                       languages: hash_set![ LanguageCode::AR, LanguageCode::EN ] },
		CountryCode::SE: Country { code: CountryCode::SE, name: s!("Sweden"),                                               currencies: hash_set![ CurrencyCode::SEK ],                                       languages: hash_set![ LanguageCode::SV ] },
		CountryCode::SG: Country { code: CountryCode::SG, name: s!("Singapore"),                                            currencies: hash_set![ CurrencyCode::SGD ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::MS, LanguageCode::TA, LanguageCode::ZH ] },
		CountryCode::SH: Country { code: CountryCode::SH, name: s!("Saint Helena, Ascension and Tristan da Cunha"),         currencies: hash_set![ CurrencyCode::GBP, CurrencyCode::SHP ],                    languages: hash_set![ LanguageCode::EN ] },
		CountryCode::SI: Country { code: CountryCode::SI, name: s!("Slovenia"),                                             currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::SL ] },
		CountryCode::SJ: Country { code: CountryCode::SJ, name: s!("Svalbard and Jan Mayen"),                               currencies: hash_set![ CurrencyCode::NOK ],                                       languages: hash_set![ LanguageCode::NO ] },
		CountryCode::SK: Country { code: CountryCode::SK, name: s!("Slovakia"),                                             currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::SK ] },
		CountryCode::SL: Country { code: CountryCode::SL, name: s!("Sierra Leone"),                                         currencies: hash_set![ CurrencyCode::SLE, CurrencyCode::SLL ],                    languages: hash_set![ LanguageCode::EN ] },
		CountryCode::SM: Country { code: CountryCode::SM, name: s!("San Marino"),                                           currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::IT ] },
		CountryCode::SN: Country { code: CountryCode::SN, name: s!("Senegal"),                                              currencies: hash_set![ CurrencyCode::XOF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::SO: Country { code: CountryCode::SO, name: s!("Somalia"),                                              currencies: hash_set![ CurrencyCode::SOS ],                                       languages: hash_set![ LanguageCode::AR, LanguageCode::SO ] },
		CountryCode::SR: Country { code: CountryCode::SR, name: s!("Suriname"),                                             currencies: hash_set![ CurrencyCode::SRD ],                                       languages: hash_set![ LanguageCode::NL ] },
		CountryCode::SS: Country { code: CountryCode::SS, name: s!("South Sudan"),                                          currencies: hash_set![ CurrencyCode::SSP ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::ST: Country { code: CountryCode::ST, name: s!("Sao Tome and Principe"),                                currencies: hash_set![ CurrencyCode::STN ],                                       languages: hash_set![ LanguageCode::PT ] },
		CountryCode::SV: Country { code: CountryCode::SV, name: s!("El Salvador"),                                          currencies: hash_set![ CurrencyCode::SVC, CurrencyCode::USD ],                    languages: hash_set![ LanguageCode::ES ] },
		CountryCode::SX: Country { code: CountryCode::SX, name: s!("Sint Maarten (Dutch part)"),                            currencies: hash_set![ CurrencyCode::ANG ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::NL ] },
		CountryCode::SY: Country { code: CountryCode::SY, name: s!("Syrian Arab Republic"),                                 currencies: hash_set![ CurrencyCode::SYP ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::SZ: Country { code: CountryCode::SZ, name: s!("Eswatini"),                                             currencies: hash_set![ CurrencyCode::SZL, CurrencyCode::ZAR ],                    languages: hash_set![ LanguageCode::EN, LanguageCode::SS ] },
		CountryCode::TC: Country { code: CountryCode::TC, name: s!("Turks and Caicos Islands"),                             currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::TD: Country { code: CountryCode::TD, name: s!("Chad"),                                                 currencies: hash_set![ CurrencyCode::XAF ],                                       languages: hash_set![ LanguageCode::AR, LanguageCode::FR ] },
		CountryCode::TF: Country { code: CountryCode::TF, name: s!("French Southern Territories"),                          currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::TG: Country { code: CountryCode::TG, name: s!("Togo"),                                                 currencies: hash_set![ CurrencyCode::XOF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::TH: Country { code: CountryCode::TH, name: s!("Thailand"),                                             currencies: hash_set![ CurrencyCode::THB ],                                       languages: hash_set![ LanguageCode::TH ] },
		CountryCode::TJ: Country { code: CountryCode::TJ, name: s!("Tajikistan"),                                           currencies: hash_set![ CurrencyCode::TJS ],                                       languages: hash_set![ LanguageCode::TG ] },
		CountryCode::TK: Country { code: CountryCode::TK, name: s!("Tokelau"),                                              currencies: hash_set![ CurrencyCode::NZD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::TL: Country { code: CountryCode::TL, name: s!("Timor-Leste"),                                          currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::PT ] },
		CountryCode::TM: Country { code: CountryCode::TM, name: s!("Turkmenistan"),                                         currencies: hash_set![ CurrencyCode::TMT ],                                       languages: hash_set![ LanguageCode::TK ] },
		CountryCode::TN: Country { code: CountryCode::TN, name: s!("Tunisia"),                                              currencies: hash_set![ CurrencyCode::TND ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::TO: Country { code: CountryCode::TO, name: s!("Tonga"),                                                currencies: hash_set![ CurrencyCode::TOP ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::TO ] },
		CountryCode::TR: Country { code: CountryCode::TR, name: s!("Türkiye"),                                              currencies: hash_set![ CurrencyCode::TRY ],                                       languages: hash_set![ LanguageCode::TR ] },
		CountryCode::TT: Country { code: CountryCode::TT, name: s!("Trinidad and Tobago"),                                  currencies: hash_set![ CurrencyCode::TTD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::TV: Country { code: CountryCode::TV, name: s!("Tuvalu"),                                               currencies: hash_set![ CurrencyCode::AUD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::TW: Country { code: CountryCode::TW, name: s!("Taiwan, Province of China"),                            currencies: hash_set![ CurrencyCode::TWD ],                                       languages: hash_set![ LanguageCode::ZH ] },
		CountryCode::TZ: Country { code: CountryCode::TZ, name: s!("Tanzania, United Republic of"),                         currencies: hash_set![ CurrencyCode::TZS ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::SW ] },
		CountryCode::UA: Country { code: CountryCode::UA, name: s!("Ukraine"),                                              currencies: hash_set![ CurrencyCode::UAH ],                                       languages: hash_set![ LanguageCode::UK ] },
		CountryCode::UG: Country { code: CountryCode::UG, name: s!("Uganda"),                                               currencies: hash_set![ CurrencyCode::UGX ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::SW ] },
		CountryCode::UM: Country { code: CountryCode::UM, name: s!("United States Minor Outlying Islands"),                 currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::US: Country { code: CountryCode::US, name: s!("United States of America"),                             currencies: hash_set![ CurrencyCode::USD, CurrencyCode::USN ],                    languages: hash_set![ LanguageCode::EN ] },
		CountryCode::UY: Country { code: CountryCode::UY, name: s!("Uruguay"),                                              currencies: hash_set![ CurrencyCode::UYI, CurrencyCode::UYU, CurrencyCode::UYW ], languages: hash_set![ LanguageCode::ES ] },
		CountryCode::UZ: Country { code: CountryCode::UZ, name: s!("Uzbekistan"),                                           currencies: hash_set![ CurrencyCode::UZS ],                                       languages: hash_set![ LanguageCode::UZ ] },
		CountryCode::VA: Country { code: CountryCode::VA, name: s!("Holy See"),                                             currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::IT, LanguageCode::LA ] },
		CountryCode::VC: Country { code: CountryCode::VC, name: s!("Saint Vincent and the Grenadines"),                     currencies: hash_set![ CurrencyCode::XCD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::VE: Country { code: CountryCode::VE, name: s!("Venezuela (Bolivarian Republic of)"),                   currencies: hash_set![ CurrencyCode::VED, CurrencyCode::VES ],                    languages: hash_set![ LanguageCode::ES ] },
		CountryCode::VG: Country { code: CountryCode::VG, name: s!("Virgin Islands (British)"),                             currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::VI: Country { code: CountryCode::VI, name: s!("Virgin Islands (U.S.)"),                                currencies: hash_set![ CurrencyCode::USD ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::VN: Country { code: CountryCode::VN, name: s!("Viet Nam"),                                             currencies: hash_set![ CurrencyCode::VND ],                                       languages: hash_set![ LanguageCode::VI ] },
		CountryCode::VU: Country { code: CountryCode::VU, name: s!("Vanuatu"),                                              currencies: hash_set![ CurrencyCode::VUV ],                                       languages: hash_set![ LanguageCode::BI, LanguageCode::EN, LanguageCode::FR ] },
		CountryCode::WF: Country { code: CountryCode::WF, name: s!("Wallis and Futuna"),                                    currencies: hash_set![ CurrencyCode::XPF ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::WS: Country { code: CountryCode::WS, name: s!("Samoa"),                                                currencies: hash_set![ CurrencyCode::WST ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::SM ] },
		CountryCode::YE: Country { code: CountryCode::YE, name: s!("Yemen"),                                                currencies: hash_set![ CurrencyCode::YER ],                                       languages: hash_set![ LanguageCode::AR ] },
		CountryCode::YT: Country { code: CountryCode::YT, name: s!("Mayotte"),                                              currencies: hash_set![ CurrencyCode::EUR ],                                       languages: hash_set![ LanguageCode::FR ] },
		CountryCode::ZA: Country { code: CountryCode::ZA, name: s!("South Africa"),                                         currencies: hash_set![ CurrencyCode::ZAR ],                                       languages: hash_set![ LanguageCode::AF, LanguageCode::EN, LanguageCode::NR, LanguageCode::SS, LanguageCode::ST, LanguageCode::TN, LanguageCode::TS, LanguageCode::VE, LanguageCode::XH, LanguageCode::ZU ] },
		CountryCode::ZM: Country { code: CountryCode::ZM, name: s!("Zambia"),                                               currencies: hash_set![ CurrencyCode::ZMW ],                                       languages: hash_set![ LanguageCode::EN ] },
		CountryCode::ZW: Country { code: CountryCode::ZW, name: s!("Zimbabwe"),                                             currencies: hash_set![ CurrencyCode::ZWL ],                                       languages: hash_set![ LanguageCode::EN, LanguageCode::NR, LanguageCode::NY, LanguageCode::SN, LanguageCode::ST, LanguageCode::TN, LanguageCode::VE, LanguageCode::XH ] },
	}
});



//		Enums

//		CountryCode																
/// The possible countries' codes.
/// 
/// These codes are based on the ISO 3166 standard, specifically ISO 3166-1,
/// which defines codes of two and three characters to represent countries and
/// territories. There are both alphabetic and numeric codes, using either two
/// letters, three letters, or three numbers.
/// 
/// # Alphabetic codes
/// 
/// The alphabetic codes are defined by the ISO 3166-1 alpha-2 set, which is the
/// most widely-used of the three sets; and the ISO 3166-1 alpha-3 set, which is
/// less widely-used but gives better visual association between the codes and
/// country names than the alpha-2 set.
/// 
/// # Numeric codes
/// 
/// The three-digit numeric code is a useful alternative when the letter-based
/// code may not be appropriate.
/// 
/// # Data sources
/// 
/// The list of codes is available from [the ISO site](https://www.iso.org/iso-3166-country-codes.html),
/// and from [Wikipedia](https://en.wikipedia.org/wiki/ISO_3166-1).
/// 
/// # See also
/// 
/// * [`Country`]
/// 
#[cfg_attr(    feature = "reasons",  allow(clippy::upper_case_acronyms, reason = "Uppercase is suitable here"))]
#[cfg_attr(not(feature = "reasons"), allow(clippy::upper_case_acronyms))]
#[cfg_attr(    feature = "reasons",  allow(clippy::zero_prefixed_literal, reason = "Zeroes aid readability here"))]
#[cfg_attr(not(feature = "reasons"), allow(clippy::zero_prefixed_literal))]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ToSchema)]
#[repr(u16)]
#[serde(into = "String", try_from = "String")]
#[non_exhaustive]
pub enum CountryCode {
	//		Two-letter codes (ISO 3166-1 alpha-2)								
	/// Andorra
	AD  = 020,
	
	/// United Arab Emirates
	AE  = 784,
	
	/// Afghanistan
	AF  = 004,
	
	/// Antigua and Barbuda
	AG  = 028,
	
	/// Anguilla
	AI  = 660,
	
	/// Albania
	AL  = 008,
	
	/// Armenia
	AM  = 051,
	
	/// Angola
	AO  = 024,
	
	/// Antarctica
	AQ  = 010,
	
	/// Argentina
	AR  = 032,
	
	/// American Samoa
	AS  = 016,
	
	/// Austria
	AT  = 040,
	
	/// Australia
	AU  = 036,
	
	/// Aruba
	AW  = 533,
	
	/// Åland Islands
	AX  = 248,
	
	/// Azerbaijan
	AZ  = 031,
	
	/// Bosnia and Herzegovina
	BA  = 070,
	
	/// Barbados
	BB  = 052,
	
	/// Bangladesh
	BD  = 050,
	
	/// Belgium
	BE  = 056,
	
	/// Burkina Faso
	BF  = 854,
	
	/// Bulgaria
	BG  = 100,
	
	/// Bahrain
	BH  = 048,
	
	/// Burundi
	BI  = 108,
	
	/// Benin
	BJ  = 204,
	
	/// Saint Barthélemy
	BL  = 652,
	
	/// Bermuda
	BM  = 060,
	
	/// Brunei Darussalam
	BN  = 096,
	
	/// Bolivia (Plurinational State of)
	BO  = 068,
	
	/// Bonaire, Sint Eustatius and Saba
	BQ  = 535,
	
	/// Brazil
	BR  = 076,
	
	/// Bahamas
	BS  = 044,
	
	/// Bhutan
	BT  = 064,
	
	/// Bouvet Island
	BV  = 074,
	
	/// Botswana
	BW  = 072,
	
	/// Belarus
	BY  = 112,
	
	/// Belize
	BZ  = 084,
	
	/// Canada
	CA  = 124,
	
	/// Cocos (Keeling) Islands
	CC  = 166,
	
	/// Congo, Democratic Republic of the
	CD  = 180,
	
	/// Central African Republic
	CF  = 140,
	
	/// Congo
	CG  = 178,
	
	/// Switzerland
	CH  = 756,
	
	/// Côte d'Ivoire
	CI  = 384,
	
	/// Cook Islands
	CK  = 184,
	
	/// Chile
	CL  = 152,
	
	/// Cameroon
	CM  = 120,
	
	/// China
	CN  = 156,
	
	/// Colombia
	CO  = 170,
	
	/// Costa Rica
	CR  = 188,
	
	/// Cuba
	CU  = 192,
	
	/// Cabo Verde
	CV  = 132,
	
	/// Curaçao
	CW  = 531,
	
	/// Christmas Island
	CX  = 162,
	
	/// Cyprus
	CY  = 196,
	
	/// Czechia
	CZ  = 203,
	
	/// Germany
	DE  = 276,
	
	/// Djibouti
	DJ  = 262,
	
	/// Denmark
	DK  = 208,
	
	/// Dominica
	DM  = 212,
	
	/// Dominican Republic
	DO  = 214,
	
	/// Algeria
	DZ  = 012,
	
	/// Ecuador
	EC  = 218,
	
	/// Estonia
	EE  = 233,
	
	/// Egypt
	EG  = 818,
	
	/// Western Sahara
	EH  = 732,
	
	/// Eritrea
	ER  = 232,
	
	/// Spain
	ES  = 724,
	
	/// Ethiopia
	ET  = 231,
	
	/// Finland
	FI  = 246,
	
	/// Fiji
	FJ  = 242,
	
	/// Falkland Islands (Malvinas)
	FK  = 238,
	
	/// Micronesia (Federated States of)
	FM  = 583,
	
	/// Faroe Islands
	FO  = 234,
	
	/// France
	FR  = 250,
	
	/// Gabon
	GA  = 266,
	
	/// United Kingdom of Great Britain and Northern Ireland
	GB  = 826,
	
	/// Grenada
	GD  = 308,
	
	/// Georgia
	GE  = 268,
	
	/// French Guiana
	GF  = 254,
	
	/// Guernsey
	GG  = 831,
	
	/// Ghana
	GH  = 288,
	
	/// Gibraltar
	GI  = 292,
	
	/// Greenland
	GL  = 304,
	
	/// Gambia
	GM  = 270,
	
	/// Guinea
	GN  = 324,
	
	/// Guadeloupe
	GP  = 312,
	
	/// Equatorial Guinea
	GQ  = 226,
	
	/// Greece
	GR  = 300,
	
	/// South Georgia and the South Sandwich Islands
	GS  = 239,
	
	/// Guatemala
	GT  = 320,
	
	/// Guam
	GU  = 316,
	
	/// Guinea-Bissau
	GW  = 624,
	
	/// Guyana
	GY  = 328,
	
	/// Hong Kong
	HK  = 344,
	
	/// Heard Island and McDonald Islands
	HM  = 334,
	
	/// Honduras
	HN  = 340,
	
	/// Croatia
	HR  = 191,
	
	/// Haiti
	HT  = 332,
	
	/// Hungary
	HU  = 348,
	
	/// Indonesia
	ID  = 360,
	
	/// Ireland
	IE  = 372,
	
	/// Israel
	IL  = 376,
	
	/// Isle of Man
	IM  = 833,
	
	/// India
	IN  = 356,
	
	/// British Indian Ocean Territory
	IO  = 086,
	
	/// Iraq
	IQ  = 368,
	
	/// Iran (Islamic Republic of)
	IR  = 364,
	
	/// Iceland
	IS  = 352,
	
	/// Italy
	IT  = 380,
	
	/// Jersey
	JE  = 832,
	
	/// Jamaica
	JM  = 388,
	
	/// Jordan
	JO  = 400,
	
	/// Japan
	JP  = 392,
	
	/// Kenya
	KE  = 404,
	
	/// Kyrgyzstan
	KG  = 417,
	
	/// Cambodia
	KH  = 116,
	
	/// Kiribati
	KI  = 296,
	
	/// Comoros
	KM  = 174,
	
	/// Saint Kitts and Nevis
	KN  = 659,
	
	/// Korea (Democratic People's Republic of)
	KP  = 408,
	
	/// Korea, Republic of
	KR  = 410,
	
	/// Kuwait
	KW  = 414,
	
	/// Cayman Islands
	KY  = 136,
	
	/// Kazakhstan
	KZ  = 398,
	
	/// Lao People's Democratic Republic
	LA  = 418,
	
	/// Lebanon
	LB  = 422,
	
	/// Saint Lucia
	LC  = 662,
	
	/// Liechtenstein
	LI  = 438,
	
	/// Sri Lanka
	LK  = 144,
	
	/// Liberia
	LR  = 430,
	
	/// Lesotho
	LS  = 426,
	
	/// Lithuania
	LT  = 440,
	
	/// Luxembourg
	LU  = 442,
	
	/// Latvia
	LV  = 428,
	
	/// Libya
	LY  = 434,
	
	/// Morocco
	MA  = 504,
	
	/// Monaco
	MC  = 492,
	
	/// Moldova, Republic of
	MD  = 498,
	
	/// Montenegro
	ME  = 499,
	
	/// Saint Martin (French part)
	MF  = 663,
	
	/// Madagascar
	MG  = 450,
	
	/// Marshall Islands
	MH  = 584,
	
	/// North Macedonia
	MK  = 807,
	
	/// Mali
	ML  = 466,
	
	/// Myanmar
	MM  = 104,
	
	/// Mongolia
	MN  = 496,
	
	/// Macao
	MO  = 446,
	
	/// Northern Mariana Islands
	MP  = 580,
	
	/// Martinique
	MQ  = 474,
	
	/// Mauritania
	MR  = 478,
	
	/// Montserrat
	MS  = 500,
	
	/// Malta
	MT  = 470,
	
	/// Mauritius
	MU  = 480,
	
	/// Maldives
	MV  = 462,
	
	/// Malawi
	MW  = 454,
	
	/// Mexico
	MX  = 484,
	
	/// Malaysia
	MY  = 458,
	
	/// Mozambique
	MZ  = 508,
	
	/// Namibia
	NA  = 516,
	
	/// New Caledonia
	NC  = 540,
	
	/// Niger
	NE  = 562,
	
	/// Norfolk Island
	NF  = 574,
	
	/// Nigeria
	NG  = 566,
	
	/// Nicaragua
	NI  = 558,
	
	/// Netherlands, Kingdom of the
	NL  = 528,
	
	/// Norway
	NO  = 578,
	
	/// Nepal
	NP  = 524,
	
	/// Nauru
	NR  = 520,
	
	/// Niue
	NU  = 570,
	
	/// New Zealand
	NZ  = 554,
	
	/// Oman
	OM  = 512,
	
	/// Panama
	PA  = 591,
	
	/// Peru
	PE  = 604,
	
	/// French Polynesia
	PF  = 258,
	
	/// Papua New Guinea
	PG  = 598,
	
	/// Philippines
	PH  = 608,
	
	/// Pakistan
	PK  = 586,
	
	/// Poland
	PL  = 616,
	
	/// Saint Pierre and Miquelon
	PM  = 666,
	
	/// Pitcairn
	PN  = 612,
	
	/// Puerto Rico
	PR  = 630,
	
	/// Palestine, State of
	PS  = 275,
	
	/// Portugal
	PT  = 620,
	
	/// Palau
	PW  = 585,
	
	/// Paraguay
	PY  = 600,
	
	/// Qatar
	QA  = 634,
	
	/// Réunion
	RE  = 638,
	
	/// Romania
	RO  = 642,
	
	/// Serbia
	RS  = 688,
	
	/// Russian Federation
	RU  = 643,
	
	/// Rwanda
	RW  = 646,
	
	/// Saudi Arabia
	SA  = 682,
	
	/// Solomon Islands
	SB  = 090,
	
	/// Seychelles
	SC  = 690,
	
	/// Sudan
	SD  = 729,
	
	/// Sweden
	SE  = 752,
	
	/// Singapore
	SG  = 702,
	
	/// Saint Helena, Ascension and Tristan da Cunha
	SH  = 654,
	
	/// Slovenia
	SI  = 705,
	
	/// Svalbard and Jan Mayen
	SJ  = 744,
	
	/// Slovakia
	SK  = 703,
	
	/// Sierra Leone
	SL  = 694,
	
	/// San Marino
	SM  = 674,
	
	/// Senegal
	SN  = 686,
	
	/// Somalia
	SO  = 706,
	
	/// Suriname
	SR  = 740,
	
	/// South Sudan
	SS  = 728,
	
	/// Sao Tome and Principe
	ST  = 678,
	
	/// El Salvador
	SV  = 222,
	
	/// Sint Maarten (Dutch part)
	SX  = 534,
	
	/// Syrian Arab Republic
	SY  = 760,
	
	/// Eswatini
	SZ  = 748,
	
	/// Turks and Caicos Islands
	TC  = 796,
	
	/// Chad
	TD  = 148,
	
	/// French Southern Territories
	TF  = 260,
	
	/// Togo
	TG  = 768,
	
	/// Thailand
	TH  = 764,
	
	/// Tajikistan
	TJ  = 762,
	
	/// Tokelau
	TK  = 772,
	
	/// Timor-Leste
	TL  = 626,
	
	/// Turkmenistan
	TM  = 795,
	
	/// Tunisia
	TN  = 788,
	
	/// Tonga
	TO  = 776,
	
	/// Türkiye
	TR  = 792,
	
	/// Trinidad and Tobago
	TT  = 780,
	
	/// Tuvalu
	TV  = 798,
	
	/// Taiwan, Province of China
	TW  = 158,
	
	/// Tanzania, United Republic of
	TZ  = 834,
	
	/// Ukraine
	UA  = 804,
	
	/// Uganda
	UG  = 800,
	
	/// United States Minor Outlying Islands
	UM  = 581,
	
	/// United States of America
	US  = 840,
	
	/// Uruguay
	UY  = 858,
	
	/// Uzbekistan
	UZ  = 860,
	
	/// Holy See
	VA  = 336,
	
	/// Saint Vincent and the Grenadines
	VC  = 670,
	
	/// Venezuela (Bolivarian Republic of)
	VE  = 862,
	
	/// Virgin Islands (British)
	VG  = 092,
	
	/// Virgin Islands (U.S.)
	VI  = 850,
	
	/// Viet Nam
	VN  = 704,
	
	/// Vanuatu
	VU  = 548,
	
	/// Wallis and Futuna
	WF  = 876,
	
	/// Samoa
	WS  = 882,
	
	/// Yemen
	YE  = 887,
	
	/// Mayotte
	YT  = 175,
	
	/// South Africa
	ZA  = 710,
	
	/// Zambia
	ZM  = 894,
	
	/// Zimbabwe
	ZW  = 716,
	
	//		Three-letter codes (ISO 3166-1 alpha-3)								
	//	For maximum ease of use, both two-letter and three-letter codes are
	//	stored in the same enum. However, this causes a collision between the
	//	numeric representations. To avoid this, the three-letter codes have
	//	1,000 added to them, for the sole purpose of internal storage. This
	//	gets adjusted when the enum variants are serialized or otherwise
	//	represented as an integer.
	
	/// Aruba
	ABW = 1_533,
	
	/// Afghanistan
	AFG = 1_004,
	
	/// Angola
	AGO = 1_024,
	
	/// Anguilla
	AIA = 1_660,
	
	/// Åland Islands
	ALA = 1_248,
	
	/// Albania
	ALB = 1_008,
	
	/// Andorra
	AND = 1_020,
	
	/// United Arab Emirates
	ARE = 1_784,
	
	/// Argentina
	ARG = 1_032,
	
	/// Armenia
	ARM = 1_051,
	
	/// American Samoa
	ASM = 1_016,
	
	/// Antarctica
	ATA = 1_010,
	
	/// French Southern Territories
	ATF = 1_260,
	
	/// Antigua and Barbuda
	ATG = 1_028,
	
	/// Australia
	AUS = 1_036,
	
	/// Austria
	AUT = 1_040,
	
	/// Azerbaijan
	AZE = 1_031,
	
	/// Burundi
	BDI = 1_108,
	
	/// Belgium
	BEL = 1_056,
	
	/// Benin
	BEN = 1_204,
	
	/// Bonaire, Sint Eustatius and Saba
	BES = 1_535,
	
	/// Burkina Faso
	BFA = 1_854,
	
	/// Bangladesh
	BGD = 1_050,
	
	/// Bulgaria
	BGR = 1_100,
	
	/// Bahrain
	BHR = 1_048,
	
	/// Bahamas
	BHS = 1_044,
	
	/// Bosnia and Herzegovina
	BIH = 1_070,
	
	/// Saint Barthélemy
	BLM = 1_652,
	
	/// Belarus
	BLR = 1_112,
	
	/// Belize
	BLZ = 1_084,
	
	/// Bermuda
	BMU = 1_060,
	
	/// Bolivia (Plurinational State of)
	BOL = 1_068,
	
	/// Brazil
	BRA = 1_076,
	
	/// Barbados
	BRB = 1_052,
	
	/// Brunei Darussalam
	BRN = 1_096,
	
	/// Bhutan
	BTN = 1_064,
	
	/// Bouvet Island
	BVT = 1_074,
	
	/// Botswana
	BWA = 1_072,
	
	/// Central African Republic
	CAF = 1_140,
	
	/// Canada
	CAN = 1_124,
	
	/// Cocos (Keeling) Islands
	CCK = 1_166,
	
	/// Switzerland
	CHE = 1_756,
	
	/// Chile
	CHL = 1_152,
	
	/// China
	CHN = 1_156,
	
	/// Côte d'Ivoire
	CIV = 1_384,
	
	/// Cameroon
	CMR = 1_120,
	
	/// Congo, Democratic Republic of the
	COD = 1_180,
	
	/// Congo
	COG = 1_178,
	
	/// Cook Islands
	COK = 1_184,
	
	/// Colombia
	COL = 1_170,
	
	/// Comoros
	COM = 1_174,
	
	/// Cabo Verde
	CPV = 1_132,
	
	/// Costa Rica
	CRI = 1_188,
	
	/// Cuba
	CUB = 1_192,
	
	/// Curaçao
	CUW = 1_531,
	
	/// Christmas Island
	CXR = 1_162,
	
	/// Cayman Islands
	CYM = 1_136,
	
	/// Cyprus
	CYP = 1_196,
	
	/// Czechia
	CZE = 1_203,
	
	/// Germany
	DEU = 1_276,
	
	/// Djibouti
	DJI = 1_262,
	
	/// Dominica
	DMA = 1_212,
	
	/// Denmark
	DNK = 1_208,
	
	/// Dominican Republic
	DOM = 1_214,
	
	/// Algeria
	DZA = 1_012,
	
	/// Ecuador
	ECU = 1_218,
	
	/// Egypt
	EGY = 1_818,
	
	/// Eritrea
	ERI = 1_232,
	
	/// Western Sahara
	ESH = 1_732,
	
	/// Spain
	ESP = 1_724,
	
	/// Estonia
	EST = 1_233,
	
	/// Ethiopia
	ETH = 1_231,
	
	/// Finland
	FIN = 1_246,
	
	/// Fiji
	FJI = 1_242,
	
	/// Falkland Islands (Malvinas)
	FLK = 1_238,
	
	/// France
	FRA = 1_250,
	
	/// Faroe Islands
	FRO = 1_234,
	
	/// Micronesia (Federated States of)
	FSM = 1_583,
	
	/// Gabon
	GAB = 1_266,
	
	/// United Kingdom of Great Britain and Northern Ireland
	GBR = 1_826,
	
	/// Georgia
	GEO = 1_268,
	
	/// Guernsey
	GGY = 1_831,
	
	/// Ghana
	GHA = 1_288,
	
	/// Gibraltar
	GIB = 1_292,
	
	/// Guinea
	GIN = 1_324,
	
	/// Guadeloupe
	GLP = 1_312,
	
	/// Gambia
	GMB = 1_270,
	
	/// Guinea-Bissau
	GNB = 1_624,
	
	/// Equatorial Guinea
	GNQ = 1_226,
	
	/// Greece
	GRC = 1_300,
	
	/// Grenada
	GRD = 1_308,
	
	/// Greenland
	GRL = 1_304,
	
	/// Guatemala
	GTM = 1_320,
	
	/// French Guiana
	GUF = 1_254,
	
	/// Guam
	GUM = 1_316,
	
	/// Guyana
	GUY = 1_328,
	
	/// Hong Kong
	HKG = 1_344,
	
	/// Heard Island and McDonald Islands
	HMD = 1_334,
	
	/// Honduras
	HND = 1_340,
	
	/// Croatia
	HRV = 1_191,
	
	/// Haiti
	HTI = 1_332,
	
	/// Hungary
	HUN = 1_348,
	
	/// Indonesia
	IDN = 1_360,
	
	/// Isle of Man
	IMN = 1_833,
	
	/// India
	IND = 1_356,
	
	/// British Indian Ocean Territory
	IOT = 1_086,
	
	/// Ireland
	IRL = 1_372,
	
	/// Iran (Islamic Republic of)
	IRN = 1_364,
	
	/// Iraq
	IRQ = 1_368,
	
	/// Iceland
	ISL = 1_352,
	
	/// Israel
	ISR = 1_376,
	
	/// Italy
	ITA = 1_380,
	
	/// Jamaica
	JAM = 1_388,
	
	/// Jersey
	JEY = 1_832,
	
	/// Jordan
	JOR = 1_400,
	
	/// Japan
	JPN = 1_392,
	
	/// Kazakhstan
	KAZ = 1_398,
	
	/// Kenya
	KEN = 1_404,
	
	/// Kyrgyzstan
	KGZ = 1_417,
	
	/// Cambodia
	KHM = 1_116,
	
	/// Kiribati
	KIR = 1_296,
	
	/// Saint Kitts and Nevis
	KNA = 1_659,
	
	/// Korea, Republic of
	KOR = 1_410,
	
	/// Kuwait
	KWT = 1_414,
	
	/// Lao People's Democratic Republic
	LAO = 1_418,
	
	/// Lebanon
	LBN = 1_422,
	
	/// Liberia
	LBR = 1_430,
	
	/// Libya
	LBY = 1_434,
	
	/// Saint Lucia
	LCA = 1_662,
	
	/// Liechtenstein
	LIE = 1_438,
	
	/// Sri Lanka
	LKA = 1_144,
	
	/// Lesotho
	LSO = 1_426,
	
	/// Lithuania
	LTU = 1_440,
	
	/// Luxembourg
	LUX = 1_442,
	
	/// Latvia
	LVA = 1_428,
	
	/// Macao
	MAC = 1_446,
	
	/// Saint Martin (French part)
	MAF = 1_663,
	
	/// Morocco
	MAR = 1_504,
	
	/// Monaco
	MCO = 1_492,
	
	/// Moldova, Republic of
	MDA = 1_498,
	
	/// Madagascar
	MDG = 1_450,
	
	/// Maldives
	MDV = 1_462,
	
	/// Mexico
	MEX = 1_484,
	
	/// Marshall Islands
	MHL = 1_584,
	
	/// North Macedonia
	MKD = 1_807,
	
	/// Mali
	MLI = 1_466,
	
	/// Malta
	MLT = 1_470,
	
	/// Myanmar
	MMR = 1_104,
	
	/// Montenegro
	MNE = 1_499,
	
	/// Mongolia
	MNG = 1_496,
	
	/// Northern Mariana Islands
	MNP = 1_580,
	
	/// Mozambique
	MOZ = 1_508,
	
	/// Mauritania
	MRT = 1_478,
	
	/// Montserrat
	MSR = 1_500,
	
	/// Martinique
	MTQ = 1_474,
	
	/// Mauritius
	MUS = 1_480,
	
	/// Malawi
	MWI = 1_454,
	
	/// Malaysia
	MYS = 1_458,
	
	/// Mayotte
	MYT = 1_175,
	
	/// Namibia
	NAM = 1_516,
	
	/// New Caledonia
	NCL = 1_540,
	
	/// Niger
	NER = 1_562,
	
	/// Norfolk Island
	NFK = 1_574,
	
	/// Nigeria
	NGA = 1_566,
	
	/// Nicaragua
	NIC = 1_558,
	
	/// Niue
	NIU = 1_570,
	
	/// Netherlands, Kingdom of the
	NLD = 1_528,
	
	/// Norway
	NOR = 1_578,
	
	/// Nepal
	NPL = 1_524,
	
	/// Nauru
	NRU = 1_520,
	
	/// New Zealand
	NZL = 1_554,
	
	/// Oman
	OMN = 1_512,
	
	/// Pakistan
	PAK = 1_586,
	
	/// Panama
	PAN = 1_591,
	
	/// Pitcairn
	PCN = 1_612,
	
	/// Peru
	PER = 1_604,
	
	/// Philippines
	PHL = 1_608,
	
	/// Palau
	PLW = 1_585,
	
	/// Papua New Guinea
	PNG = 1_598,
	
	/// Poland
	POL = 1_616,
	
	/// Puerto Rico
	PRI = 1_630,
	
	/// Korea (Democratic People's Republic of)
	PRK = 1_408,
	
	/// Portugal
	PRT = 1_620,
	
	/// Paraguay
	PRY = 1_600,
	
	/// Palestine, State of
	PSE = 1_275,
	
	/// French Polynesia
	PYF = 1_258,
	
	/// Qatar
	QAT = 1_634,
	
	/// Réunion
	REU = 1_638,
	
	/// Romania
	ROU = 1_642,
	
	/// Russian Federation
	RUS = 1_643,
	
	/// Rwanda
	RWA = 1_646,
	
	/// Saudi Arabia
	SAU = 1_682,
	
	/// Sudan
	SDN = 1_729,
	
	/// Senegal
	SEN = 1_686,
	
	/// Singapore
	SGP = 1_702,
	
	/// South Georgia and the South Sandwich Islands
	SGS = 1_239,
	
	/// Saint Helena, Ascension and Tristan da Cunha
	SHN = 1_654,
	
	/// Svalbard and Jan Mayen
	SJM = 1_744,
	
	/// Solomon Islands
	SLB = 1_090,
	
	/// Sierra Leone
	SLE = 1_694,
	
	/// El Salvador
	SLV = 1_222,
	
	/// San Marino
	SMR = 1_674,
	
	/// Somalia
	SOM = 1_706,
	
	/// Saint Pierre and Miquelon
	SPM = 1_666,
	
	/// Serbia
	SRB = 1_688,
	
	/// South Sudan
	SSD = 1_728,
	
	/// Sao Tome and Principe
	STP = 1_678,
	
	/// Suriname
	SUR = 1_740,
	
	/// Slovakia
	SVK = 1_703,
	
	/// Slovenia
	SVN = 1_705,
	
	/// Sweden
	SWE = 1_752,
	
	/// Eswatini
	SWZ = 1_748,
	
	/// Sint Maarten (Dutch part)
	SXM = 1_534,
	
	/// Seychelles
	SYC = 1_690,
	
	/// Syrian Arab Republic
	SYR = 1_760,
	
	/// Turks and Caicos Islands
	TCA = 1_796,
	
	/// Chad
	TCD = 1_148,
	
	/// Togo
	TGO = 1_768,
	
	/// Thailand
	THA = 1_764,
	
	/// Tajikistan
	TJK = 1_762,
	
	/// Tokelau
	TKL = 1_772,
	
	/// Turkmenistan
	TKM = 1_795,
	
	/// Timor-Leste
	TLS = 1_626,
	
	/// Tonga
	TON = 1_776,
	
	/// Trinidad and Tobago
	TTO = 1_780,
	
	/// Tunisia
	TUN = 1_788,
	
	/// Türkiye
	TUR = 1_792,
	
	/// Tuvalu
	TUV = 1_798,
	
	/// Taiwan, Province of China
	TWN = 1_158,
	
	/// Tanzania, United Republic of
	TZA = 1_834,
	
	/// Uganda
	UGA = 1_800,
	
	/// Ukraine
	UKR = 1_804,
	
	/// United States Minor Outlying Islands
	UMI = 1_581,
	
	/// Uruguay
	URY = 1_858,
	
	/// United States of America
	USA = 1_840,
	
	/// Uzbekistan
	UZB = 1_860,
	
	/// Holy See
	VAT = 1_336,
	
	/// Saint Vincent and the Grenadines
	VCT = 1_670,
	
	/// Venezuela (Bolivarian Republic of)
	VEN = 1_862,
	
	/// Virgin Islands (British)
	VGB = 1_092,
	
	/// Virgin Islands (U.S.)
	VIR = 1_850,
	
	/// Viet Nam
	VNM = 1_704,
	
	/// Vanuatu
	VUT = 1_548,
	
	/// Wallis and Futuna
	WLF = 1_876,
	
	/// Samoa
	WSM = 1_882,
	
	/// Yemen
	YEM = 1_887,
	
	/// South Africa
	ZAF = 1_710,
	
	/// Zambia
	ZMB = 1_894,
	
	/// Zimbabwe
	ZWE = 1_716,
}

impl CountryCode {
	//		as_str																
	/// Returns a string representation of the `CountryCode` variant.
	/// 
	/// This method provides a way to obtain a static string slice corresponding
	/// to a variant of the `CountryCode` enum. The returned string slice is
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
			//		Two-letter codes (ISO 3166-1 alpha-2)						
			Self::AD  => "AD",
			Self::AE  => "AE",
			Self::AF  => "AF",
			Self::AG  => "AG",
			Self::AI  => "AI",
			Self::AL  => "AL",
			Self::AM  => "AM",
			Self::AO  => "AO",
			Self::AQ  => "AQ",
			Self::AR  => "AR",
			Self::AS  => "AS",
			Self::AT  => "AT",
			Self::AU  => "AU",
			Self::AW  => "AW",
			Self::AX  => "AX",
			Self::AZ  => "AZ",
			Self::BA  => "BA",
			Self::BB  => "BB",
			Self::BD  => "BD",
			Self::BE  => "BE",
			Self::BF  => "BF",
			Self::BG  => "BG",
			Self::BH  => "BH",
			Self::BI  => "BI",
			Self::BJ  => "BJ",
			Self::BL  => "BL",
			Self::BM  => "BM",
			Self::BN  => "BN",
			Self::BO  => "BO",
			Self::BQ  => "BQ",
			Self::BR  => "BR",
			Self::BS  => "BS",
			Self::BT  => "BT",
			Self::BV  => "BV",
			Self::BW  => "BW",
			Self::BY  => "BY",
			Self::BZ  => "BZ",
			Self::CA  => "CA",
			Self::CC  => "CC",
			Self::CD  => "CD",
			Self::CF  => "CF",
			Self::CG  => "CG",
			Self::CH  => "CH",
			Self::CI  => "CI",
			Self::CK  => "CK",
			Self::CL  => "CL",
			Self::CM  => "CM",
			Self::CN  => "CN",
			Self::CO  => "CO",
			Self::CR  => "CR",
			Self::CU  => "CU",
			Self::CV  => "CV",
			Self::CW  => "CW",
			Self::CX  => "CX",
			Self::CY  => "CY",
			Self::CZ  => "CZ",
			Self::DE  => "DE",
			Self::DJ  => "DJ",
			Self::DK  => "DK",
			Self::DM  => "DM",
			Self::DO  => "DO",
			Self::DZ  => "DZ",
			Self::EC  => "EC",
			Self::EE  => "EE",
			Self::EG  => "EG",
			Self::EH  => "EH",
			Self::ER  => "ER",
			Self::ES  => "ES",
			Self::ET  => "ET",
			Self::FI  => "FI",
			Self::FJ  => "FJ",
			Self::FK  => "FK",
			Self::FM  => "FM",
			Self::FO  => "FO",
			Self::FR  => "FR",
			Self::GA  => "GA",
			Self::GB  => "GB",
			Self::GD  => "GD",
			Self::GE  => "GE",
			Self::GF  => "GF",
			Self::GG  => "GG",
			Self::GH  => "GH",
			Self::GI  => "GI",
			Self::GL  => "GL",
			Self::GM  => "GM",
			Self::GN  => "GN",
			Self::GP  => "GP",
			Self::GQ  => "GQ",
			Self::GR  => "GR",
			Self::GS  => "GS",
			Self::GT  => "GT",
			Self::GU  => "GU",
			Self::GW  => "GW",
			Self::GY  => "GY",
			Self::HK  => "HK",
			Self::HM  => "HM",
			Self::HN  => "HN",
			Self::HR  => "HR",
			Self::HT  => "HT",
			Self::HU  => "HU",
			Self::ID  => "ID",
			Self::IE  => "IE",
			Self::IL  => "IL",
			Self::IM  => "IM",
			Self::IN  => "IN",
			Self::IO  => "IO",
			Self::IQ  => "IQ",
			Self::IR  => "IR",
			Self::IS  => "IS",
			Self::IT  => "IT",
			Self::JE  => "JE",
			Self::JM  => "JM",
			Self::JO  => "JO",
			Self::JP  => "JP",
			Self::KE  => "KE",
			Self::KG  => "KG",
			Self::KH  => "KH",
			Self::KI  => "KI",
			Self::KM  => "KM",
			Self::KN  => "KN",
			Self::KP  => "KP",
			Self::KR  => "KR",
			Self::KW  => "KW",
			Self::KY  => "KY",
			Self::KZ  => "KZ",
			Self::LA  => "LA",
			Self::LB  => "LB",
			Self::LC  => "LC",
			Self::LI  => "LI",
			Self::LK  => "LK",
			Self::LR  => "LR",
			Self::LS  => "LS",
			Self::LT  => "LT",
			Self::LU  => "LU",
			Self::LV  => "LV",
			Self::LY  => "LY",
			Self::MA  => "MA",
			Self::MC  => "MC",
			Self::MD  => "MD",
			Self::ME  => "ME",
			Self::MF  => "MF",
			Self::MG  => "MG",
			Self::MH  => "MH",
			Self::MK  => "MK",
			Self::ML  => "ML",
			Self::MM  => "MM",
			Self::MN  => "MN",
			Self::MO  => "MO",
			Self::MP  => "MP",
			Self::MQ  => "MQ",
			Self::MR  => "MR",
			Self::MS  => "MS",
			Self::MT  => "MT",
			Self::MU  => "MU",
			Self::MV  => "MV",
			Self::MW  => "MW",
			Self::MX  => "MX",
			Self::MY  => "MY",
			Self::MZ  => "MZ",
			Self::NA  => "NA",
			Self::NC  => "NC",
			Self::NE  => "NE",
			Self::NF  => "NF",
			Self::NG  => "NG",
			Self::NI  => "NI",
			Self::NL  => "NL",
			Self::NO  => "NO",
			Self::NP  => "NP",
			Self::NR  => "NR",
			Self::NU  => "NU",
			Self::NZ  => "NZ",
			Self::OM  => "OM",
			Self::PA  => "PA",
			Self::PE  => "PE",
			Self::PF  => "PF",
			Self::PG  => "PG",
			Self::PH  => "PH",
			Self::PK  => "PK",
			Self::PL  => "PL",
			Self::PM  => "PM",
			Self::PN  => "PN",
			Self::PR  => "PR",
			Self::PS  => "PS",
			Self::PT  => "PT",
			Self::PW  => "PW",
			Self::PY  => "PY",
			Self::QA  => "QA",
			Self::RE  => "RE",
			Self::RO  => "RO",
			Self::RS  => "RS",
			Self::RU  => "RU",
			Self::RW  => "RW",
			Self::SA  => "SA",
			Self::SB  => "SB",
			Self::SC  => "SC",
			Self::SD  => "SD",
			Self::SE  => "SE",
			Self::SG  => "SG",
			Self::SH  => "SH",
			Self::SI  => "SI",
			Self::SJ  => "SJ",
			Self::SK  => "SK",
			Self::SL  => "SL",
			Self::SM  => "SM",
			Self::SN  => "SN",
			Self::SO  => "SO",
			Self::SR  => "SR",
			Self::SS  => "SS",
			Self::ST  => "ST",
			Self::SV  => "SV",
			Self::SX  => "SX",
			Self::SY  => "SY",
			Self::SZ  => "SZ",
			Self::TC  => "TC",
			Self::TD  => "TD",
			Self::TF  => "TF",
			Self::TG  => "TG",
			Self::TH  => "TH",
			Self::TJ  => "TJ",
			Self::TK  => "TK",
			Self::TL  => "TL",
			Self::TM  => "TM",
			Self::TN  => "TN",
			Self::TO  => "TO",
			Self::TR  => "TR",
			Self::TT  => "TT",
			Self::TV  => "TV",
			Self::TW  => "TW",
			Self::TZ  => "TZ",
			Self::UA  => "UA",
			Self::UG  => "UG",
			Self::UM  => "UM",
			Self::US  => "US",
			Self::UY  => "UY",
			Self::UZ  => "UZ",
			Self::VA  => "VA",
			Self::VC  => "VC",
			Self::VE  => "VE",
			Self::VG  => "VG",
			Self::VI  => "VI",
			Self::VN  => "VN",
			Self::VU  => "VU",
			Self::WF  => "WF",
			Self::WS  => "WS",
			Self::YE  => "YE",
			Self::YT  => "YT",
			Self::ZA  => "ZA",
			Self::ZM  => "ZM",
			Self::ZW  => "ZW",
			//		Three-letter codes (ISO 3166-1 alpha-3)						
			Self::ABW => "ABW",
			Self::AFG => "AFG",
			Self::AGO => "AGO",
			Self::AIA => "AIA",
			Self::ALA => "ALA",
			Self::ALB => "ALB",
			Self::AND => "AND",
			Self::ARE => "ARE",
			Self::ARG => "ARG",
			Self::ARM => "ARM",
			Self::ASM => "ASM",
			Self::ATA => "ATA",
			Self::ATF => "ATF",
			Self::ATG => "ATG",
			Self::AUS => "AUS",
			Self::AUT => "AUT",
			Self::AZE => "AZE",
			Self::BDI => "BDI",
			Self::BEL => "BEL",
			Self::BEN => "BEN",
			Self::BES => "BES",
			Self::BFA => "BFA",
			Self::BGD => "BGD",
			Self::BGR => "BGR",
			Self::BHR => "BHR",
			Self::BHS => "BHS",
			Self::BIH => "BIH",
			Self::BLM => "BLM",
			Self::BLR => "BLR",
			Self::BLZ => "BLZ",
			Self::BMU => "BMU",
			Self::BOL => "BOL",
			Self::BRA => "BRA",
			Self::BRB => "BRB",
			Self::BRN => "BRN",
			Self::BTN => "BTN",
			Self::BVT => "BVT",
			Self::BWA => "BWA",
			Self::CAF => "CAF",
			Self::CAN => "CAN",
			Self::CCK => "CCK",
			Self::CHE => "CHE",
			Self::CHL => "CHL",
			Self::CHN => "CHN",
			Self::CIV => "CIV",
			Self::CMR => "CMR",
			Self::COD => "COD",
			Self::COG => "COG",
			Self::COK => "COK",
			Self::COL => "COL",
			Self::COM => "COM",
			Self::CPV => "CPV",
			Self::CRI => "CRI",
			Self::CUB => "CUB",
			Self::CUW => "CUW",
			Self::CXR => "CXR",
			Self::CYM => "CYM",
			Self::CYP => "CYP",
			Self::CZE => "CZE",
			Self::DEU => "DEU",
			Self::DJI => "DJI",
			Self::DMA => "DMA",
			Self::DNK => "DNK",
			Self::DOM => "DOM",
			Self::DZA => "DZA",
			Self::ECU => "ECU",
			Self::EGY => "EGY",
			Self::ERI => "ERI",
			Self::ESH => "ESH",
			Self::ESP => "ESP",
			Self::EST => "EST",
			Self::ETH => "ETH",
			Self::FIN => "FIN",
			Self::FJI => "FJI",
			Self::FLK => "FLK",
			Self::FRA => "FRA",
			Self::FRO => "FRO",
			Self::FSM => "FSM",
			Self::GAB => "GAB",
			Self::GBR => "GBR",
			Self::GEO => "GEO",
			Self::GGY => "GGY",
			Self::GHA => "GHA",
			Self::GIB => "GIB",
			Self::GIN => "GIN",
			Self::GLP => "GLP",
			Self::GMB => "GMB",
			Self::GNB => "GNB",
			Self::GNQ => "GNQ",
			Self::GRC => "GRC",
			Self::GRD => "GRD",
			Self::GRL => "GRL",
			Self::GTM => "GTM",
			Self::GUF => "GUF",
			Self::GUM => "GUM",
			Self::GUY => "GUY",
			Self::HKG => "HKG",
			Self::HMD => "HMD",
			Self::HND => "HND",
			Self::HRV => "HRV",
			Self::HTI => "HTI",
			Self::HUN => "HUN",
			Self::IDN => "IDN",
			Self::IMN => "IMN",
			Self::IND => "IND",
			Self::IOT => "IOT",
			Self::IRL => "IRL",
			Self::IRN => "IRN",
			Self::IRQ => "IRQ",
			Self::ISL => "ISL",
			Self::ISR => "ISR",
			Self::ITA => "ITA",
			Self::JAM => "JAM",
			Self::JEY => "JEY",
			Self::JOR => "JOR",
			Self::JPN => "JPN",
			Self::KAZ => "KAZ",
			Self::KEN => "KEN",
			Self::KGZ => "KGZ",
			Self::KHM => "KHM",
			Self::KIR => "KIR",
			Self::KNA => "KNA",
			Self::KOR => "KOR",
			Self::KWT => "KWT",
			Self::LAO => "LAO",
			Self::LBN => "LBN",
			Self::LBR => "LBR",
			Self::LBY => "LBY",
			Self::LCA => "LCA",
			Self::LIE => "LIE",
			Self::LKA => "LKA",
			Self::LSO => "LSO",
			Self::LTU => "LTU",
			Self::LUX => "LUX",
			Self::LVA => "LVA",
			Self::MAC => "MAC",
			Self::MAF => "MAF",
			Self::MAR => "MAR",
			Self::MCO => "MCO",
			Self::MDA => "MDA",
			Self::MDG => "MDG",
			Self::MDV => "MDV",
			Self::MEX => "MEX",
			Self::MHL => "MHL",
			Self::MKD => "MKD",
			Self::MLI => "MLI",
			Self::MLT => "MLT",
			Self::MMR => "MMR",
			Self::MNE => "MNE",
			Self::MNG => "MNG",
			Self::MNP => "MNP",
			Self::MOZ => "MOZ",
			Self::MRT => "MRT",
			Self::MSR => "MSR",
			Self::MTQ => "MTQ",
			Self::MUS => "MUS",
			Self::MWI => "MWI",
			Self::MYS => "MYS",
			Self::MYT => "MYT",
			Self::NAM => "NAM",
			Self::NCL => "NCL",
			Self::NER => "NER",
			Self::NFK => "NFK",
			Self::NGA => "NGA",
			Self::NIC => "NIC",
			Self::NIU => "NIU",
			Self::NLD => "NLD",
			Self::NOR => "NOR",
			Self::NPL => "NPL",
			Self::NRU => "NRU",
			Self::NZL => "NZL",
			Self::OMN => "OMN",
			Self::PAK => "PAK",
			Self::PAN => "PAN",
			Self::PCN => "PCN",
			Self::PER => "PER",
			Self::PHL => "PHL",
			Self::PLW => "PLW",
			Self::PNG => "PNG",
			Self::POL => "POL",
			Self::PRI => "PRI",
			Self::PRK => "PRK",
			Self::PRT => "PRT",
			Self::PRY => "PRY",
			Self::PSE => "PSE",
			Self::PYF => "PYF",
			Self::QAT => "QAT",
			Self::REU => "REU",
			Self::ROU => "ROU",
			Self::RUS => "RUS",
			Self::RWA => "RWA",
			Self::SAU => "SAU",
			Self::SDN => "SDN",
			Self::SEN => "SEN",
			Self::SGP => "SGP",
			Self::SGS => "SGS",
			Self::SHN => "SHN",
			Self::SJM => "SJM",
			Self::SLB => "SLB",
			Self::SLE => "SLE",
			Self::SLV => "SLV",
			Self::SMR => "SMR",
			Self::SOM => "SOM",
			Self::SPM => "SPM",
			Self::SRB => "SRB",
			Self::SSD => "SSD",
			Self::STP => "STP",
			Self::SUR => "SUR",
			Self::SVK => "SVK",
			Self::SVN => "SVN",
			Self::SWE => "SWE",
			Self::SWZ => "SWZ",
			Self::SXM => "SXM",
			Self::SYC => "SYC",
			Self::SYR => "SYR",
			Self::TCA => "TCA",
			Self::TCD => "TCD",
			Self::TGO => "TGO",
			Self::THA => "THA",
			Self::TJK => "TJK",
			Self::TKL => "TKL",
			Self::TKM => "TKM",
			Self::TLS => "TLS",
			Self::TON => "TON",
			Self::TTO => "TTO",
			Self::TUN => "TUN",
			Self::TUR => "TUR",
			Self::TUV => "TUV",
			Self::TWN => "TWN",
			Self::TZA => "TZA",
			Self::UGA => "UGA",
			Self::UKR => "UKR",
			Self::UMI => "UMI",
			Self::URY => "URY",
			Self::USA => "USA",
			Self::UZB => "UZB",
			Self::VAT => "VAT",
			Self::VCT => "VCT",
			Self::VEN => "VEN",
			Self::VGB => "VGB",
			Self::VIR => "VIR",
			Self::VNM => "VNM",
			Self::VUT => "VUT",
			Self::WLF => "WLF",
			Self::WSM => "WSM",
			Self::YEM => "YEM",
			Self::ZAF => "ZAF",
			Self::ZMB => "ZMB",
			Self::ZWE => "ZWE",
		}
	}
	
	//		country																
	/// Returns the `Country` instance corresponding to the `CountryCode`.
	/// 
	/// This method provides an easy way to get to the associated `Country`
	/// instance from a `CountryCode` enum variant.
	/// 
	#[cfg_attr(    feature = "reasons",  allow(clippy::missing_panics_doc, reason = "Infallible"))]
	#[cfg_attr(not(feature = "reasons"), allow(clippy::missing_panics_doc))]
	pub fn country(&self) -> &Country {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		//	This should be infallible. If it isn't, then the data is wrong, and one
		//	of the countries is missing from the list, which is a bug.
		COUNTRIES.get(self).unwrap()
	}
}

impl Display for CountryCode {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl From<CountryCode> for u16 {
	//		from																
	fn from(code: CountryCode) -> Self {
		//	For maximum ease of use, both two-letter and three-letter codes are
		//	stored in the same enum. However, this causes a collision between the
		//	numeric representations. To avoid this, the three-letter codes have
		//	1,000 added to them, for the sole purpose of internal storage. This
		//	needs to be adjusted when the enum variants are serialized or otherwise
		//	represented as an integer.
		#[cfg_attr(    feature = "reasons",  allow(clippy::arithmetic_side_effects, reason = "Range is controlled"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::arithmetic_side_effects))]
		if code as Self > 1_000 {
			code as Self - 1_000
		} else {
			code as Self
		}
	}
}

impl From<CountryCode> for String {
	//		from																
	fn from(code: CountryCode) -> Self {
		code.to_string()
	}
}

impl FromStr for CountryCode {
	type Err = String;
	
	//		from_str															
	#[cfg_attr(    feature = "reasons",  allow(clippy::too_many_lines, reason = "Data not logic"))]
	#[cfg_attr(not(feature = "reasons"), allow(clippy::too_many_lines))]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			//		Two-letter codes (ISO 3166-1 alpha-2)						
			"AD"  => Ok(Self::AD),
			"AE"  => Ok(Self::AE),
			"AF"  => Ok(Self::AF),
			"AG"  => Ok(Self::AG),
			"AI"  => Ok(Self::AI),
			"AL"  => Ok(Self::AL),
			"AM"  => Ok(Self::AM),
			"AO"  => Ok(Self::AO),
			"AQ"  => Ok(Self::AQ),
			"AR"  => Ok(Self::AR),
			"AS"  => Ok(Self::AS),
			"AT"  => Ok(Self::AT),
			"AU"  => Ok(Self::AU),
			"AW"  => Ok(Self::AW),
			"AX"  => Ok(Self::AX),
			"AZ"  => Ok(Self::AZ),
			"BA"  => Ok(Self::BA),
			"BB"  => Ok(Self::BB),
			"BD"  => Ok(Self::BD),
			"BE"  => Ok(Self::BE),
			"BF"  => Ok(Self::BF),
			"BG"  => Ok(Self::BG),
			"BH"  => Ok(Self::BH),
			"BI"  => Ok(Self::BI),
			"BJ"  => Ok(Self::BJ),
			"BL"  => Ok(Self::BL),
			"BM"  => Ok(Self::BM),
			"BN"  => Ok(Self::BN),
			"BO"  => Ok(Self::BO),
			"BQ"  => Ok(Self::BQ),
			"BR"  => Ok(Self::BR),
			"BS"  => Ok(Self::BS),
			"BT"  => Ok(Self::BT),
			"BV"  => Ok(Self::BV),
			"BW"  => Ok(Self::BW),
			"BY"  => Ok(Self::BY),
			"BZ"  => Ok(Self::BZ),
			"CA"  => Ok(Self::CA),
			"CC"  => Ok(Self::CC),
			"CD"  => Ok(Self::CD),
			"CF"  => Ok(Self::CF),
			"CG"  => Ok(Self::CG),
			"CH"  => Ok(Self::CH),
			"CI"  => Ok(Self::CI),
			"CK"  => Ok(Self::CK),
			"CL"  => Ok(Self::CL),
			"CM"  => Ok(Self::CM),
			"CN"  => Ok(Self::CN),
			"CO"  => Ok(Self::CO),
			"CR"  => Ok(Self::CR),
			"CU"  => Ok(Self::CU),
			"CV"  => Ok(Self::CV),
			"CW"  => Ok(Self::CW),
			"CX"  => Ok(Self::CX),
			"CY"  => Ok(Self::CY),
			"CZ"  => Ok(Self::CZ),
			"DE"  => Ok(Self::DE),
			"DJ"  => Ok(Self::DJ),
			"DK"  => Ok(Self::DK),
			"DM"  => Ok(Self::DM),
			"DO"  => Ok(Self::DO),
			"DZ"  => Ok(Self::DZ),
			"EC"  => Ok(Self::EC),
			"EE"  => Ok(Self::EE),
			"EG"  => Ok(Self::EG),
			"EH"  => Ok(Self::EH),
			"ER"  => Ok(Self::ER),
			"ES"  => Ok(Self::ES),
			"ET"  => Ok(Self::ET),
			"FI"  => Ok(Self::FI),
			"FJ"  => Ok(Self::FJ),
			"FK"  => Ok(Self::FK),
			"FM"  => Ok(Self::FM),
			"FO"  => Ok(Self::FO),
			"FR"  => Ok(Self::FR),
			"GA"  => Ok(Self::GA),
			"GB"  => Ok(Self::GB),
			"GD"  => Ok(Self::GD),
			"GE"  => Ok(Self::GE),
			"GF"  => Ok(Self::GF),
			"GG"  => Ok(Self::GG),
			"GH"  => Ok(Self::GH),
			"GI"  => Ok(Self::GI),
			"GL"  => Ok(Self::GL),
			"GM"  => Ok(Self::GM),
			"GN"  => Ok(Self::GN),
			"GP"  => Ok(Self::GP),
			"GQ"  => Ok(Self::GQ),
			"GR"  => Ok(Self::GR),
			"GS"  => Ok(Self::GS),
			"GT"  => Ok(Self::GT),
			"GU"  => Ok(Self::GU),
			"GW"  => Ok(Self::GW),
			"GY"  => Ok(Self::GY),
			"HK"  => Ok(Self::HK),
			"HM"  => Ok(Self::HM),
			"HN"  => Ok(Self::HN),
			"HR"  => Ok(Self::HR),
			"HT"  => Ok(Self::HT),
			"HU"  => Ok(Self::HU),
			"ID"  => Ok(Self::ID),
			"IE"  => Ok(Self::IE),
			"IL"  => Ok(Self::IL),
			"IM"  => Ok(Self::IM),
			"IN"  => Ok(Self::IN),
			"IO"  => Ok(Self::IO),
			"IQ"  => Ok(Self::IQ),
			"IR"  => Ok(Self::IR),
			"IS"  => Ok(Self::IS),
			"IT"  => Ok(Self::IT),
			"JE"  => Ok(Self::JE),
			"JM"  => Ok(Self::JM),
			"JO"  => Ok(Self::JO),
			"JP"  => Ok(Self::JP),
			"KE"  => Ok(Self::KE),
			"KG"  => Ok(Self::KG),
			"KH"  => Ok(Self::KH),
			"KI"  => Ok(Self::KI),
			"KM"  => Ok(Self::KM),
			"KN"  => Ok(Self::KN),
			"KP"  => Ok(Self::KP),
			"KR"  => Ok(Self::KR),
			"KW"  => Ok(Self::KW),
			"KY"  => Ok(Self::KY),
			"KZ"  => Ok(Self::KZ),
			"LA"  => Ok(Self::LA),
			"LB"  => Ok(Self::LB),
			"LC"  => Ok(Self::LC),
			"LI"  => Ok(Self::LI),
			"LK"  => Ok(Self::LK),
			"LR"  => Ok(Self::LR),
			"LS"  => Ok(Self::LS),
			"LT"  => Ok(Self::LT),
			"LU"  => Ok(Self::LU),
			"LV"  => Ok(Self::LV),
			"LY"  => Ok(Self::LY),
			"MA"  => Ok(Self::MA),
			"MC"  => Ok(Self::MC),
			"MD"  => Ok(Self::MD),
			"ME"  => Ok(Self::ME),
			"MF"  => Ok(Self::MF),
			"MG"  => Ok(Self::MG),
			"MH"  => Ok(Self::MH),
			"MK"  => Ok(Self::MK),
			"ML"  => Ok(Self::ML),
			"MM"  => Ok(Self::MM),
			"MN"  => Ok(Self::MN),
			"MO"  => Ok(Self::MO),
			"MP"  => Ok(Self::MP),
			"MQ"  => Ok(Self::MQ),
			"MR"  => Ok(Self::MR),
			"MS"  => Ok(Self::MS),
			"MT"  => Ok(Self::MT),
			"MU"  => Ok(Self::MU),
			"MV"  => Ok(Self::MV),
			"MW"  => Ok(Self::MW),
			"MX"  => Ok(Self::MX),
			"MY"  => Ok(Self::MY),
			"MZ"  => Ok(Self::MZ),
			"NA"  => Ok(Self::NA),
			"NC"  => Ok(Self::NC),
			"NE"  => Ok(Self::NE),
			"NF"  => Ok(Self::NF),
			"NG"  => Ok(Self::NG),
			"NI"  => Ok(Self::NI),
			"NL"  => Ok(Self::NL),
			"NO"  => Ok(Self::NO),
			"NP"  => Ok(Self::NP),
			"NR"  => Ok(Self::NR),
			"NU"  => Ok(Self::NU),
			"NZ"  => Ok(Self::NZ),
			"OM"  => Ok(Self::OM),
			"PA"  => Ok(Self::PA),
			"PE"  => Ok(Self::PE),
			"PF"  => Ok(Self::PF),
			"PG"  => Ok(Self::PG),
			"PH"  => Ok(Self::PH),
			"PK"  => Ok(Self::PK),
			"PL"  => Ok(Self::PL),
			"PM"  => Ok(Self::PM),
			"PN"  => Ok(Self::PN),
			"PR"  => Ok(Self::PR),
			"PS"  => Ok(Self::PS),
			"PT"  => Ok(Self::PT),
			"PW"  => Ok(Self::PW),
			"PY"  => Ok(Self::PY),
			"QA"  => Ok(Self::QA),
			"RE"  => Ok(Self::RE),
			"RO"  => Ok(Self::RO),
			"RS"  => Ok(Self::RS),
			"RU"  => Ok(Self::RU),
			"RW"  => Ok(Self::RW),
			"SA"  => Ok(Self::SA),
			"SB"  => Ok(Self::SB),
			"SC"  => Ok(Self::SC),
			"SD"  => Ok(Self::SD),
			"SE"  => Ok(Self::SE),
			"SG"  => Ok(Self::SG),
			"SH"  => Ok(Self::SH),
			"SI"  => Ok(Self::SI),
			"SJ"  => Ok(Self::SJ),
			"SK"  => Ok(Self::SK),
			"SL"  => Ok(Self::SL),
			"SM"  => Ok(Self::SM),
			"SN"  => Ok(Self::SN),
			"SO"  => Ok(Self::SO),
			"SR"  => Ok(Self::SR),
			"SS"  => Ok(Self::SS),
			"ST"  => Ok(Self::ST),
			"SV"  => Ok(Self::SV),
			"SX"  => Ok(Self::SX),
			"SY"  => Ok(Self::SY),
			"SZ"  => Ok(Self::SZ),
			"TC"  => Ok(Self::TC),
			"TD"  => Ok(Self::TD),
			"TF"  => Ok(Self::TF),
			"TG"  => Ok(Self::TG),
			"TH"  => Ok(Self::TH),
			"TJ"  => Ok(Self::TJ),
			"TK"  => Ok(Self::TK),
			"TL"  => Ok(Self::TL),
			"TM"  => Ok(Self::TM),
			"TN"  => Ok(Self::TN),
			"TO"  => Ok(Self::TO),
			"TR"  => Ok(Self::TR),
			"TT"  => Ok(Self::TT),
			"TV"  => Ok(Self::TV),
			"TW"  => Ok(Self::TW),
			"TZ"  => Ok(Self::TZ),
			"UA"  => Ok(Self::UA),
			"UG"  => Ok(Self::UG),
			"UM"  => Ok(Self::UM),
			"US"  => Ok(Self::US),
			"UY"  => Ok(Self::UY),
			"UZ"  => Ok(Self::UZ),
			"VA"  => Ok(Self::VA),
			"VC"  => Ok(Self::VC),
			"VE"  => Ok(Self::VE),
			"VG"  => Ok(Self::VG),
			"VI"  => Ok(Self::VI),
			"VN"  => Ok(Self::VN),
			"VU"  => Ok(Self::VU),
			"WF"  => Ok(Self::WF),
			"WS"  => Ok(Self::WS),
			"YE"  => Ok(Self::YE),
			"YT"  => Ok(Self::YT),
			"ZA"  => Ok(Self::ZA),
			"ZM"  => Ok(Self::ZM),
			"ZW"  => Ok(Self::ZW),
			//		Three-letter codes (ISO 3166-1 alpha-3)						
			"ABW" => Ok(Self::ABW),
			"AFG" => Ok(Self::AFG),
			"AGO" => Ok(Self::AGO),
			"AIA" => Ok(Self::AIA),
			"ALA" => Ok(Self::ALA),
			"ALB" => Ok(Self::ALB),
			"AND" => Ok(Self::AND),
			"ARE" => Ok(Self::ARE),
			"ARG" => Ok(Self::ARG),
			"ARM" => Ok(Self::ARM),
			"ASM" => Ok(Self::ASM),
			"ATA" => Ok(Self::ATA),
			"ATF" => Ok(Self::ATF),
			"ATG" => Ok(Self::ATG),
			"AUS" => Ok(Self::AUS),
			"AUT" => Ok(Self::AUT),
			"AZE" => Ok(Self::AZE),
			"BDI" => Ok(Self::BDI),
			"BEL" => Ok(Self::BEL),
			"BEN" => Ok(Self::BEN),
			"BES" => Ok(Self::BES),
			"BFA" => Ok(Self::BFA),
			"BGD" => Ok(Self::BGD),
			"BGR" => Ok(Self::BGR),
			"BHR" => Ok(Self::BHR),
			"BHS" => Ok(Self::BHS),
			"BIH" => Ok(Self::BIH),
			"BLM" => Ok(Self::BLM),
			"BLR" => Ok(Self::BLR),
			"BLZ" => Ok(Self::BLZ),
			"BMU" => Ok(Self::BMU),
			"BOL" => Ok(Self::BOL),
			"BRA" => Ok(Self::BRA),
			"BRB" => Ok(Self::BRB),
			"BRN" => Ok(Self::BRN),
			"BTN" => Ok(Self::BTN),
			"BVT" => Ok(Self::BVT),
			"BWA" => Ok(Self::BWA),
			"CAF" => Ok(Self::CAF),
			"CAN" => Ok(Self::CAN),
			"CCK" => Ok(Self::CCK),
			"CHE" => Ok(Self::CHE),
			"CHL" => Ok(Self::CHL),
			"CHN" => Ok(Self::CHN),
			"CIV" => Ok(Self::CIV),
			"CMR" => Ok(Self::CMR),
			"COD" => Ok(Self::COD),
			"COG" => Ok(Self::COG),
			"COK" => Ok(Self::COK),
			"COL" => Ok(Self::COL),
			"COM" => Ok(Self::COM),
			"CPV" => Ok(Self::CPV),
			"CRI" => Ok(Self::CRI),
			"CUB" => Ok(Self::CUB),
			"CUW" => Ok(Self::CUW),
			"CXR" => Ok(Self::CXR),
			"CYM" => Ok(Self::CYM),
			"CYP" => Ok(Self::CYP),
			"CZE" => Ok(Self::CZE),
			"DEU" => Ok(Self::DEU),
			"DJI" => Ok(Self::DJI),
			"DMA" => Ok(Self::DMA),
			"DNK" => Ok(Self::DNK),
			"DOM" => Ok(Self::DOM),
			"DZA" => Ok(Self::DZA),
			"ECU" => Ok(Self::ECU),
			"EGY" => Ok(Self::EGY),
			"ERI" => Ok(Self::ERI),
			"ESH" => Ok(Self::ESH),
			"ESP" => Ok(Self::ESP),
			"EST" => Ok(Self::EST),
			"ETH" => Ok(Self::ETH),
			"FIN" => Ok(Self::FIN),
			"FJI" => Ok(Self::FJI),
			"FLK" => Ok(Self::FLK),
			"FRA" => Ok(Self::FRA),
			"FRO" => Ok(Self::FRO),
			"FSM" => Ok(Self::FSM),
			"GAB" => Ok(Self::GAB),
			"GBR" => Ok(Self::GBR),
			"GEO" => Ok(Self::GEO),
			"GGY" => Ok(Self::GGY),
			"GHA" => Ok(Self::GHA),
			"GIB" => Ok(Self::GIB),
			"GIN" => Ok(Self::GIN),
			"GLP" => Ok(Self::GLP),
			"GMB" => Ok(Self::GMB),
			"GNB" => Ok(Self::GNB),
			"GNQ" => Ok(Self::GNQ),
			"GRC" => Ok(Self::GRC),
			"GRD" => Ok(Self::GRD),
			"GRL" => Ok(Self::GRL),
			"GTM" => Ok(Self::GTM),
			"GUF" => Ok(Self::GUF),
			"GUM" => Ok(Self::GUM),
			"GUY" => Ok(Self::GUY),
			"HKG" => Ok(Self::HKG),
			"HMD" => Ok(Self::HMD),
			"HND" => Ok(Self::HND),
			"HRV" => Ok(Self::HRV),
			"HTI" => Ok(Self::HTI),
			"HUN" => Ok(Self::HUN),
			"IDN" => Ok(Self::IDN),
			"IMN" => Ok(Self::IMN),
			"IND" => Ok(Self::IND),
			"IOT" => Ok(Self::IOT),
			"IRL" => Ok(Self::IRL),
			"IRN" => Ok(Self::IRN),
			"IRQ" => Ok(Self::IRQ),
			"ISL" => Ok(Self::ISL),
			"ISR" => Ok(Self::ISR),
			"ITA" => Ok(Self::ITA),
			"JAM" => Ok(Self::JAM),
			"JEY" => Ok(Self::JEY),
			"JOR" => Ok(Self::JOR),
			"JPN" => Ok(Self::JPN),
			"KAZ" => Ok(Self::KAZ),
			"KEN" => Ok(Self::KEN),
			"KGZ" => Ok(Self::KGZ),
			"KHM" => Ok(Self::KHM),
			"KIR" => Ok(Self::KIR),
			"KNA" => Ok(Self::KNA),
			"KOR" => Ok(Self::KOR),
			"KWT" => Ok(Self::KWT),
			"LAO" => Ok(Self::LAO),
			"LBN" => Ok(Self::LBN),
			"LBR" => Ok(Self::LBR),
			"LBY" => Ok(Self::LBY),
			"LCA" => Ok(Self::LCA),
			"LIE" => Ok(Self::LIE),
			"LKA" => Ok(Self::LKA),
			"LSO" => Ok(Self::LSO),
			"LTU" => Ok(Self::LTU),
			"LUX" => Ok(Self::LUX),
			"LVA" => Ok(Self::LVA),
			"MAC" => Ok(Self::MAC),
			"MAF" => Ok(Self::MAF),
			"MAR" => Ok(Self::MAR),
			"MCO" => Ok(Self::MCO),
			"MDA" => Ok(Self::MDA),
			"MDG" => Ok(Self::MDG),
			"MDV" => Ok(Self::MDV),
			"MEX" => Ok(Self::MEX),
			"MHL" => Ok(Self::MHL),
			"MKD" => Ok(Self::MKD),
			"MLI" => Ok(Self::MLI),
			"MLT" => Ok(Self::MLT),
			"MMR" => Ok(Self::MMR),
			"MNE" => Ok(Self::MNE),
			"MNG" => Ok(Self::MNG),
			"MNP" => Ok(Self::MNP),
			"MOZ" => Ok(Self::MOZ),
			"MRT" => Ok(Self::MRT),
			"MSR" => Ok(Self::MSR),
			"MTQ" => Ok(Self::MTQ),
			"MUS" => Ok(Self::MUS),
			"MWI" => Ok(Self::MWI),
			"MYS" => Ok(Self::MYS),
			"MYT" => Ok(Self::MYT),
			"NAM" => Ok(Self::NAM),
			"NCL" => Ok(Self::NCL),
			"NER" => Ok(Self::NER),
			"NFK" => Ok(Self::NFK),
			"NGA" => Ok(Self::NGA),
			"NIC" => Ok(Self::NIC),
			"NIU" => Ok(Self::NIU),
			"NLD" => Ok(Self::NLD),
			"NOR" => Ok(Self::NOR),
			"NPL" => Ok(Self::NPL),
			"NRU" => Ok(Self::NRU),
			"NZL" => Ok(Self::NZL),
			"OMN" => Ok(Self::OMN),
			"PAK" => Ok(Self::PAK),
			"PAN" => Ok(Self::PAN),
			"PCN" => Ok(Self::PCN),
			"PER" => Ok(Self::PER),
			"PHL" => Ok(Self::PHL),
			"PLW" => Ok(Self::PLW),
			"PNG" => Ok(Self::PNG),
			"POL" => Ok(Self::POL),
			"PRI" => Ok(Self::PRI),
			"PRK" => Ok(Self::PRK),
			"PRT" => Ok(Self::PRT),
			"PRY" => Ok(Self::PRY),
			"PSE" => Ok(Self::PSE),
			"PYF" => Ok(Self::PYF),
			"QAT" => Ok(Self::QAT),
			"REU" => Ok(Self::REU),
			"ROU" => Ok(Self::ROU),
			"RUS" => Ok(Self::RUS),
			"RWA" => Ok(Self::RWA),
			"SAU" => Ok(Self::SAU),
			"SDN" => Ok(Self::SDN),
			"SEN" => Ok(Self::SEN),
			"SGP" => Ok(Self::SGP),
			"SGS" => Ok(Self::SGS),
			"SHN" => Ok(Self::SHN),
			"SJM" => Ok(Self::SJM),
			"SLB" => Ok(Self::SLB),
			"SLE" => Ok(Self::SLE),
			"SLV" => Ok(Self::SLV),
			"SMR" => Ok(Self::SMR),
			"SOM" => Ok(Self::SOM),
			"SPM" => Ok(Self::SPM),
			"SRB" => Ok(Self::SRB),
			"SSD" => Ok(Self::SSD),
			"STP" => Ok(Self::STP),
			"SUR" => Ok(Self::SUR),
			"SVK" => Ok(Self::SVK),
			"SVN" => Ok(Self::SVN),
			"SWE" => Ok(Self::SWE),
			"SWZ" => Ok(Self::SWZ),
			"SXM" => Ok(Self::SXM),
			"SYC" => Ok(Self::SYC),
			"SYR" => Ok(Self::SYR),
			"TCA" => Ok(Self::TCA),
			"TCD" => Ok(Self::TCD),
			"TGO" => Ok(Self::TGO),
			"THA" => Ok(Self::THA),
			"TJK" => Ok(Self::TJK),
			"TKL" => Ok(Self::TKL),
			"TKM" => Ok(Self::TKM),
			"TLS" => Ok(Self::TLS),
			"TON" => Ok(Self::TON),
			"TTO" => Ok(Self::TTO),
			"TUN" => Ok(Self::TUN),
			"TUR" => Ok(Self::TUR),
			"TUV" => Ok(Self::TUV),
			"TWN" => Ok(Self::TWN),
			"TZA" => Ok(Self::TZA),
			"UGA" => Ok(Self::UGA),
			"UKR" => Ok(Self::UKR),
			"UMI" => Ok(Self::UMI),
			"URY" => Ok(Self::URY),
			"USA" => Ok(Self::USA),
			"UZB" => Ok(Self::UZB),
			"VAT" => Ok(Self::VAT),
			"VCT" => Ok(Self::VCT),
			"VEN" => Ok(Self::VEN),
			"VGB" => Ok(Self::VGB),
			"VIR" => Ok(Self::VIR),
			"VNM" => Ok(Self::VNM),
			"VUT" => Ok(Self::VUT),
			"WLF" => Ok(Self::WLF),
			"WSM" => Ok(Self::WSM),
			"YEM" => Ok(Self::YEM),
			"ZAF" => Ok(Self::ZAF),
			"ZMB" => Ok(Self::ZMB),
			"ZWE" => Ok(Self::ZWE),
			//		Invalid														
			_     => Err(format!("Invalid CountryCode: {s}")),
		}
	}
}

#[cfg_attr(    feature = "reasons",  allow(clippy::zero_prefixed_literal, reason = "Zeroes aid readability here"))]
#[cfg_attr(not(feature = "reasons"), allow(clippy::zero_prefixed_literal))]
impl TryFrom<u16> for CountryCode {
	type Error = String;
	
	//		try_from															
	#[cfg_attr(    feature = "reasons",  allow(clippy::too_many_lines, reason = "Data not logic"))]
	#[cfg_attr(not(feature = "reasons"), allow(clippy::too_many_lines))]
	fn try_from(value: u16) -> Result<Self, Self::Error> {
		match value {
			//		Two-letter codes (ISO 3166-1 alpha-2)								
			//	The two-letter codes are chosen in preference to the three-letter codes
			//	when converting from numerical representation.
			004 => Ok(Self::AF),
			008 => Ok(Self::AL),
			010 => Ok(Self::AQ),
			012 => Ok(Self::DZ),
			016 => Ok(Self::AS),
			020 => Ok(Self::AD),
			024 => Ok(Self::AO),
			028 => Ok(Self::AG),
			031 => Ok(Self::AZ),
			032 => Ok(Self::AR),
			036 => Ok(Self::AU),
			040 => Ok(Self::AT),
			044 => Ok(Self::BS),
			048 => Ok(Self::BH),
			050 => Ok(Self::BD),
			051 => Ok(Self::AM),
			052 => Ok(Self::BB),
			056 => Ok(Self::BE),
			060 => Ok(Self::BM),
			064 => Ok(Self::BT),
			068 => Ok(Self::BO),
			070 => Ok(Self::BA),
			072 => Ok(Self::BW),
			074 => Ok(Self::BV),
			076 => Ok(Self::BR),
			084 => Ok(Self::BZ),
			086 => Ok(Self::IO),
			090 => Ok(Self::SB),
			092 => Ok(Self::VG),
			096 => Ok(Self::BN),
			100 => Ok(Self::BG),
			104 => Ok(Self::MM),
			108 => Ok(Self::BI),
			112 => Ok(Self::BY),
			116 => Ok(Self::KH),
			120 => Ok(Self::CM),
			124 => Ok(Self::CA),
			132 => Ok(Self::CV),
			136 => Ok(Self::KY),
			140 => Ok(Self::CF),
			144 => Ok(Self::LK),
			148 => Ok(Self::TD),
			152 => Ok(Self::CL),
			156 => Ok(Self::CN),
			158 => Ok(Self::TW),
			162 => Ok(Self::CX),
			166 => Ok(Self::CC),
			170 => Ok(Self::CO),
			174 => Ok(Self::KM),
			175 => Ok(Self::YT),
			178 => Ok(Self::CG),
			180 => Ok(Self::CD),
			184 => Ok(Self::CK),
			188 => Ok(Self::CR),
			191 => Ok(Self::HR),
			192 => Ok(Self::CU),
			196 => Ok(Self::CY),
			203 => Ok(Self::CZ),
			204 => Ok(Self::BJ),
			208 => Ok(Self::DK),
			212 => Ok(Self::DM),
			214 => Ok(Self::DO),
			218 => Ok(Self::EC),
			222 => Ok(Self::SV),
			226 => Ok(Self::GQ),
			231 => Ok(Self::ET),
			232 => Ok(Self::ER),
			233 => Ok(Self::EE),
			234 => Ok(Self::FO),
			238 => Ok(Self::FK),
			239 => Ok(Self::GS),
			242 => Ok(Self::FJ),
			246 => Ok(Self::FI),
			248 => Ok(Self::AX),
			250 => Ok(Self::FR),
			254 => Ok(Self::GF),
			258 => Ok(Self::PF),
			260 => Ok(Self::TF),
			262 => Ok(Self::DJ),
			266 => Ok(Self::GA),
			268 => Ok(Self::GE),
			270 => Ok(Self::GM),
			275 => Ok(Self::PS),
			276 => Ok(Self::DE),
			288 => Ok(Self::GH),
			292 => Ok(Self::GI),
			296 => Ok(Self::KI),
			300 => Ok(Self::GR),
			304 => Ok(Self::GL),
			308 => Ok(Self::GD),
			312 => Ok(Self::GP),
			316 => Ok(Self::GU),
			320 => Ok(Self::GT),
			324 => Ok(Self::GN),
			328 => Ok(Self::GY),
			332 => Ok(Self::HT),
			334 => Ok(Self::HM),
			336 => Ok(Self::VA),
			340 => Ok(Self::HN),
			344 => Ok(Self::HK),
			348 => Ok(Self::HU),
			352 => Ok(Self::IS),
			356 => Ok(Self::IN),
			360 => Ok(Self::ID),
			364 => Ok(Self::IR),
			368 => Ok(Self::IQ),
			372 => Ok(Self::IE),
			376 => Ok(Self::IL),
			380 => Ok(Self::IT),
			384 => Ok(Self::CI),
			388 => Ok(Self::JM),
			392 => Ok(Self::JP),
			398 => Ok(Self::KZ),
			400 => Ok(Self::JO),
			404 => Ok(Self::KE),
			408 => Ok(Self::KP),
			410 => Ok(Self::KR),
			414 => Ok(Self::KW),
			417 => Ok(Self::KG),
			418 => Ok(Self::LA),
			422 => Ok(Self::LB),
			426 => Ok(Self::LS),
			428 => Ok(Self::LV),
			430 => Ok(Self::LR),
			434 => Ok(Self::LY),
			438 => Ok(Self::LI),
			440 => Ok(Self::LT),
			442 => Ok(Self::LU),
			446 => Ok(Self::MO),
			450 => Ok(Self::MG),
			454 => Ok(Self::MW),
			458 => Ok(Self::MY),
			462 => Ok(Self::MV),
			466 => Ok(Self::ML),
			470 => Ok(Self::MT),
			474 => Ok(Self::MQ),
			478 => Ok(Self::MR),
			480 => Ok(Self::MU),
			484 => Ok(Self::MX),
			492 => Ok(Self::MC),
			496 => Ok(Self::MN),
			498 => Ok(Self::MD),
			499 => Ok(Self::ME),
			500 => Ok(Self::MS),
			504 => Ok(Self::MA),
			508 => Ok(Self::MZ),
			512 => Ok(Self::OM),
			516 => Ok(Self::NA),
			520 => Ok(Self::NR),
			524 => Ok(Self::NP),
			528 => Ok(Self::NL),
			531 => Ok(Self::CW),
			533 => Ok(Self::AW),
			534 => Ok(Self::SX),
			535 => Ok(Self::BQ),
			540 => Ok(Self::NC),
			548 => Ok(Self::VU),
			554 => Ok(Self::NZ),
			558 => Ok(Self::NI),
			562 => Ok(Self::NE),
			566 => Ok(Self::NG),
			570 => Ok(Self::NU),
			574 => Ok(Self::NF),
			578 => Ok(Self::NO),
			580 => Ok(Self::MP),
			581 => Ok(Self::UM),
			583 => Ok(Self::FM),
			584 => Ok(Self::MH),
			585 => Ok(Self::PW),
			586 => Ok(Self::PK),
			591 => Ok(Self::PA),
			598 => Ok(Self::PG),
			600 => Ok(Self::PY),
			604 => Ok(Self::PE),
			608 => Ok(Self::PH),
			612 => Ok(Self::PN),
			616 => Ok(Self::PL),
			620 => Ok(Self::PT),
			624 => Ok(Self::GW),
			626 => Ok(Self::TL),
			630 => Ok(Self::PR),
			634 => Ok(Self::QA),
			638 => Ok(Self::RE),
			642 => Ok(Self::RO),
			643 => Ok(Self::RU),
			646 => Ok(Self::RW),
			652 => Ok(Self::BL),
			654 => Ok(Self::SH),
			659 => Ok(Self::KN),
			660 => Ok(Self::AI),
			662 => Ok(Self::LC),
			663 => Ok(Self::MF),
			666 => Ok(Self::PM),
			670 => Ok(Self::VC),
			674 => Ok(Self::SM),
			678 => Ok(Self::ST),
			682 => Ok(Self::SA),
			686 => Ok(Self::SN),
			688 => Ok(Self::RS),
			690 => Ok(Self::SC),
			694 => Ok(Self::SL),
			702 => Ok(Self::SG),
			703 => Ok(Self::SK),
			704 => Ok(Self::VN),
			705 => Ok(Self::SI),
			706 => Ok(Self::SO),
			710 => Ok(Self::ZA),
			716 => Ok(Self::ZW),
			724 => Ok(Self::ES),
			728 => Ok(Self::SS),
			729 => Ok(Self::SD),
			732 => Ok(Self::EH),
			740 => Ok(Self::SR),
			744 => Ok(Self::SJ),
			748 => Ok(Self::SZ),
			752 => Ok(Self::SE),
			756 => Ok(Self::CH),
			760 => Ok(Self::SY),
			762 => Ok(Self::TJ),
			764 => Ok(Self::TH),
			768 => Ok(Self::TG),
			772 => Ok(Self::TK),
			776 => Ok(Self::TO),
			780 => Ok(Self::TT),
			784 => Ok(Self::AE),
			788 => Ok(Self::TN),
			792 => Ok(Self::TR),
			795 => Ok(Self::TM),
			796 => Ok(Self::TC),
			798 => Ok(Self::TV),
			800 => Ok(Self::UG),
			804 => Ok(Self::UA),
			807 => Ok(Self::MK),
			818 => Ok(Self::EG),
			826 => Ok(Self::GB),
			831 => Ok(Self::GG),
			832 => Ok(Self::JE),
			833 => Ok(Self::IM),
			834 => Ok(Self::TZ),
			840 => Ok(Self::US),
			850 => Ok(Self::VI),
			854 => Ok(Self::BF),
			858 => Ok(Self::UY),
			860 => Ok(Self::UZ),
			862 => Ok(Self::VE),
			876 => Ok(Self::WF),
			882 => Ok(Self::WS),
			887 => Ok(Self::YE),
			894 => Ok(Self::ZM),
			//		Three-letter codes (ISO 3166-1 alpha-3)						
			//	As both the two-letter and three-letter codes have the same numerical
			//	representation, there is no specific number that will lead to a three-
			//	letter code being produced. The two-letter codes are chosen in
			//	preference, and are considered to be equivalent.
			//		Invalid														
			_   => Err(format!("Invalid CountryCode: {value}")),
		}
	}
}

impl TryFrom<String> for CountryCode {
	type Error = String;
	
	//		try_from															
	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.as_str().parse()
	}
}




//		Structs

//		Country																	
/// A country.
/// 
/// A country has a number of properties, including a name, a country code, the
/// currencies it uses, and the languages used in it.
/// 
/// Each country is identified by a country code, which can be expressed as two
/// or three letters or three numbers, as defined by the ISO 3166-1 standard.
/// 
/// # Data sources
/// 
/// The list of codes and other country information is available from
/// [the ISO site](https://www.iso.org/iso-3166-country-codes.html), and from
/// [Wikipedia](https://en.wikipedia.org/wiki/ISO_3166-1).
/// 
/// # See also
/// 
/// * [`CountryCode`]
/// 
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[non_exhaustive]
pub struct Country {
	//		Public properties													
	/// The name of the country.
	pub name:       String,
	
	/// The country code. For more information, see [`CountryCode`].
	pub code:       CountryCode,
	
	/// The currencies used in the country.
	pub currencies: HashSet<CurrencyCode>,
	
	/// The languages used in the country.
	pub languages:  HashSet<LanguageCode>,
}


