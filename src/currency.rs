//! Currency-related types.
//! 
//! This module provides ISO 4217 currencies with alpha3/numeric codes and basic
//! names. The currencies and codes are provided as enums, for ease of use and
//! performance.
//! 
//! The currencies are related to countries, and vice versa, making lookups
//! easy. The information comes from the ISO and Wikipedia.
//! 
//! The currency codes are two in one, with the alpha3 code being the variant
//! name and the numeric code being the variant value. Either can be chosen for
//! serialised form, but the default is the string representation.
//! 



//		Modules

#[cfg(test)]
#[path = "tests/currency.rs"]
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

/// The possible currencies.
/// 
/// # Data sources
/// 
/// The list of codes and other currency information is available from
/// [the ISO site](https://www.iso.org/iso-4217-currency-codes.html), and from
/// [Wikipedia](https://en.wikipedia.org/wiki/ISO_4217).
/// 
/// # See also
/// 
/// * [`CurrencyCode`]
/// * [`Currency`]
/// 
static CURRENCIES: LazyLock<HashMap<Currency, CurrencyInfo>> = LazyLock::new(|| {
	hash_map!{
		Currency::AED: CurrencyInfo { code: CurrencyCode::AED, name: s!("United Arab Emirates dirham"),                   digits: 2, countries: vh![ CountryCode: AE ] },
		Currency::AFN: CurrencyInfo { code: CurrencyCode::AFN, name: s!("Afghan afghani"),                                digits: 2, countries: vh![ CountryCode: AF ] },
		Currency::ALL: CurrencyInfo { code: CurrencyCode::ALL, name: s!("Albanian lek"),                                  digits: 2, countries: vh![ CountryCode: AL ] },
		Currency::AMD: CurrencyInfo { code: CurrencyCode::AMD, name: s!("Armenian dram"),                                 digits: 2, countries: vh![ CountryCode: AM ] },
		Currency::ANG: CurrencyInfo { code: CurrencyCode::ANG, name: s!("Netherlands Antillean guilder"),                 digits: 2, countries: vh![ CountryCode: CW, SX ] },
		Currency::AOA: CurrencyInfo { code: CurrencyCode::AOA, name: s!("Angolan kwanza"),                                digits: 2, countries: vh![ CountryCode: AO ] },
		Currency::ARS: CurrencyInfo { code: CurrencyCode::ARS, name: s!("Argentine peso"),                                digits: 2, countries: vh![ CountryCode: AR ] },
		Currency::AUD: CurrencyInfo { code: CurrencyCode::AUD, name: s!("Australian dollar"),                             digits: 2, countries: vh![ CountryCode: AU, CC, CX, HM, KI, NF, NR, TV ] },
		Currency::AWG: CurrencyInfo { code: CurrencyCode::AWG, name: s!("Aruban florin"),                                 digits: 2, countries: vh![ CountryCode: AW ] },
		Currency::AZN: CurrencyInfo { code: CurrencyCode::AZN, name: s!("Azerbaijani manat"),                             digits: 2, countries: vh![ CountryCode: AZ ] },
		Currency::BAM: CurrencyInfo { code: CurrencyCode::BAM, name: s!("Bosnia and Herzegovina convertible mark"),       digits: 2, countries: vh![ CountryCode: BA ] },
		Currency::BBD: CurrencyInfo { code: CurrencyCode::BBD, name: s!("Barbados dollar"),                               digits: 2, countries: vh![ CountryCode: BB ] },
		Currency::BDT: CurrencyInfo { code: CurrencyCode::BDT, name: s!("Bangladeshi taka"),                              digits: 2, countries: vh![ CountryCode: BD ] },
		Currency::BGN: CurrencyInfo { code: CurrencyCode::BGN, name: s!("Bulgarian lev"),                                 digits: 2, countries: vh![ CountryCode: BG ] },
		Currency::BHD: CurrencyInfo { code: CurrencyCode::BHD, name: s!("Bahraini dinar"),                                digits: 3, countries: vh![ CountryCode: BH ] },
		Currency::BIF: CurrencyInfo { code: CurrencyCode::BIF, name: s!("Burundian franc"),                               digits: 0, countries: vh![ CountryCode: BI ] },
		Currency::BMD: CurrencyInfo { code: CurrencyCode::BMD, name: s!("Bermudian dollar"),                              digits: 2, countries: vh![ CountryCode: BM ] },
		Currency::BND: CurrencyInfo { code: CurrencyCode::BND, name: s!("Brunei dollar"),                                 digits: 2, countries: vh![ CountryCode: BN ] },
		Currency::BOB: CurrencyInfo { code: CurrencyCode::BOB, name: s!("Boliviano"),                                     digits: 2, countries: vh![ CountryCode: BO ] },
		Currency::BOV: CurrencyInfo { code: CurrencyCode::BOV, name: s!("Bolivian Mvdol"),                                digits: 2, countries: vh![ CountryCode: BO ] },
		Currency::BRL: CurrencyInfo { code: CurrencyCode::BRL, name: s!("Brazilian real"),                                digits: 2, countries: vh![ CountryCode: BR ] },
		Currency::BSD: CurrencyInfo { code: CurrencyCode::BSD, name: s!("Bahamian dollar"),                               digits: 2, countries: vh![ CountryCode: BS ] },
		Currency::BTN: CurrencyInfo { code: CurrencyCode::BTN, name: s!("Bhutanese ngultrum"),                            digits: 2, countries: vh![ CountryCode: BT ] },
		Currency::BWP: CurrencyInfo { code: CurrencyCode::BWP, name: s!("Botswana pula"),                                 digits: 2, countries: vh![ CountryCode: BW ] },
		Currency::BYN: CurrencyInfo { code: CurrencyCode::BYN, name: s!("Belarusian ruble"),                              digits: 2, countries: vh![ CountryCode: BY ] },
		Currency::BZD: CurrencyInfo { code: CurrencyCode::BZD, name: s!("Belize dollar"),                                 digits: 2, countries: vh![ CountryCode: BZ ] },
		Currency::CAD: CurrencyInfo { code: CurrencyCode::CAD, name: s!("Canadian dollar"),                               digits: 2, countries: vh![ CountryCode: CA ] },
		Currency::CDF: CurrencyInfo { code: CurrencyCode::CDF, name: s!("Congolese franc"),                               digits: 2, countries: vh![ CountryCode: CD ] },
		Currency::CHE: CurrencyInfo { code: CurrencyCode::CHE, name: s!("WIR euro"),                                      digits: 2, countries: vh![ CountryCode: CH ] },
		Currency::CHF: CurrencyInfo { code: CurrencyCode::CHF, name: s!("Swiss franc"),                                   digits: 2, countries: vh![ CountryCode: CH, LI ] },
		Currency::CHW: CurrencyInfo { code: CurrencyCode::CHW, name: s!("WIR franc"),                                     digits: 2, countries: vh![ CountryCode: CH ] },
		Currency::CLF: CurrencyInfo { code: CurrencyCode::CLF, name: s!("Unidad de Fomento"),                             digits: 4, countries: vh![ CountryCode: CL ] },
		Currency::CLP: CurrencyInfo { code: CurrencyCode::CLP, name: s!("Chilean peso"),                                  digits: 0, countries: vh![ CountryCode: CL ] },
		Currency::CNY: CurrencyInfo { code: CurrencyCode::CNY, name: s!("Renminbi"),                                      digits: 2, countries: vh![ CountryCode: CN ] },
		Currency::COP: CurrencyInfo { code: CurrencyCode::COP, name: s!("Colombian peso"),                                digits: 2, countries: vh![ CountryCode: CO ] },
		Currency::COU: CurrencyInfo { code: CurrencyCode::COU, name: s!("Unidad de Valor Real (UVR)"),                    digits: 2, countries: vh![ CountryCode: CO ] },
		Currency::CRC: CurrencyInfo { code: CurrencyCode::CRC, name: s!("Costa Rican colon"),                             digits: 2, countries: vh![ CountryCode: CR ] },
		Currency::CUP: CurrencyInfo { code: CurrencyCode::CUP, name: s!("Cuban peso"),                                    digits: 2, countries: vh![ CountryCode: CU ] },
		Currency::CVE: CurrencyInfo { code: CurrencyCode::CVE, name: s!("Cape Verdean escudo"),                           digits: 2, countries: vh![ CountryCode: CV ] },
		Currency::CZK: CurrencyInfo { code: CurrencyCode::CZK, name: s!("Czech koruna"),                                  digits: 2, countries: vh![ CountryCode: CZ ] },
		Currency::DJF: CurrencyInfo { code: CurrencyCode::DJF, name: s!("Djiboutian franc"),                              digits: 0, countries: vh![ CountryCode: DJ ] },
		Currency::DKK: CurrencyInfo { code: CurrencyCode::DKK, name: s!("Danish krone"),                                  digits: 2, countries: vh![ CountryCode: DK, FO, GL ] },
		Currency::DOP: CurrencyInfo { code: CurrencyCode::DOP, name: s!("Dominican peso"),                                digits: 2, countries: vh![ CountryCode: DO ] },
		Currency::DZD: CurrencyInfo { code: CurrencyCode::DZD, name: s!("Algerian dinar"),                                digits: 2, countries: vh![ CountryCode: DZ ] },
		Currency::EGP: CurrencyInfo { code: CurrencyCode::EGP, name: s!("Egyptian pound"),                                digits: 2, countries: vh![ CountryCode: EG ] },
		Currency::ERN: CurrencyInfo { code: CurrencyCode::ERN, name: s!("Eritrean nakfa"),                                digits: 2, countries: vh![ CountryCode: ER ] },
		Currency::ETB: CurrencyInfo { code: CurrencyCode::ETB, name: s!("Ethiopian birr"),                                digits: 2, countries: vh![ CountryCode: ET ] },
		Currency::EUR: CurrencyInfo { code: CurrencyCode::EUR, name: s!("Euro"),                                          digits: 2, countries: vh![ CountryCode: AD, AT, AX, BE, BL, CY, DE, EE, ES, FI, FR, GF, GP, GR, HR, IE, IT, LT, LU, LV, MC, ME, MF, MQ, MT, NL, PM, PT, RE, SI, SK, SM, TF, VA, YT ] },
		Currency::FJD: CurrencyInfo { code: CurrencyCode::FJD, name: s!("Fiji dollar"),                                   digits: 2, countries: vh![ CountryCode: FJ ] },
		Currency::FKP: CurrencyInfo { code: CurrencyCode::FKP, name: s!("Falkland Islands pound"),                        digits: 2, countries: vh![ CountryCode: FK ] },
		Currency::GBP: CurrencyInfo { code: CurrencyCode::GBP, name: s!("Pound sterling"),                                digits: 2, countries: vh![ CountryCode: GB, GG, IM, JE, SH ] },
		Currency::GEL: CurrencyInfo { code: CurrencyCode::GEL, name: s!("Georgian lari"),                                 digits: 2, countries: vh![ CountryCode: GE ] },
		Currency::GHS: CurrencyInfo { code: CurrencyCode::GHS, name: s!("Ghanaian cedi"),                                 digits: 2, countries: vh![ CountryCode: GH ] },
		Currency::GIP: CurrencyInfo { code: CurrencyCode::GIP, name: s!("Gibraltar pound"),                               digits: 2, countries: vh![ CountryCode: GI ] },
		Currency::GMD: CurrencyInfo { code: CurrencyCode::GMD, name: s!("Gambian dalasi"),                                digits: 2, countries: vh![ CountryCode: GM ] },
		Currency::GNF: CurrencyInfo { code: CurrencyCode::GNF, name: s!("Guinean franc"),                                 digits: 0, countries: vh![ CountryCode: GN ] },
		Currency::GTQ: CurrencyInfo { code: CurrencyCode::GTQ, name: s!("Guatemalan quetzal"),                            digits: 2, countries: vh![ CountryCode: GT ] },
		Currency::GYD: CurrencyInfo { code: CurrencyCode::GYD, name: s!("Guyanese dollar"),                               digits: 2, countries: vh![ CountryCode: GY ] },
		Currency::HKD: CurrencyInfo { code: CurrencyCode::HKD, name: s!("Hong Kong dollar"),                              digits: 2, countries: vh![ CountryCode: HK ] },
		Currency::HNL: CurrencyInfo { code: CurrencyCode::HNL, name: s!("Honduran lempira"),                              digits: 2, countries: vh![ CountryCode: HN ] },
		Currency::HTG: CurrencyInfo { code: CurrencyCode::HTG, name: s!("Haitian gourde"),                                digits: 2, countries: vh![ CountryCode: HT ] },
		Currency::HUF: CurrencyInfo { code: CurrencyCode::HUF, name: s!("Hungarian forint"),                              digits: 2, countries: vh![ CountryCode: HU ] },
		Currency::IDR: CurrencyInfo { code: CurrencyCode::IDR, name: s!("Indonesian rupiah"),                             digits: 2, countries: vh![ CountryCode: ID ] },
		Currency::ILS: CurrencyInfo { code: CurrencyCode::ILS, name: s!("Israeli new shekel"),                            digits: 2, countries: vh![ CountryCode: IL ] },
		Currency::INR: CurrencyInfo { code: CurrencyCode::INR, name: s!("Indian rupee"),                                  digits: 2, countries: vh![ CountryCode: BT, IN ] },
		Currency::IQD: CurrencyInfo { code: CurrencyCode::IQD, name: s!("Iraqi dinar"),                                   digits: 3, countries: vh![ CountryCode: IQ ] },
		Currency::IRR: CurrencyInfo { code: CurrencyCode::IRR, name: s!("Iranian rial"),                                  digits: 2, countries: vh![ CountryCode: IR ] },
		Currency::ISK: CurrencyInfo { code: CurrencyCode::ISK, name: s!("Icelandic króna"),                               digits: 0, countries: vh![ CountryCode: IS ] },
		Currency::JMD: CurrencyInfo { code: CurrencyCode::JMD, name: s!("Jamaican dollar"),                               digits: 2, countries: vh![ CountryCode: JM ] },
		Currency::JOD: CurrencyInfo { code: CurrencyCode::JOD, name: s!("Jordanian dinar"),                               digits: 3, countries: vh![ CountryCode: JO ] },
		Currency::JPY: CurrencyInfo { code: CurrencyCode::JPY, name: s!("Japanese yen"),                                  digits: 0, countries: vh![ CountryCode: JP ] },
		Currency::KES: CurrencyInfo { code: CurrencyCode::KES, name: s!("Kenyan shilling"),                               digits: 2, countries: vh![ CountryCode: KE ] },
		Currency::KGS: CurrencyInfo { code: CurrencyCode::KGS, name: s!("Kyrgyzstani som"),                               digits: 2, countries: vh![ CountryCode: KG ] },
		Currency::KHR: CurrencyInfo { code: CurrencyCode::KHR, name: s!("Cambodian riel"),                                digits: 2, countries: vh![ CountryCode: KH ] },
		Currency::KMF: CurrencyInfo { code: CurrencyCode::KMF, name: s!("Comoro franc"),                                  digits: 0, countries: vh![ CountryCode: KM ] },
		Currency::KPW: CurrencyInfo { code: CurrencyCode::KPW, name: s!("North Korean won"),                              digits: 2, countries: vh![ CountryCode: KP ] },
		Currency::KRW: CurrencyInfo { code: CurrencyCode::KRW, name: s!("South Korean won"),                              digits: 0, countries: vh![ CountryCode: KR ] },
		Currency::KWD: CurrencyInfo { code: CurrencyCode::KWD, name: s!("Kuwaiti dinar"),                                 digits: 3, countries: vh![ CountryCode: KW ] },
		Currency::KYD: CurrencyInfo { code: CurrencyCode::KYD, name: s!("Cayman Islands dollar"),                         digits: 2, countries: vh![ CountryCode: KY ] },
		Currency::KZT: CurrencyInfo { code: CurrencyCode::KZT, name: s!("Kazakhstani tenge"),                             digits: 2, countries: vh![ CountryCode: KZ ] },
		Currency::LAK: CurrencyInfo { code: CurrencyCode::LAK, name: s!("Lao kip"),                                       digits: 2, countries: vh![ CountryCode: LA ] },
		Currency::LBP: CurrencyInfo { code: CurrencyCode::LBP, name: s!("Lebanese pound"),                                digits: 2, countries: vh![ CountryCode: LB ] },
		Currency::LKR: CurrencyInfo { code: CurrencyCode::LKR, name: s!("Sri Lankan rupee"),                              digits: 2, countries: vh![ CountryCode: LK ] },
		Currency::LRD: CurrencyInfo { code: CurrencyCode::LRD, name: s!("Liberian dollar"),                               digits: 2, countries: vh![ CountryCode: LR ] },
		Currency::LSL: CurrencyInfo { code: CurrencyCode::LSL, name: s!("Lesotho loti"),                                  digits: 2, countries: vh![ CountryCode: LS ] },
		Currency::LYD: CurrencyInfo { code: CurrencyCode::LYD, name: s!("Libyan dinar"),                                  digits: 3, countries: vh![ CountryCode: LY ] },
		Currency::MAD: CurrencyInfo { code: CurrencyCode::MAD, name: s!("Moroccan dirham"),                               digits: 2, countries: vh![ CountryCode: EH, MA ] },
		Currency::MDL: CurrencyInfo { code: CurrencyCode::MDL, name: s!("Moldovan leu"),                                  digits: 2, countries: vh![ CountryCode: MD ] },
		Currency::MGA: CurrencyInfo { code: CurrencyCode::MGA, name: s!("Malagasy ariary"),                               digits: 2, countries: vh![ CountryCode: MG ] },
		Currency::MKD: CurrencyInfo { code: CurrencyCode::MKD, name: s!("Macedonian denar"),                              digits: 2, countries: vh![ CountryCode: MK ] },
		Currency::MMK: CurrencyInfo { code: CurrencyCode::MMK, name: s!("Myanmar kyat"),                                  digits: 2, countries: vh![ CountryCode: MM ] },
		Currency::MNT: CurrencyInfo { code: CurrencyCode::MNT, name: s!("Mongolian tögrög"),                              digits: 2, countries: vh![ CountryCode: MN ] },
		Currency::MOP: CurrencyInfo { code: CurrencyCode::MOP, name: s!("Macanese pataca"),                               digits: 2, countries: vh![ CountryCode: MO ] },
		Currency::MRU: CurrencyInfo { code: CurrencyCode::MRU, name: s!("Mauritanian ouguiya"),                           digits: 2, countries: vh![ CountryCode: MR ] },
		Currency::MUR: CurrencyInfo { code: CurrencyCode::MUR, name: s!("Mauritian rupee"),                               digits: 2, countries: vh![ CountryCode: MU ] },
		Currency::MVR: CurrencyInfo { code: CurrencyCode::MVR, name: s!("Maldivian rufiyaa"),                             digits: 2, countries: vh![ CountryCode: MV ] },
		Currency::MWK: CurrencyInfo { code: CurrencyCode::MWK, name: s!("Malawian kwacha"),                               digits: 2, countries: vh![ CountryCode: MW ] },
		Currency::MXN: CurrencyInfo { code: CurrencyCode::MXN, name: s!("Mexican peso"),                                  digits: 2, countries: vh![ CountryCode: MX ] },
		Currency::MXV: CurrencyInfo { code: CurrencyCode::MXV, name: s!("Mexican Unidad de Inversion (UDI)"),             digits: 2, countries: vh![ CountryCode: MX ] },
		Currency::MYR: CurrencyInfo { code: CurrencyCode::MYR, name: s!("Malaysian ringgit"),                             digits: 2, countries: vh![ CountryCode: MY ] },
		Currency::MZN: CurrencyInfo { code: CurrencyCode::MZN, name: s!("Mozambican metical"),                            digits: 2, countries: vh![ CountryCode: MZ ] },
		Currency::NAD: CurrencyInfo { code: CurrencyCode::NAD, name: s!("Namibian dollar"),                               digits: 2, countries: vh![ CountryCode: NA ] },
		Currency::NGN: CurrencyInfo { code: CurrencyCode::NGN, name: s!("Nigerian naira"),                                digits: 2, countries: vh![ CountryCode: NG ] },
		Currency::NIO: CurrencyInfo { code: CurrencyCode::NIO, name: s!("Nicaraguan córdoba"),                            digits: 2, countries: vh![ CountryCode: NI ] },
		Currency::NOK: CurrencyInfo { code: CurrencyCode::NOK, name: s!("Norwegian krone"),                               digits: 2, countries: vh![ CountryCode: BV, NO, SJ ] },
		Currency::NPR: CurrencyInfo { code: CurrencyCode::NPR, name: s!("Nepalese rupee"),                                digits: 2, countries: vh![ CountryCode: NP ] },
		Currency::NZD: CurrencyInfo { code: CurrencyCode::NZD, name: s!("New Zealand dollar"),                            digits: 2, countries: vh![ CountryCode: CK, NU, NZ, PN, TK ] },
		Currency::OMR: CurrencyInfo { code: CurrencyCode::OMR, name: s!("Omani rial"),                                    digits: 3, countries: vh![ CountryCode: OM ] },
		Currency::PAB: CurrencyInfo { code: CurrencyCode::PAB, name: s!("Panamanian balboa"),                             digits: 2, countries: vh![ CountryCode: PA ] },
		Currency::PEN: CurrencyInfo { code: CurrencyCode::PEN, name: s!("Peruvian sol"),                                  digits: 2, countries: vh![ CountryCode: PE ] },
		Currency::PGK: CurrencyInfo { code: CurrencyCode::PGK, name: s!("Papua New Guinean kina"),                        digits: 2, countries: vh![ CountryCode: PG ] },
		Currency::PHP: CurrencyInfo { code: CurrencyCode::PHP, name: s!("Philippine peso"),                               digits: 2, countries: vh![ CountryCode: PH ] },
		Currency::PKR: CurrencyInfo { code: CurrencyCode::PKR, name: s!("Pakistani rupee"),                               digits: 2, countries: vh![ CountryCode: PK ] },
		Currency::PLN: CurrencyInfo { code: CurrencyCode::PLN, name: s!("Polish złoty"),                                  digits: 2, countries: vh![ CountryCode: PL ] },
		Currency::PYG: CurrencyInfo { code: CurrencyCode::PYG, name: s!("Paraguayan guaraní"),                            digits: 0, countries: vh![ CountryCode: PY ] },
		Currency::QAR: CurrencyInfo { code: CurrencyCode::QAR, name: s!("Qatari riyal"),                                  digits: 2, countries: vh![ CountryCode: QA ] },
		Currency::RON: CurrencyInfo { code: CurrencyCode::RON, name: s!("Romanian leu"),                                  digits: 2, countries: vh![ CountryCode: RO ] },
		Currency::RSD: CurrencyInfo { code: CurrencyCode::RSD, name: s!("Serbian dinar"),                                 digits: 2, countries: vh![ CountryCode: RS ] },
		Currency::RUB: CurrencyInfo { code: CurrencyCode::RUB, name: s!("Russian ruble"),                                 digits: 2, countries: vh![ CountryCode: RU ] },
		Currency::RWF: CurrencyInfo { code: CurrencyCode::RWF, name: s!("Rwandan franc"),                                 digits: 0, countries: vh![ CountryCode: RW ] },
		Currency::SAR: CurrencyInfo { code: CurrencyCode::SAR, name: s!("Saudi riyal"),                                   digits: 2, countries: vh![ CountryCode: SA ] },
		Currency::SBD: CurrencyInfo { code: CurrencyCode::SBD, name: s!("Solomon Islands dollar"),                        digits: 2, countries: vh![ CountryCode: SB ] },
		Currency::SCR: CurrencyInfo { code: CurrencyCode::SCR, name: s!("Seychelles rupee"),                              digits: 2, countries: vh![ CountryCode: SC ] },
		Currency::SDG: CurrencyInfo { code: CurrencyCode::SDG, name: s!("Sudanese pound"),                                digits: 2, countries: vh![ CountryCode: SD ] },
		Currency::SEK: CurrencyInfo { code: CurrencyCode::SEK, name: s!("Swedish krona"),                                 digits: 2, countries: vh![ CountryCode: SE ] },
		Currency::SGD: CurrencyInfo { code: CurrencyCode::SGD, name: s!("Singapore dollar"),                              digits: 2, countries: vh![ CountryCode: SG ] },
		Currency::SHP: CurrencyInfo { code: CurrencyCode::SHP, name: s!("Saint Helena pound"),                            digits: 2, countries: vh![ CountryCode: SH ] },
		Currency::SLE: CurrencyInfo { code: CurrencyCode::SLE, name: s!("Sierra Leonean leone (new leone)"),              digits: 2, countries: vh![ CountryCode: SL ] },
		Currency::SLL: CurrencyInfo { code: CurrencyCode::SLL, name: s!("Sierra Leonean leone (old leone)"),              digits: 2, countries: vh![ CountryCode: SL ] },
		Currency::SOS: CurrencyInfo { code: CurrencyCode::SOS, name: s!("Somali shilling"),                               digits: 2, countries: vh![ CountryCode: SO ] },
		Currency::SRD: CurrencyInfo { code: CurrencyCode::SRD, name: s!("Surinamese dollar"),                             digits: 2, countries: vh![ CountryCode: SR ] },
		Currency::SSP: CurrencyInfo { code: CurrencyCode::SSP, name: s!("South Sudanese pound"),                          digits: 2, countries: vh![ CountryCode: SS ] },
		Currency::STN: CurrencyInfo { code: CurrencyCode::STN, name: s!("São Tomé and Príncipe dobra"),                   digits: 2, countries: vh![ CountryCode: ST ] },
		Currency::SVC: CurrencyInfo { code: CurrencyCode::SVC, name: s!("Salvadoran colón"),                              digits: 2, countries: vh![ CountryCode: SV ] },
		Currency::SYP: CurrencyInfo { code: CurrencyCode::SYP, name: s!("Syrian pound"),                                  digits: 2, countries: vh![ CountryCode: SY ] },
		Currency::SZL: CurrencyInfo { code: CurrencyCode::SZL, name: s!("Swazi lilangeni"),                               digits: 2, countries: vh![ CountryCode: SZ ] },
		Currency::THB: CurrencyInfo { code: CurrencyCode::THB, name: s!("Thai baht"),                                     digits: 2, countries: vh![ CountryCode: TH ] },
		Currency::TJS: CurrencyInfo { code: CurrencyCode::TJS, name: s!("Tajikistani somoni"),                            digits: 2, countries: vh![ CountryCode: TJ ] },
		Currency::TMT: CurrencyInfo { code: CurrencyCode::TMT, name: s!("Turkmenistan manat"),                            digits: 2, countries: vh![ CountryCode: TM ] },
		Currency::TND: CurrencyInfo { code: CurrencyCode::TND, name: s!("Tunisian dinar"),                                digits: 3, countries: vh![ CountryCode: TN ] },
		Currency::TOP: CurrencyInfo { code: CurrencyCode::TOP, name: s!("Tongan paʻanga"),                                digits: 2, countries: vh![ CountryCode: TO ] },
		Currency::TRY: CurrencyInfo { code: CurrencyCode::TRY, name: s!("Turkish lira"),                                  digits: 2, countries: vh![ CountryCode: TR ] },
		Currency::TTD: CurrencyInfo { code: CurrencyCode::TTD, name: s!("Trinidad and Tobago dollar"),                    digits: 2, countries: vh![ CountryCode: TT ] },
		Currency::TWD: CurrencyInfo { code: CurrencyCode::TWD, name: s!("New Taiwan dollar"),                             digits: 2, countries: vh![ CountryCode: TW ] },
		Currency::TZS: CurrencyInfo { code: CurrencyCode::TZS, name: s!("Tanzanian shilling"),                            digits: 2, countries: vh![ CountryCode: TZ ] },
		Currency::UAH: CurrencyInfo { code: CurrencyCode::UAH, name: s!("Ukrainian hryvnia"),                             digits: 2, countries: vh![ CountryCode: UA ] },
		Currency::UGX: CurrencyInfo { code: CurrencyCode::UGX, name: s!("Ugandan shilling"),                              digits: 0, countries: vh![ CountryCode: UG ] },
		Currency::USD: CurrencyInfo { code: CurrencyCode::USD, name: s!("United States dollar"),                          digits: 2, countries: vh![ CountryCode: AS, BQ, EC, FM, GU, IO, MH, MP, PA, PR, PW, SV, TC, TL, UM, US, VG, VI ] },
		Currency::USN: CurrencyInfo { code: CurrencyCode::USN, name: s!("United States dollar (next day)"),               digits: 2, countries: vh![ CountryCode: US ] },
		Currency::UYI: CurrencyInfo { code: CurrencyCode::UYI, name: s!("Uruguay Peso en Unidades Indexadas (URUIURUI)"), digits: 0, countries: vh![ CountryCode: UY ] },
		Currency::UYU: CurrencyInfo { code: CurrencyCode::UYU, name: s!("Uruguayan peso"),                                digits: 2, countries: vh![ CountryCode: UY ] },
		Currency::UYW: CurrencyInfo { code: CurrencyCode::UYW, name: s!("Unidad previsional"),                            digits: 4, countries: vh![ CountryCode: UY ] },
		Currency::UZS: CurrencyInfo { code: CurrencyCode::UZS, name: s!("Uzbekistan sum"),                                digits: 2, countries: vh![ CountryCode: UZ ] },
		Currency::VED: CurrencyInfo { code: CurrencyCode::VED, name: s!("Venezuelan digital bolívar"),                    digits: 2, countries: vh![ CountryCode: VE ] },
		Currency::VES: CurrencyInfo { code: CurrencyCode::VES, name: s!("Venezuelan sovereign bolívar"),                  digits: 2, countries: vh![ CountryCode: VE ] },
		Currency::VND: CurrencyInfo { code: CurrencyCode::VND, name: s!("Vietnamese đồng"),                               digits: 0, countries: vh![ CountryCode: VN ] },
		Currency::VUV: CurrencyInfo { code: CurrencyCode::VUV, name: s!("Vanuatu vatu"),                                  digits: 0, countries: vh![ CountryCode: VU ] },
		Currency::WST: CurrencyInfo { code: CurrencyCode::WST, name: s!("Samoan tala"),                                   digits: 2, countries: vh![ CountryCode: WS ] },
		Currency::XAF: CurrencyInfo { code: CurrencyCode::XAF, name: s!("CFA franc BEAC"),                                digits: 0, countries: vh![ CountryCode: CF, CG, CM, GA, GQ, TD ] },
		Currency::XAG: CurrencyInfo { code: CurrencyCode::XAG, name: s!("Silver (one troy ounce)"),                       digits: 0, countries: vh![] },
		Currency::XAU: CurrencyInfo { code: CurrencyCode::XAU, name: s!("Gold (one troy ounce)"),                         digits: 0, countries: vh![] },
		Currency::XBA: CurrencyInfo { code: CurrencyCode::XBA, name: s!("European Composite Unit (EURCO)"),               digits: 0, countries: vh![] },
		Currency::XBB: CurrencyInfo { code: CurrencyCode::XBB, name: s!("European Monetary Unit (E.M.U.-6)"),             digits: 0, countries: vh![] },
		Currency::XBC: CurrencyInfo { code: CurrencyCode::XBC, name: s!("European Unit of Account 9 (E.U.A.-9)"),         digits: 0, countries: vh![] },
		Currency::XBD: CurrencyInfo { code: CurrencyCode::XBD, name: s!("European Unit of Account 17 (E.U.A.-17)"),       digits: 0, countries: vh![] },
		Currency::XCD: CurrencyInfo { code: CurrencyCode::XCD, name: s!("East Caribbean dollar"),                         digits: 2, countries: vh![ CountryCode: AG, AI, DM, GD, KN, LC, MS, VC ] },
		Currency::XDR: CurrencyInfo { code: CurrencyCode::XDR, name: s!("Special drawing rights"),                        digits: 0, countries: vh![] },
		Currency::XOF: CurrencyInfo { code: CurrencyCode::XOF, name: s!("CFA franc BCEAO"),                               digits: 0, countries: vh![ CountryCode: BF, BJ, CI, GW, ML, NE, SN, TG ] },
		Currency::XPD: CurrencyInfo { code: CurrencyCode::XPD, name: s!("Palladium (one troy ounce)"),                    digits: 0, countries: vh![] },
		Currency::XPF: CurrencyInfo { code: CurrencyCode::XPF, name: s!("CFP franc (franc Pacifique)"),                   digits: 0, countries: vh![ CountryCode: NC, PF, WF ] },
		Currency::XPT: CurrencyInfo { code: CurrencyCode::XPT, name: s!("Platinum (one troy ounce)"),                     digits: 0, countries: vh![] },
		Currency::XSU: CurrencyInfo { code: CurrencyCode::XSU, name: s!("SUCRE"),                                         digits: 0, countries: vh![] },
		Currency::XTS: CurrencyInfo { code: CurrencyCode::XTS, name: s!("Code reserved for testing"),                     digits: 0, countries: vh![] },
		Currency::XUA: CurrencyInfo { code: CurrencyCode::XUA, name: s!("ADB Unit of Account"),                           digits: 0, countries: vh![] },
		Currency::XXX: CurrencyInfo { code: CurrencyCode::XXX, name: s!("No currency"),                                   digits: 0, countries: vh![] },
		Currency::YER: CurrencyInfo { code: CurrencyCode::YER, name: s!("Yemeni rial"),                                   digits: 2, countries: vh![ CountryCode: YE ] },
		Currency::ZAR: CurrencyInfo { code: CurrencyCode::ZAR, name: s!("South African rand"),                            digits: 2, countries: vh![ CountryCode: LS, NA, SZ, ZA ] },
		Currency::ZMW: CurrencyInfo { code: CurrencyCode::ZMW, name: s!("Zambian kwacha"),                                digits: 2, countries: vh![ CountryCode: ZM ] },
		Currency::ZWL: CurrencyInfo { code: CurrencyCode::ZWL, name: s!("Zimbabwean dollar (fifth)"),                     digits: 2, countries: vh![ CountryCode: ZW ] },
	}
});



//		Enums

//		Currency																
/// A currency.
/// 
/// A currency has a number of properties, including a name, a currency code,
/// the number of digits after the decimal point, and the countries where the
/// currency is used.
/// 
/// Each currency is identified by a currency code, which can be expressed as
/// three letters or three numbers, as defined by the ISO 4217 standard.
/// 
/// # Data sources
/// 
/// The list of codes and other currency information is available from
/// [the ISO site](https://www.iso.org/iso-4217-currency-codes.html), and from
/// [Wikipedia](https://en.wikipedia.org/wiki/ISO_4217).
/// 
/// # See also
/// 
/// * [`CurrencyCode`]
/// 
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[serde(into = "String", try_from = "String")]
#[non_exhaustive]
pub enum Currency {
	/// United Arab Emirates dirham.
	AED,
	
	/// Afghan afghani.
	AFN,
	
	/// Albanian lek.
	ALL,
	
	/// Armenian dram.
	AMD,
	
	/// Netherlands Antillean guilder.
	ANG,
	
	/// Angolan kwanza.
	AOA,
	
	/// Argentine peso.
	ARS,
	
	/// Australian dollar.
	AUD,
	
	/// Aruban florin.
	AWG,
	
	/// Azerbaijani manat.
	AZN,
	
	/// Bosnia and Herzegovina convertible mark.
	BAM,
	
	/// Barbados dollar.
	BBD,
	
	/// Bangladeshi taka.
	BDT,
	
	/// Bulgarian lev.
	BGN,
	
	/// Bahraini dinar.
	BHD,
	
	/// Burundian franc.
	BIF,
	
	/// Bermudian dollar.
	BMD,
	
	/// Brunei dollar.
	BND,
	
	/// Boliviano.
	BOB,
	
	/// Bolivian Mvdol.
	BOV,
	
	/// Brazilian real.
	BRL,
	
	/// Bahamian dollar.
	BSD,
	
	/// Bhutanese ngultrum.
	BTN,
	
	/// Botswana pula.
	BWP,
	
	/// Belarusian ruble.
	BYN,
	
	/// Belize dollar.
	BZD,
	
	/// Canadian dollar.
	CAD,
	
	/// Congolese franc.
	CDF,
	
	/// WIR euro.
	CHE,
	
	/// Swiss franc.
	CHF,
	
	/// WIR franc.
	CHW,
	
	/// Unidad de Fomento.
	CLF,
	
	/// Chilean peso.
	CLP,
	
	/// Renminbi.
	CNY,
	
	/// Colombian peso.
	COP,
	
	/// Unidad de Valor Real (UVR).
	COU,
	
	/// Costa Rican colon.
	CRC,
	
	/// Cuban peso.
	CUP,
	
	/// Cape Verdean escudo.
	CVE,
	
	/// Czech koruna.
	CZK,
	
	/// Djiboutian franc.
	DJF,
	
	/// Danish krone.
	DKK,
	
	/// Dominican peso.
	DOP,
	
	/// Algerian dinar.
	DZD,
	
	/// Egyptian pound.
	EGP,
	
	/// Eritrean nakfa.
	ERN,
	
	/// Ethiopian birr.
	ETB,
	
	/// Euro.
	EUR,
	
	/// Fiji dollar.
	FJD,
	
	/// Falkland Islands pound.
	FKP,
	
	/// Pound sterling.
	GBP,
	
	/// Georgian lari.
	GEL,
	
	/// Ghanaian cedi.
	GHS,
	
	/// Gibraltar pound.
	GIP,
	
	/// Gambian dalasi.
	GMD,
	
	/// Guinean franc.
	GNF,
	
	/// Guatemalan quetzal.
	GTQ,
	
	/// Guyanese dollar.
	GYD,
	
	/// Hong Kong dollar.
	HKD,
	
	/// Honduran lempira.
	HNL,
	
	/// Haitian gourde.
	HTG,
	
	/// Hungarian forint.
	HUF,
	
	/// Indonesian rupiah.
	IDR,
	
	/// Israeli new shekel.
	ILS,
	
	/// Indian rupee.
	INR,
	
	/// Iraqi dinar.
	IQD,
	
	/// Iranian rial.
	IRR,
	
	/// Icelandic króna.
	ISK,
	
	/// Jamaican dollar.
	JMD,
	
	/// Jordanian dinar.
	JOD,
	
	/// Japanese yen.
	JPY,
	
	/// Kenyan shilling.
	KES,
	
	/// Kyrgyzstani som.
	KGS,
	
	/// Cambodian riel.
	KHR,
	
	/// Comoro franc.
	KMF,
	
	/// North Korean won.
	KPW,
	
	/// South Korean won.
	KRW,
	
	/// Kuwaiti dinar.
	KWD,
	
	/// Cayman Islands dollar.
	KYD,
	
	/// Kazakhstani tenge.
	KZT,
	
	/// Lao kip.
	LAK,
	
	/// Lebanese pound.
	LBP,
	
	/// Sri Lankan rupee.
	LKR,
	
	/// Liberian dollar.
	LRD,
	
	/// Lesotho loti.
	LSL,
	
	/// Libyan dinar.
	LYD,
	
	/// Moroccan dirham.
	MAD,
	
	/// Moldovan leu.
	MDL,
	
	/// Malagasy ariary.
	MGA,
	
	/// Macedonian denar.
	MKD,
	
	/// Myanmar kyat.
	MMK,
	
	/// Mongolian tögrög.
	MNT,
	
	/// Macanese pataca.
	MOP,
	
	/// Mauritanian ouguiya.
	MRU,
	
	/// Mauritian rupee.
	MUR,
	
	/// Maldivian rufiyaa.
	MVR,
	
	/// Malawian kwacha.
	MWK,
	
	/// Mexican peso.
	MXN,
	
	/// Mexican Unidad de Inversion (UDI).
	MXV,
	
	/// Malaysian ringgit.
	MYR,
	
	/// Mozambican metical.
	MZN,
	
	/// Namibian dollar.
	NAD,
	
	/// Nigerian naira.
	NGN,
	
	/// Nicaraguan córdoba.
	NIO,
	
	/// Norwegian krone.
	NOK,
	
	/// Nepalese rupee.
	NPR,
	
	/// New Zealand dollar.
	NZD,
	
	/// Omani rial.
	OMR,
	
	/// Panamanian balboa.
	PAB,
	
	/// Peruvian sol.
	PEN,
	
	/// Papua New Guinean kina.
	PGK,
	
	/// Philippine peso.
	PHP,
	
	/// Pakistani rupee.
	PKR,
	
	/// Polish złoty.
	PLN,
	
	/// Paraguayan guaraní.
	PYG,
	
	/// Qatari riyal.
	QAR,
	
	/// Romanian leu.
	RON,
	
	/// Serbian dinar.
	RSD,
	
	/// Russian ruble.
	RUB,
	
	/// Rwandan franc.
	RWF,
	
	/// Saudi riyal.
	SAR,
	
	/// Solomon Islands dollar.
	SBD,
	
	/// Seychelles rupee.
	SCR,
	
	/// Sudanese pound.
	SDG,
	
	/// Swedish krona.
	SEK,
	
	/// Singapore dollar.
	SGD,
	
	/// Saint Helena pound.
	SHP,
	
	/// Sierra Leonean leone (new leone).
	SLE,
	
	/// Sierra Leonean leone (old leone).
	SLL,
	
	/// Somali shilling.
	SOS,
	
	/// Surinamese dollar.
	SRD,
	
	/// South Sudanese pound.
	SSP,
	
	/// São Tomé and Príncipe dobra.
	STN,
	
	/// Salvadoran colón.
	SVC,
	
	/// Syrian pound.
	SYP,
	
	/// Swazi lilangeni.
	SZL,
	
	/// Thai baht.
	THB,
	
	/// Tajikistani somoni.
	TJS,
	
	/// Turkmenistan manat.
	TMT,
	
	/// Tunisian dinar.
	TND,
	
	/// Tongan paʻanga.
	TOP,
	
	/// Turkish lira.
	TRY,
	
	/// Trinidad and Tobago dollar.
	TTD,
	
	/// New Taiwan dollar.
	TWD,
	
	/// Tanzanian shilling.
	TZS,
	
	/// Ukrainian hryvnia.
	UAH,
	
	/// Ugandan shilling.
	UGX,
	
	/// United States dollar.
	USD,
	
	/// United States dollar (next day).
	USN,
	
	/// Uruguay Peso en Unidades Indexadas (URUIURUI).
	UYI,
	
	/// Uruguayan peso.
	UYU,
	
	/// Unidad previsional.
	UYW,
	
	/// Uzbekistan sum.
	UZS,
	
	/// Venezuelan digital bolívar.
	VED,
	
	/// Venezuelan sovereign bolívar.
	VES,
	
	/// Vietnamese đồng.
	VND,
	
	/// Vanuatu vatu.
	VUV,
	
	/// Samoan tala.
	WST,
	
	/// CFA franc BEAC.
	XAF,
	
	/// Silver (one troy ounce).
	XAG,
	
	/// Gold (one troy ounce).
	XAU,
	
	/// European Composite Unit (EURCO).
	XBA,
	
	/// European Monetary Unit (E.M.U.-6).
	XBB,
	
	/// European Unit of Account 9 (E.U.A.-9).
	XBC,
	
	/// European Unit of Account 17 (E.U.A.-17).
	XBD,
	
	/// East Caribbean dollar.
	XCD,
	
	/// Special drawing rights.
	XDR,
	
	/// CFA franc BCEAO.
	XOF,
	
	/// Palladium (one troy ounce).
	XPD,
	
	/// CFP franc (franc Pacifique).
	XPF,
	
	/// Platinum (one troy ounce).
	XPT,
	
	/// SUCRE.
	XSU,
	
	/// Code reserved for testing.
	XTS,
	
	/// ADB Unit of Account.
	XUA,
	
	/// No currency.
	XXX,
	
	/// Yemeni rial.
	YER,
	
	/// South African rand.
	ZAR,
	
	/// Zambian kwacha.
	ZMW,
	
	/// Zimbabwean dollar (fifth).
	ZWL,
}

//󰭅		Currency																
impl Currency {
	//		all																	
	/// Returns all the currencies.
	pub fn all() -> Vec<Self> {
		CURRENCIES.keys().copied().collect()
	}
	
	//		info																
	/// Returns the `CurrencyInfo` instance corresponding to the `Currency`.
	/// 
	/// This method provides an easy way to get to the associated `CurrencyInfo`
	/// instance from a `Currency` enum variant.
	/// 
	#[must_use]
	fn info(self) -> &'static CurrencyInfo {
		#[expect(clippy::unwrap_used, reason = "Infallible")]
		//	This should be infallible. If it isn't, then the data is wrong, and one
		//	of the currencies is missing from the list, which is a bug.
		CURRENCIES.get(&self).unwrap()
	}
	
	//		name																
	/// Returns the name of the currency.
	#[cfg_attr(feature = "utoipa", expect(clippy::same_name_method, reason = "Doesn't matter"))]
	#[must_use]
	pub fn name(&self) -> &str {
		&self.info().name
	}
	
	//		code																
	/// Returns the currency code.
	#[must_use]
	pub fn code(&self) -> CurrencyCode {
		self.info().code
	}
	
	//		digits																
	/// Returns the number of digits after the decimal point.
	#[must_use]
	pub fn digits(&self) -> u8 {
		self.info().digits
	}
	
	//		countries															
	/// Returns the countries where the currency is used.
	#[must_use]
	pub fn countries(&self) -> &HashSet<CountryCode> {
		&self.info().countries
	}
}

//󰭅		AsStr																	
impl AsStr for Currency {
	//		as_str																
	fn as_str(&self) -> &str {
		&self.info().name
	}
}

//󰭅		Debug																	
impl Debug for Currency {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.info().code.as_str(), self.as_str())
	}
}

//󰭅		Display																	
impl Display for Currency {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

//󰭅		From<Currency> for String												
impl From<Currency> for String {
	//		from																
	fn from(currency: Currency) -> Self {
		currency.to_string()
	}
}

//󰭅		FromStr																	
impl FromStr for Currency {
	type Err = String;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		CURRENCIES
			.values()
			.find(|info| info.name == s)
			.map_or_else(
				||     Err(format!("Invalid Currency: {s}")),
				|info| Ok(info.code.currency())
			)
	}
}

//󰭅		TryFrom<String>															
impl TryFrom<String> for Currency {
	type Error = String;
	
	//		try_from															
	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.as_str().parse()
	}
}

//		CurrencyCode															
/// The possible currencies' codes.
/// 
/// These codes are based on the ISO 4217 standard, which defines codes of three
/// characters to represent currencies used by countries and territories. There
/// are both alphabetic and numeric codes, using either three letters or three
/// numbers.
/// 
/// # Alphabetic codes
/// 
/// The alphabetic codes are based on the ISO 3166-1 alpha-2 country codes, with
/// the first two letters of the ISO 4217 three-letter code being the same as
/// the code for the country name.
/// 
/// # Numeric codes
/// 
/// The three-digit numeric code is a useful alternative when the three-letter
/// code may not be appropriate. Interestingly, the three-digit numeric code is
/// the same as the numeric country code where possible.
/// 
/// # Data sources
/// 
/// The list of codes is available from [the ISO site](https://www.iso.org/iso-4217-currency-codes.html),
/// and from [Wikipedia](https://en.wikipedia.org/wiki/ISO_4217).
/// 
/// # See also
/// 
/// * [`Currency`]
/// 
#[expect(clippy::zero_prefixed_literal, reason = "Zeroes aid readability here")]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[repr(u16)]
#[serde(into = "String", try_from = "String")]
#[non_exhaustive]
pub enum CurrencyCode {
	/// United Arab Emirates dirham.
	AED = 784,
	
	/// Afghan afghani.
	AFN = 971,
	
	/// Albanian lek.
	ALL = 008,
	
	/// Armenian dram.
	AMD = 051,
	
	/// Netherlands Antillean guilder.
	ANG = 532,
	
	/// Angolan kwanza.
	AOA = 973,
	
	/// Argentine peso.
	ARS = 032,
	
	/// Australian dollar.
	AUD = 036,
	
	/// Aruban florin.
	AWG = 533,
	
	/// Azerbaijani manat.
	AZN = 944,
	
	/// Bosnia and Herzegovina convertible mark.
	BAM = 977,
	
	/// Barbados dollar.
	BBD = 052,
	
	/// Bangladeshi taka.
	BDT = 050,
	
	/// Bulgarian lev.
	BGN = 975,
	
	/// Bahraini dinar.
	BHD = 048,
	
	/// Burundian franc.
	BIF = 108,
	
	/// Bermudian dollar.
	BMD = 060,
	
	/// Brunei dollar.
	BND = 096,
	
	/// Boliviano.
	BOB = 068,
	
	/// Bolivian Mvdol.
	BOV = 984,
	
	/// Brazilian real.
	BRL = 986,
	
	/// Bahamian dollar.
	BSD = 044,
	
	/// Bhutanese ngultrum.
	BTN = 064,
	
	/// Botswana pula.
	BWP = 072,
	
	/// Belarusian ruble.
	BYN = 933,
	
	/// Belize dollar.
	BZD = 084,
	
	/// Canadian dollar.
	CAD = 124,
	
	/// Congolese franc.
	CDF = 976,
	
	/// WIR euro.
	CHE = 947,
	
	/// Swiss franc.
	CHF = 756,
	
	/// WIR franc.
	CHW = 948,
	
	/// Unidad de Fomento.
	CLF = 990,
	
	/// Chilean peso.
	CLP = 152,
	
	/// Renminbi.
	CNY = 156,
	
	/// Colombian peso.
	COP = 170,
	
	/// Unidad de Valor Real (UVR).
	COU = 970,
	
	/// Costa Rican colon.
	CRC = 188,
	
	/// Cuban peso.
	CUP = 192,
	
	/// Cape Verdean escudo.
	CVE = 132,
	
	/// Czech koruna.
	CZK = 203,
	
	/// Djiboutian franc.
	DJF = 262,
	
	/// Danish krone.
	DKK = 208,
	
	/// Dominican peso.
	DOP = 214,
	
	/// Algerian dinar.
	DZD = 012,
	
	/// Egyptian pound.
	EGP = 818,
	
	/// Eritrean nakfa.
	ERN = 232,
	
	/// Ethiopian birr.
	ETB = 230,
	
	/// Euro.
	EUR = 978,
	
	/// Fiji dollar.
	FJD = 242,
	
	/// Falkland Islands pound.
	FKP = 238,
	
	/// Pound sterling.
	GBP = 826,
	
	/// Georgian lari.
	GEL = 981,
	
	/// Ghanaian cedi.
	GHS = 936,
	
	/// Gibraltar pound.
	GIP = 292,
	
	/// Gambian dalasi.
	GMD = 270,
	
	/// Guinean franc.
	GNF = 324,
	
	/// Guatemalan quetzal.
	GTQ = 320,
	
	/// Guyanese dollar.
	GYD = 328,
	
	/// Hong Kong dollar.
	HKD = 344,
	
	/// Honduran lempira.
	HNL = 340,
	
	/// Haitian gourde.
	HTG = 332,
	
	/// Hungarian forint.
	HUF = 348,
	
	/// Indonesian rupiah.
	IDR = 360,
	
	/// Israeli new shekel.
	ILS = 376,
	
	/// Indian rupee.
	INR = 356,
	
	/// Iraqi dinar.
	IQD = 368,
	
	/// Iranian rial.
	IRR = 364,
	
	/// Icelandic króna.
	ISK = 352,
	
	/// Jamaican dollar.
	JMD = 388,
	
	/// Jordanian dinar.
	JOD = 400,
	
	/// Japanese yen.
	JPY = 392,
	
	/// Kenyan shilling.
	KES = 404,
	
	/// Kyrgyzstani som.
	KGS = 417,
	
	/// Cambodian riel.
	KHR = 116,
	
	/// Comoro franc.
	KMF = 174,
	
	/// North Korean won.
	KPW = 408,
	
	/// South Korean won.
	KRW = 410,
	
	/// Kuwaiti dinar.
	KWD = 414,
	
	/// Cayman Islands dollar.
	KYD = 136,
	
	/// Kazakhstani tenge.
	KZT = 398,
	
	/// Lao kip.
	LAK = 418,
	
	/// Lebanese pound.
	LBP = 422,
	
	/// Sri Lankan rupee.
	LKR = 144,
	
	/// Liberian dollar.
	LRD = 430,
	
	/// Lesotho loti.
	LSL = 426,
	
	/// Libyan dinar.
	LYD = 434,
	
	/// Moroccan dirham.
	MAD = 504,
	
	/// Moldovan leu.
	MDL = 498,
	
	/// Malagasy ariary.
	MGA = 969,
	
	/// Macedonian denar.
	MKD = 807,
	
	/// Myanmar kyat.
	MMK = 104,
	
	/// Mongolian tögrög.
	MNT = 496,
	
	/// Macanese pataca.
	MOP = 446,
	
	/// Mauritanian ouguiya.
	MRU = 929,
	
	/// Mauritian rupee.
	MUR = 480,
	
	/// Maldivian rufiyaa.
	MVR = 462,
	
	/// Malawian kwacha.
	MWK = 454,
	
	/// Mexican peso.
	MXN = 484,
	
	/// Mexican Unidad de Inversion (UDI).
	MXV = 979,
	
	/// Malaysian ringgit.
	MYR = 458,
	
	/// Mozambican metical.
	MZN = 943,
	
	/// Namibian dollar.
	NAD = 516,
	
	/// Nigerian naira.
	NGN = 566,
	
	/// Nicaraguan córdoba.
	NIO = 558,
	
	/// Norwegian krone.
	NOK = 578,
	
	/// Nepalese rupee.
	NPR = 524,
	
	/// New Zealand dollar.
	NZD = 554,
	
	/// Omani rial.
	OMR = 512,
	
	/// Panamanian balboa.
	PAB = 590,
	
	/// Peruvian sol.
	PEN = 604,
	
	/// Papua New Guinean kina.
	PGK = 598,
	
	/// Philippine peso.
	PHP = 608,
	
	/// Pakistani rupee.
	PKR = 586,
	
	/// Polish złoty.
	PLN = 985,
	
	/// Paraguayan guaraní.
	PYG = 600,
	
	/// Qatari riyal.
	QAR = 634,
	
	/// Romanian leu.
	RON = 946,
	
	/// Serbian dinar.
	RSD = 941,
	
	/// Russian ruble.
	RUB = 643,
	
	/// Rwandan franc.
	RWF = 646,
	
	/// Saudi riyal.
	SAR = 682,
	
	/// Solomon Islands dollar.
	SBD = 090,
	
	/// Seychelles rupee.
	SCR = 690,
	
	/// Sudanese pound.
	SDG = 938,
	
	/// Swedish krona.
	SEK = 752,
	
	/// Singapore dollar.
	SGD = 702,
	
	/// Saint Helena pound.
	SHP = 654,
	
	/// Sierra Leonean leone (new leone).
	SLE = 925,
	
	/// Sierra Leonean leone (old leone).
	SLL = 694,
	
	/// Somali shilling.
	SOS = 706,
	
	/// Surinamese dollar.
	SRD = 968,
	
	/// South Sudanese pound.
	SSP = 728,
	
	/// São Tomé and Príncipe dobra.
	STN = 930,
	
	/// Salvadoran colón.
	SVC = 222,
	
	/// Syrian pound.
	SYP = 760,
	
	/// Swazi lilangeni.
	SZL = 748,
	
	/// Thai baht.
	THB = 764,
	
	/// Tajikistani somoni.
	TJS = 972,
	
	/// Turkmenistan manat.
	TMT = 934,
	
	/// Tunisian dinar.
	TND = 788,
	
	/// Tongan paʻanga.
	TOP = 776,
	
	/// Turkish lira.
	TRY = 949,
	
	/// Trinidad and Tobago dollar.
	TTD = 780,
	
	/// New Taiwan dollar.
	TWD = 901,
	
	/// Tanzanian shilling.
	TZS = 834,
	
	/// Ukrainian hryvnia.
	UAH = 980,
	
	/// Ugandan shilling.
	UGX = 800,
	
	/// United States dollar.
	USD = 840,
	
	/// United States dollar (next day).
	USN = 997,
	
	/// Uruguay Peso en Unidades Indexadas (URUIURUI).
	UYI = 940,
	
	/// Uruguayan peso.
	UYU = 858,
	
	/// Unidad previsional.
	UYW = 927,
	
	/// Uzbekistan sum.
	UZS = 860,
	
	/// Venezuelan digital bolívar.
	VED = 926,
	
	/// Venezuelan sovereign bolívar.
	VES = 928,
	
	/// Vietnamese đồng.
	VND = 704,
	
	/// Vanuatu vatu.
	VUV = 548,
	
	/// Samoan tala.
	WST = 882,
	
	/// CFA franc BEAC.
	XAF = 950,
	
	/// Silver (one troy ounce).
	XAG = 961,
	
	/// Gold (one troy ounce).
	XAU = 959,
	
	/// European Composite Unit (EURCO).
	XBA = 955,
	
	/// European Monetary Unit (E.M.U.-6).
	XBB = 956,
	
	/// European Unit of Account 9 (E.U.A.-9).
	XBC = 957,
	
	/// European Unit of Account 17 (E.U.A.-17).
	XBD = 958,
	
	/// East Caribbean dollar.
	XCD = 951,
	
	/// Special drawing rights.
	XDR = 960,
	
	/// CFA franc BCEAO.
	XOF = 952,
	
	/// Palladium (one troy ounce).
	XPD = 964,
	
	/// CFP franc (franc Pacifique).
	XPF = 953,
	
	/// Platinum (one troy ounce).
	XPT = 962,
	
	/// SUCRE.
	XSU = 994,
	
	/// Code reserved for testing.
	XTS = 963,
	
	/// ADB Unit of Account.
	XUA = 965,
	
	/// No currency.
	XXX = 999,
	
	/// Yemeni rial.
	YER = 886,
	
	/// South African rand.
	ZAR = 710,
	
	/// Zambian kwacha.
	ZMW = 967,
	
	/// Zimbabwean dollar (fifth).
	ZWL = 932,
}

//󰭅		CurrencyCode															
impl CurrencyCode {
	//		all																	
	/// Returns all the currency codes.
	pub fn all() -> Vec<Self> {
		CURRENCIES.values().map(|info| info.code).collect()
	}
	
	//		currency															
	/// Returns the `Currency` variant corresponding to the `CurrencyCode`.
	/// 
	/// This method provides an easy way to get to the associated `Currency`
	/// variant from a `CurrencyCode` enum variant.
	/// 
	#[expect(clippy::too_many_lines, reason = "Data not logic")]
	#[must_use]
	pub const fn currency(&self) -> Currency {
		match *self {
			Self::AED => Currency::AED,
			Self::AFN => Currency::AFN,
			Self::ALL => Currency::ALL,
			Self::AMD => Currency::AMD,
			Self::ANG => Currency::ANG,
			Self::AOA => Currency::AOA,
			Self::ARS => Currency::ARS,
			Self::AUD => Currency::AUD,
			Self::AWG => Currency::AWG,
			Self::AZN => Currency::AZN,
			Self::BAM => Currency::BAM,
			Self::BBD => Currency::BBD,
			Self::BDT => Currency::BDT,
			Self::BGN => Currency::BGN,
			Self::BHD => Currency::BHD,
			Self::BIF => Currency::BIF,
			Self::BMD => Currency::BMD,
			Self::BND => Currency::BND,
			Self::BOB => Currency::BOB,
			Self::BOV => Currency::BOV,
			Self::BRL => Currency::BRL,
			Self::BSD => Currency::BSD,
			Self::BTN => Currency::BTN,
			Self::BWP => Currency::BWP,
			Self::BYN => Currency::BYN,
			Self::BZD => Currency::BZD,
			Self::CAD => Currency::CAD,
			Self::CDF => Currency::CDF,
			Self::CHE => Currency::CHE,
			Self::CHF => Currency::CHF,
			Self::CHW => Currency::CHW,
			Self::CLF => Currency::CLF,
			Self::CLP => Currency::CLP,
			Self::CNY => Currency::CNY,
			Self::COP => Currency::COP,
			Self::COU => Currency::COU,
			Self::CRC => Currency::CRC,
			Self::CUP => Currency::CUP,
			Self::CVE => Currency::CVE,
			Self::CZK => Currency::CZK,
			Self::DJF => Currency::DJF,
			Self::DKK => Currency::DKK,
			Self::DOP => Currency::DOP,
			Self::DZD => Currency::DZD,
			Self::EGP => Currency::EGP,
			Self::ERN => Currency::ERN,
			Self::ETB => Currency::ETB,
			Self::EUR => Currency::EUR,
			Self::FJD => Currency::FJD,
			Self::FKP => Currency::FKP,
			Self::GBP => Currency::GBP,
			Self::GEL => Currency::GEL,
			Self::GHS => Currency::GHS,
			Self::GIP => Currency::GIP,
			Self::GMD => Currency::GMD,
			Self::GNF => Currency::GNF,
			Self::GTQ => Currency::GTQ,
			Self::GYD => Currency::GYD,
			Self::HKD => Currency::HKD,
			Self::HNL => Currency::HNL,
			Self::HTG => Currency::HTG,
			Self::HUF => Currency::HUF,
			Self::IDR => Currency::IDR,
			Self::ILS => Currency::ILS,
			Self::INR => Currency::INR,
			Self::IQD => Currency::IQD,
			Self::IRR => Currency::IRR,
			Self::ISK => Currency::ISK,
			Self::JMD => Currency::JMD,
			Self::JOD => Currency::JOD,
			Self::JPY => Currency::JPY,
			Self::KES => Currency::KES,
			Self::KGS => Currency::KGS,
			Self::KHR => Currency::KHR,
			Self::KMF => Currency::KMF,
			Self::KPW => Currency::KPW,
			Self::KRW => Currency::KRW,
			Self::KWD => Currency::KWD,
			Self::KYD => Currency::KYD,
			Self::KZT => Currency::KZT,
			Self::LAK => Currency::LAK,
			Self::LBP => Currency::LBP,
			Self::LKR => Currency::LKR,
			Self::LRD => Currency::LRD,
			Self::LSL => Currency::LSL,
			Self::LYD => Currency::LYD,
			Self::MAD => Currency::MAD,
			Self::MDL => Currency::MDL,
			Self::MGA => Currency::MGA,
			Self::MKD => Currency::MKD,
			Self::MMK => Currency::MMK,
			Self::MNT => Currency::MNT,
			Self::MOP => Currency::MOP,
			Self::MRU => Currency::MRU,
			Self::MUR => Currency::MUR,
			Self::MVR => Currency::MVR,
			Self::MWK => Currency::MWK,
			Self::MXN => Currency::MXN,
			Self::MXV => Currency::MXV,
			Self::MYR => Currency::MYR,
			Self::MZN => Currency::MZN,
			Self::NAD => Currency::NAD,
			Self::NGN => Currency::NGN,
			Self::NIO => Currency::NIO,
			Self::NOK => Currency::NOK,
			Self::NPR => Currency::NPR,
			Self::NZD => Currency::NZD,
			Self::OMR => Currency::OMR,
			Self::PAB => Currency::PAB,
			Self::PEN => Currency::PEN,
			Self::PGK => Currency::PGK,
			Self::PHP => Currency::PHP,
			Self::PKR => Currency::PKR,
			Self::PLN => Currency::PLN,
			Self::PYG => Currency::PYG,
			Self::QAR => Currency::QAR,
			Self::RON => Currency::RON,
			Self::RSD => Currency::RSD,
			Self::RUB => Currency::RUB,
			Self::RWF => Currency::RWF,
			Self::SAR => Currency::SAR,
			Self::SBD => Currency::SBD,
			Self::SCR => Currency::SCR,
			Self::SDG => Currency::SDG,
			Self::SEK => Currency::SEK,
			Self::SGD => Currency::SGD,
			Self::SHP => Currency::SHP,
			Self::SLE => Currency::SLE,
			Self::SLL => Currency::SLL,
			Self::SOS => Currency::SOS,
			Self::SRD => Currency::SRD,
			Self::SSP => Currency::SSP,
			Self::STN => Currency::STN,
			Self::SVC => Currency::SVC,
			Self::SYP => Currency::SYP,
			Self::SZL => Currency::SZL,
			Self::THB => Currency::THB,
			Self::TJS => Currency::TJS,
			Self::TMT => Currency::TMT,
			Self::TND => Currency::TND,
			Self::TOP => Currency::TOP,
			Self::TRY => Currency::TRY,
			Self::TTD => Currency::TTD,
			Self::TWD => Currency::TWD,
			Self::TZS => Currency::TZS,
			Self::UAH => Currency::UAH,
			Self::UGX => Currency::UGX,
			Self::USD => Currency::USD,
			Self::USN => Currency::USN,
			Self::UYI => Currency::UYI,
			Self::UYU => Currency::UYU,
			Self::UYW => Currency::UYW,
			Self::UZS => Currency::UZS,
			Self::VED => Currency::VED,
			Self::VES => Currency::VES,
			Self::VND => Currency::VND,
			Self::VUV => Currency::VUV,
			Self::WST => Currency::WST,
			Self::XAF => Currency::XAF,
			Self::XAG => Currency::XAG,
			Self::XAU => Currency::XAU,
			Self::XBA => Currency::XBA,
			Self::XBB => Currency::XBB,
			Self::XBC => Currency::XBC,
			Self::XBD => Currency::XBD,
			Self::XCD => Currency::XCD,
			Self::XDR => Currency::XDR,
			Self::XOF => Currency::XOF,
			Self::XPD => Currency::XPD,
			Self::XPF => Currency::XPF,
			Self::XPT => Currency::XPT,
			Self::XSU => Currency::XSU,
			Self::XTS => Currency::XTS,
			Self::XUA => Currency::XUA,
			Self::XXX => Currency::XXX,
			Self::YER => Currency::YER,
			Self::ZAR => Currency::ZAR,
			Self::ZMW => Currency::ZMW,
			Self::ZWL => Currency::ZWL,
		}
	}
}

//󰭅		AsStr																	
impl AsStr for CurrencyCode {
	//		as_str																
	#[expect(clippy::too_many_lines, reason = "Data not logic")]
	fn as_str(&self) -> &'static str {
		match *self {
			Self::AED => "AED",
			Self::AFN => "AFN",
			Self::ALL => "ALL",
			Self::AMD => "AMD",
			Self::ANG => "ANG",
			Self::AOA => "AOA",
			Self::ARS => "ARS",
			Self::AUD => "AUD",
			Self::AWG => "AWG",
			Self::AZN => "AZN",
			Self::BAM => "BAM",
			Self::BBD => "BBD",
			Self::BDT => "BDT",
			Self::BGN => "BGN",
			Self::BHD => "BHD",
			Self::BIF => "BIF",
			Self::BMD => "BMD",
			Self::BND => "BND",
			Self::BOB => "BOB",
			Self::BOV => "BOV",
			Self::BRL => "BRL",
			Self::BSD => "BSD",
			Self::BTN => "BTN",
			Self::BWP => "BWP",
			Self::BYN => "BYN",
			Self::BZD => "BZD",
			Self::CAD => "CAD",
			Self::CDF => "CDF",
			Self::CHE => "CHE",
			Self::CHF => "CHF",
			Self::CHW => "CHW",
			Self::CLF => "CLF",
			Self::CLP => "CLP",
			Self::CNY => "CNY",
			Self::COP => "COP",
			Self::COU => "COU",
			Self::CRC => "CRC",
			Self::CUP => "CUP",
			Self::CVE => "CVE",
			Self::CZK => "CZK",
			Self::DJF => "DJF",
			Self::DKK => "DKK",
			Self::DOP => "DOP",
			Self::DZD => "DZD",
			Self::EGP => "EGP",
			Self::ERN => "ERN",
			Self::ETB => "ETB",
			Self::EUR => "EUR",
			Self::FJD => "FJD",
			Self::FKP => "FKP",
			Self::GBP => "GBP",
			Self::GEL => "GEL",
			Self::GHS => "GHS",
			Self::GIP => "GIP",
			Self::GMD => "GMD",
			Self::GNF => "GNF",
			Self::GTQ => "GTQ",
			Self::GYD => "GYD",
			Self::HKD => "HKD",
			Self::HNL => "HNL",
			Self::HTG => "HTG",
			Self::HUF => "HUF",
			Self::IDR => "IDR",
			Self::ILS => "ILS",
			Self::INR => "INR",
			Self::IQD => "IQD",
			Self::IRR => "IRR",
			Self::ISK => "ISK",
			Self::JMD => "JMD",
			Self::JOD => "JOD",
			Self::JPY => "JPY",
			Self::KES => "KES",
			Self::KGS => "KGS",
			Self::KHR => "KHR",
			Self::KMF => "KMF",
			Self::KPW => "KPW",
			Self::KRW => "KRW",
			Self::KWD => "KWD",
			Self::KYD => "KYD",
			Self::KZT => "KZT",
			Self::LAK => "LAK",
			Self::LBP => "LBP",
			Self::LKR => "LKR",
			Self::LRD => "LRD",
			Self::LSL => "LSL",
			Self::LYD => "LYD",
			Self::MAD => "MAD",
			Self::MDL => "MDL",
			Self::MGA => "MGA",
			Self::MKD => "MKD",
			Self::MMK => "MMK",
			Self::MNT => "MNT",
			Self::MOP => "MOP",
			Self::MRU => "MRU",
			Self::MUR => "MUR",
			Self::MVR => "MVR",
			Self::MWK => "MWK",
			Self::MXN => "MXN",
			Self::MXV => "MXV",
			Self::MYR => "MYR",
			Self::MZN => "MZN",
			Self::NAD => "NAD",
			Self::NGN => "NGN",
			Self::NIO => "NIO",
			Self::NOK => "NOK",
			Self::NPR => "NPR",
			Self::NZD => "NZD",
			Self::OMR => "OMR",
			Self::PAB => "PAB",
			Self::PEN => "PEN",
			Self::PGK => "PGK",
			Self::PHP => "PHP",
			Self::PKR => "PKR",
			Self::PLN => "PLN",
			Self::PYG => "PYG",
			Self::QAR => "QAR",
			Self::RON => "RON",
			Self::RSD => "RSD",
			Self::RUB => "RUB",
			Self::RWF => "RWF",
			Self::SAR => "SAR",
			Self::SBD => "SBD",
			Self::SCR => "SCR",
			Self::SDG => "SDG",
			Self::SEK => "SEK",
			Self::SGD => "SGD",
			Self::SHP => "SHP",
			Self::SLE => "SLE",
			Self::SLL => "SLL",
			Self::SOS => "SOS",
			Self::SRD => "SRD",
			Self::SSP => "SSP",
			Self::STN => "STN",
			Self::SVC => "SVC",
			Self::SYP => "SYP",
			Self::SZL => "SZL",
			Self::THB => "THB",
			Self::TJS => "TJS",
			Self::TMT => "TMT",
			Self::TND => "TND",
			Self::TOP => "TOP",
			Self::TRY => "TRY",
			Self::TTD => "TTD",
			Self::TWD => "TWD",
			Self::TZS => "TZS",
			Self::UAH => "UAH",
			Self::UGX => "UGX",
			Self::USD => "USD",
			Self::USN => "USN",
			Self::UYI => "UYI",
			Self::UYU => "UYU",
			Self::UYW => "UYW",
			Self::UZS => "UZS",
			Self::VED => "VED",
			Self::VES => "VES",
			Self::VND => "VND",
			Self::VUV => "VUV",
			Self::WST => "WST",
			Self::XAF => "XAF",
			Self::XAG => "XAG",
			Self::XAU => "XAU",
			Self::XBA => "XBA",
			Self::XBB => "XBB",
			Self::XBC => "XBC",
			Self::XBD => "XBD",
			Self::XCD => "XCD",
			Self::XDR => "XDR",
			Self::XOF => "XOF",
			Self::XPD => "XPD",
			Self::XPF => "XPF",
			Self::XPT => "XPT",
			Self::XSU => "XSU",
			Self::XTS => "XTS",
			Self::XUA => "XUA",
			Self::XXX => "XXX",
			Self::YER => "YER",
			Self::ZAR => "ZAR",
			Self::ZMW => "ZMW",
			Self::ZWL => "ZWL",
		}
	}
}

//󰭅		Display																	
impl Display for CurrencyCode {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

//󰭅		From<CurrencyCode> for u16												
impl From<CurrencyCode> for u16 {
	//		from																
	fn from(code: CurrencyCode) -> Self {
		code as Self
	}
}

//󰭅		From<CurrencyCode> for String											
impl From<CurrencyCode> for String {
	//		from																
	fn from(code: CurrencyCode) -> Self {
		code.to_string()
	}
}

//󰭅		FromStr																	
impl FromStr for CurrencyCode {
	type Err = String;
	
	//		from_str															
	#[expect(clippy::too_many_lines, reason = "Data not logic")]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			"AED" => Ok(Self::AED),
			"AFN" => Ok(Self::AFN),
			"ALL" => Ok(Self::ALL),
			"AMD" => Ok(Self::AMD),
			"ANG" => Ok(Self::ANG),
			"AOA" => Ok(Self::AOA),
			"ARS" => Ok(Self::ARS),
			"AUD" => Ok(Self::AUD),
			"AWG" => Ok(Self::AWG),
			"AZN" => Ok(Self::AZN),
			"BAM" => Ok(Self::BAM),
			"BBD" => Ok(Self::BBD),
			"BDT" => Ok(Self::BDT),
			"BGN" => Ok(Self::BGN),
			"BHD" => Ok(Self::BHD),
			"BIF" => Ok(Self::BIF),
			"BMD" => Ok(Self::BMD),
			"BND" => Ok(Self::BND),
			"BOB" => Ok(Self::BOB),
			"BOV" => Ok(Self::BOV),
			"BRL" => Ok(Self::BRL),
			"BSD" => Ok(Self::BSD),
			"BTN" => Ok(Self::BTN),
			"BWP" => Ok(Self::BWP),
			"BYN" => Ok(Self::BYN),
			"BZD" => Ok(Self::BZD),
			"CAD" => Ok(Self::CAD),
			"CDF" => Ok(Self::CDF),
			"CHE" => Ok(Self::CHE),
			"CHF" => Ok(Self::CHF),
			"CHW" => Ok(Self::CHW),
			"CLF" => Ok(Self::CLF),
			"CLP" => Ok(Self::CLP),
			"CNY" => Ok(Self::CNY),
			"COP" => Ok(Self::COP),
			"COU" => Ok(Self::COU),
			"CRC" => Ok(Self::CRC),
			"CUP" => Ok(Self::CUP),
			"CVE" => Ok(Self::CVE),
			"CZK" => Ok(Self::CZK),
			"DJF" => Ok(Self::DJF),
			"DKK" => Ok(Self::DKK),
			"DOP" => Ok(Self::DOP),
			"DZD" => Ok(Self::DZD),
			"EGP" => Ok(Self::EGP),
			"ERN" => Ok(Self::ERN),
			"ETB" => Ok(Self::ETB),
			"EUR" => Ok(Self::EUR),
			"FJD" => Ok(Self::FJD),
			"FKP" => Ok(Self::FKP),
			"GBP" => Ok(Self::GBP),
			"GEL" => Ok(Self::GEL),
			"GHS" => Ok(Self::GHS),
			"GIP" => Ok(Self::GIP),
			"GMD" => Ok(Self::GMD),
			"GNF" => Ok(Self::GNF),
			"GTQ" => Ok(Self::GTQ),
			"GYD" => Ok(Self::GYD),
			"HKD" => Ok(Self::HKD),
			"HNL" => Ok(Self::HNL),
			"HTG" => Ok(Self::HTG),
			"HUF" => Ok(Self::HUF),
			"IDR" => Ok(Self::IDR),
			"ILS" => Ok(Self::ILS),
			"INR" => Ok(Self::INR),
			"IQD" => Ok(Self::IQD),
			"IRR" => Ok(Self::IRR),
			"ISK" => Ok(Self::ISK),
			"JMD" => Ok(Self::JMD),
			"JOD" => Ok(Self::JOD),
			"JPY" => Ok(Self::JPY),
			"KES" => Ok(Self::KES),
			"KGS" => Ok(Self::KGS),
			"KHR" => Ok(Self::KHR),
			"KMF" => Ok(Self::KMF),
			"KPW" => Ok(Self::KPW),
			"KRW" => Ok(Self::KRW),
			"KWD" => Ok(Self::KWD),
			"KYD" => Ok(Self::KYD),
			"KZT" => Ok(Self::KZT),
			"LAK" => Ok(Self::LAK),
			"LBP" => Ok(Self::LBP),
			"LKR" => Ok(Self::LKR),
			"LRD" => Ok(Self::LRD),
			"LSL" => Ok(Self::LSL),
			"LYD" => Ok(Self::LYD),
			"MAD" => Ok(Self::MAD),
			"MDL" => Ok(Self::MDL),
			"MGA" => Ok(Self::MGA),
			"MKD" => Ok(Self::MKD),
			"MMK" => Ok(Self::MMK),
			"MNT" => Ok(Self::MNT),
			"MOP" => Ok(Self::MOP),
			"MRU" => Ok(Self::MRU),
			"MUR" => Ok(Self::MUR),
			"MVR" => Ok(Self::MVR),
			"MWK" => Ok(Self::MWK),
			"MXN" => Ok(Self::MXN),
			"MXV" => Ok(Self::MXV),
			"MYR" => Ok(Self::MYR),
			"MZN" => Ok(Self::MZN),
			"NAD" => Ok(Self::NAD),
			"NGN" => Ok(Self::NGN),
			"NIO" => Ok(Self::NIO),
			"NOK" => Ok(Self::NOK),
			"NPR" => Ok(Self::NPR),
			"NZD" => Ok(Self::NZD),
			"OMR" => Ok(Self::OMR),
			"PAB" => Ok(Self::PAB),
			"PEN" => Ok(Self::PEN),
			"PGK" => Ok(Self::PGK),
			"PHP" => Ok(Self::PHP),
			"PKR" => Ok(Self::PKR),
			"PLN" => Ok(Self::PLN),
			"PYG" => Ok(Self::PYG),
			"QAR" => Ok(Self::QAR),
			"RON" => Ok(Self::RON),
			"RSD" => Ok(Self::RSD),
			"RUB" => Ok(Self::RUB),
			"RWF" => Ok(Self::RWF),
			"SAR" => Ok(Self::SAR),
			"SBD" => Ok(Self::SBD),
			"SCR" => Ok(Self::SCR),
			"SDG" => Ok(Self::SDG),
			"SEK" => Ok(Self::SEK),
			"SGD" => Ok(Self::SGD),
			"SHP" => Ok(Self::SHP),
			"SLE" => Ok(Self::SLE),
			"SLL" => Ok(Self::SLL),
			"SOS" => Ok(Self::SOS),
			"SRD" => Ok(Self::SRD),
			"SSP" => Ok(Self::SSP),
			"STN" => Ok(Self::STN),
			"SVC" => Ok(Self::SVC),
			"SYP" => Ok(Self::SYP),
			"SZL" => Ok(Self::SZL),
			"THB" => Ok(Self::THB),
			"TJS" => Ok(Self::TJS),
			"TMT" => Ok(Self::TMT),
			"TND" => Ok(Self::TND),
			"TOP" => Ok(Self::TOP),
			"TRY" => Ok(Self::TRY),
			"TTD" => Ok(Self::TTD),
			"TWD" => Ok(Self::TWD),
			"TZS" => Ok(Self::TZS),
			"UAH" => Ok(Self::UAH),
			"UGX" => Ok(Self::UGX),
			"USD" => Ok(Self::USD),
			"USN" => Ok(Self::USN),
			"UYI" => Ok(Self::UYI),
			"UYU" => Ok(Self::UYU),
			"UYW" => Ok(Self::UYW),
			"UZS" => Ok(Self::UZS),
			"VED" => Ok(Self::VED),
			"VES" => Ok(Self::VES),
			"VND" => Ok(Self::VND),
			"VUV" => Ok(Self::VUV),
			"WST" => Ok(Self::WST),
			"XAF" => Ok(Self::XAF),
			"XAG" => Ok(Self::XAG),
			"XAU" => Ok(Self::XAU),
			"XBA" => Ok(Self::XBA),
			"XBB" => Ok(Self::XBB),
			"XBC" => Ok(Self::XBC),
			"XBD" => Ok(Self::XBD),
			"XCD" => Ok(Self::XCD),
			"XDR" => Ok(Self::XDR),
			"XOF" => Ok(Self::XOF),
			"XPD" => Ok(Self::XPD),
			"XPF" => Ok(Self::XPF),
			"XPT" => Ok(Self::XPT),
			"XSU" => Ok(Self::XSU),
			"XTS" => Ok(Self::XTS),
			"XUA" => Ok(Self::XUA),
			"XXX" => Ok(Self::XXX),
			"YER" => Ok(Self::YER),
			"ZAR" => Ok(Self::ZAR),
			"ZMW" => Ok(Self::ZMW),
			"ZWL" => Ok(Self::ZWL),
			_     => Err(format!("Invalid CurrencyCode: {s}")),
		}
	}
}

//󰭅		TryFrom<u16>															
#[expect(clippy::zero_prefixed_literal, reason = "Zeroes aid readability here")]
impl TryFrom<u16> for CurrencyCode {
	type Error = String;
	
	//		try_from															
	#[expect(clippy::too_many_lines, reason = "Data not logic")]
	fn try_from(value: u16) -> Result<Self, Self::Error> {
		match value {
			008 => Ok(Self::ALL),
			012 => Ok(Self::DZD),
			032 => Ok(Self::ARS),
			036 => Ok(Self::AUD),
			044 => Ok(Self::BSD),
			048 => Ok(Self::BHD),
			050 => Ok(Self::BDT),
			051 => Ok(Self::AMD),
			052 => Ok(Self::BBD),
			060 => Ok(Self::BMD),
			064 => Ok(Self::BTN),
			068 => Ok(Self::BOB),
			072 => Ok(Self::BWP),
			084 => Ok(Self::BZD),
			090 => Ok(Self::SBD),
			096 => Ok(Self::BND),
			104 => Ok(Self::MMK),
			108 => Ok(Self::BIF),
			116 => Ok(Self::KHR),
			124 => Ok(Self::CAD),
			132 => Ok(Self::CVE),
			136 => Ok(Self::KYD),
			144 => Ok(Self::LKR),
			152 => Ok(Self::CLP),
			156 => Ok(Self::CNY),
			170 => Ok(Self::COP),
			174 => Ok(Self::KMF),
			188 => Ok(Self::CRC),
			192 => Ok(Self::CUP),
			203 => Ok(Self::CZK),
			208 => Ok(Self::DKK),
			214 => Ok(Self::DOP),
			222 => Ok(Self::SVC),
			230 => Ok(Self::ETB),
			232 => Ok(Self::ERN),
			238 => Ok(Self::FKP),
			242 => Ok(Self::FJD),
			262 => Ok(Self::DJF),
			270 => Ok(Self::GMD),
			292 => Ok(Self::GIP),
			320 => Ok(Self::GTQ),
			324 => Ok(Self::GNF),
			328 => Ok(Self::GYD),
			332 => Ok(Self::HTG),
			340 => Ok(Self::HNL),
			344 => Ok(Self::HKD),
			348 => Ok(Self::HUF),
			352 => Ok(Self::ISK),
			356 => Ok(Self::INR),
			360 => Ok(Self::IDR),
			364 => Ok(Self::IRR),
			368 => Ok(Self::IQD),
			376 => Ok(Self::ILS),
			388 => Ok(Self::JMD),
			392 => Ok(Self::JPY),
			398 => Ok(Self::KZT),
			400 => Ok(Self::JOD),
			404 => Ok(Self::KES),
			408 => Ok(Self::KPW),
			410 => Ok(Self::KRW),
			414 => Ok(Self::KWD),
			417 => Ok(Self::KGS),
			418 => Ok(Self::LAK),
			422 => Ok(Self::LBP),
			426 => Ok(Self::LSL),
			430 => Ok(Self::LRD),
			434 => Ok(Self::LYD),
			446 => Ok(Self::MOP),
			454 => Ok(Self::MWK),
			458 => Ok(Self::MYR),
			462 => Ok(Self::MVR),
			480 => Ok(Self::MUR),
			484 => Ok(Self::MXN),
			496 => Ok(Self::MNT),
			498 => Ok(Self::MDL),
			504 => Ok(Self::MAD),
			512 => Ok(Self::OMR),
			516 => Ok(Self::NAD),
			524 => Ok(Self::NPR),
			532 => Ok(Self::ANG),
			533 => Ok(Self::AWG),
			548 => Ok(Self::VUV),
			554 => Ok(Self::NZD),
			558 => Ok(Self::NIO),
			566 => Ok(Self::NGN),
			578 => Ok(Self::NOK),
			586 => Ok(Self::PKR),
			590 => Ok(Self::PAB),
			598 => Ok(Self::PGK),
			600 => Ok(Self::PYG),
			604 => Ok(Self::PEN),
			608 => Ok(Self::PHP),
			634 => Ok(Self::QAR),
			643 => Ok(Self::RUB),
			646 => Ok(Self::RWF),
			654 => Ok(Self::SHP),
			682 => Ok(Self::SAR),
			690 => Ok(Self::SCR),
			694 => Ok(Self::SLL),
			702 => Ok(Self::SGD),
			704 => Ok(Self::VND),
			706 => Ok(Self::SOS),
			710 => Ok(Self::ZAR),
			728 => Ok(Self::SSP),
			748 => Ok(Self::SZL),
			752 => Ok(Self::SEK),
			756 => Ok(Self::CHF),
			760 => Ok(Self::SYP),
			764 => Ok(Self::THB),
			776 => Ok(Self::TOP),
			780 => Ok(Self::TTD),
			784 => Ok(Self::AED),
			788 => Ok(Self::TND),
			800 => Ok(Self::UGX),
			807 => Ok(Self::MKD),
			818 => Ok(Self::EGP),
			826 => Ok(Self::GBP),
			834 => Ok(Self::TZS),
			840 => Ok(Self::USD),
			858 => Ok(Self::UYU),
			860 => Ok(Self::UZS),
			882 => Ok(Self::WST),
			886 => Ok(Self::YER),
			901 => Ok(Self::TWD),
			925 => Ok(Self::SLE),
			926 => Ok(Self::VED),
			927 => Ok(Self::UYW),
			928 => Ok(Self::VES),
			929 => Ok(Self::MRU),
			930 => Ok(Self::STN),
			932 => Ok(Self::ZWL),
			933 => Ok(Self::BYN),
			934 => Ok(Self::TMT),
			936 => Ok(Self::GHS),
			938 => Ok(Self::SDG),
			940 => Ok(Self::UYI),
			941 => Ok(Self::RSD),
			943 => Ok(Self::MZN),
			944 => Ok(Self::AZN),
			946 => Ok(Self::RON),
			947 => Ok(Self::CHE),
			948 => Ok(Self::CHW),
			949 => Ok(Self::TRY),
			950 => Ok(Self::XAF),
			951 => Ok(Self::XCD),
			952 => Ok(Self::XOF),
			953 => Ok(Self::XPF),
			955 => Ok(Self::XBA),
			956 => Ok(Self::XBB),
			957 => Ok(Self::XBC),
			958 => Ok(Self::XBD),
			959 => Ok(Self::XAU),
			960 => Ok(Self::XDR),
			961 => Ok(Self::XAG),
			962 => Ok(Self::XPT),
			963 => Ok(Self::XTS),
			964 => Ok(Self::XPD),
			965 => Ok(Self::XUA),
			967 => Ok(Self::ZMW),
			968 => Ok(Self::SRD),
			969 => Ok(Self::MGA),
			970 => Ok(Self::COU),
			971 => Ok(Self::AFN),
			972 => Ok(Self::TJS),
			973 => Ok(Self::AOA),
			975 => Ok(Self::BGN),
			976 => Ok(Self::CDF),
			977 => Ok(Self::BAM),
			978 => Ok(Self::EUR),
			979 => Ok(Self::MXV),
			980 => Ok(Self::UAH),
			981 => Ok(Self::GEL),
			984 => Ok(Self::BOV),
			985 => Ok(Self::PLN),
			986 => Ok(Self::BRL),
			990 => Ok(Self::CLF),
			994 => Ok(Self::XSU),
			997 => Ok(Self::USN),
			999 => Ok(Self::XXX),
			_   => Err(format!("Invalid CurrencyCode: {value}")),
		}
	}
}

//󰭅		TryFrom<String>															
impl TryFrom<String> for CurrencyCode {
	type Error = String;
	
	//		try_from															
	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.as_str().parse()
	}
}



//		Structs

//		CurrencyInfo															
/// Currency information.
/// 
/// A currency has a number of properties, including a name, a currency code,
/// the number of digits after the decimal point, and the countries where the
/// currency is used.
/// 
/// Each currency is identified by a currency code, which can be expressed as
/// three letters or three numbers, as defined by the ISO 4217 standard.
/// 
/// # Data sources
/// 
/// The list of codes and other currency information is available from
/// [the ISO site](https://www.iso.org/iso-4217-currency-codes.html), and from
/// [Wikipedia](https://en.wikipedia.org/wiki/ISO_4217).
/// 
/// # See also
/// 
/// * [`Currency`]
/// * [`CurrencyCode`]
/// 
#[non_exhaustive]
struct CurrencyInfo {
	//		Private properties													
	/// The name of the currency.
	name:      String,
	
	/// The currency code. For more information, see [`CurrencyCode`].
	code:      CurrencyCode,
	
	/// The number of digits after the decimal point.
	digits:    u8,
	
	/// The countries where the currency is used.
	countries: HashSet<CountryCode>,
}


