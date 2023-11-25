//! Currency-related types.



//		Packages

use crate::country::CountryCode;
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
pub static CURRENCIES: Lazy<HashMap<CurrencyCode, Currency>> = Lazy::new(|| {
	hash_map!{
		CurrencyCode::AED: Currency { code: CurrencyCode::AED, name: s!("United Arab Emirates dirham"),                   digits: 2, countries: hash_set![] },
		CurrencyCode::AFN: Currency { code: CurrencyCode::AFN, name: s!("Afghan afghani"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::ALL: Currency { code: CurrencyCode::ALL, name: s!("Albanian lek"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::AMD: Currency { code: CurrencyCode::AMD, name: s!("Armenian dram"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::ANG: Currency { code: CurrencyCode::ANG, name: s!("Netherlands Antillean guilder"),                 digits: 2, countries: hash_set![] },
		CurrencyCode::AOA: Currency { code: CurrencyCode::AOA, name: s!("Angolan kwanza"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::ARS: Currency { code: CurrencyCode::ARS, name: s!("Argentine peso"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::AUD: Currency { code: CurrencyCode::AUD, name: s!("Australian dollar"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::AWG: Currency { code: CurrencyCode::AWG, name: s!("Aruban florin"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::AZN: Currency { code: CurrencyCode::AZN, name: s!("Azerbaijani manat"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::BAM: Currency { code: CurrencyCode::BAM, name: s!("Bosnia and Herzegovina convertible mark"),       digits: 2, countries: hash_set![] },
		CurrencyCode::BBD: Currency { code: CurrencyCode::BBD, name: s!("Barbados dollar"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::BDT: Currency { code: CurrencyCode::BDT, name: s!("Bangladeshi taka"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::BGN: Currency { code: CurrencyCode::BGN, name: s!("Bulgarian lev"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::BHD: Currency { code: CurrencyCode::BHD, name: s!("Bahraini dinar"),                                digits: 3, countries: hash_set![] },
		CurrencyCode::BIF: Currency { code: CurrencyCode::BIF, name: s!("Burundian franc"),                               digits: 0, countries: hash_set![] },
		CurrencyCode::BMD: Currency { code: CurrencyCode::BMD, name: s!("Bermudian dollar"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::BND: Currency { code: CurrencyCode::BND, name: s!("Brunei dollar"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::BOB: Currency { code: CurrencyCode::BOB, name: s!("Boliviano"),                                     digits: 2, countries: hash_set![] },
		CurrencyCode::BOV: Currency { code: CurrencyCode::BOV, name: s!("Bolivian Mvdol"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::BRL: Currency { code: CurrencyCode::BRL, name: s!("Brazilian real"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::BSD: Currency { code: CurrencyCode::BSD, name: s!("Bahamian dollar"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::BTN: Currency { code: CurrencyCode::BTN, name: s!("Bhutanese ngultrum"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::BWP: Currency { code: CurrencyCode::BWP, name: s!("Botswana pula"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::BYN: Currency { code: CurrencyCode::BYN, name: s!("Belarusian ruble"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::BZD: Currency { code: CurrencyCode::BZD, name: s!("Belize dollar"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::CAD: Currency { code: CurrencyCode::CAD, name: s!("Canadian dollar"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::CDF: Currency { code: CurrencyCode::CDF, name: s!("Congolese franc"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::CHE: Currency { code: CurrencyCode::CHE, name: s!("WIR euro"),                                      digits: 2, countries: hash_set![] },
		CurrencyCode::CHF: Currency { code: CurrencyCode::CHF, name: s!("Swiss franc"),                                   digits: 2, countries: hash_set![] },
		CurrencyCode::CHW: Currency { code: CurrencyCode::CHW, name: s!("WIR franc"),                                     digits: 2, countries: hash_set![] },
		CurrencyCode::CLF: Currency { code: CurrencyCode::CLF, name: s!("Unidad de Fomento"),                             digits: 4, countries: hash_set![] },
		CurrencyCode::CLP: Currency { code: CurrencyCode::CLP, name: s!("Chilean peso"),                                  digits: 0, countries: hash_set![] },
		CurrencyCode::CNY: Currency { code: CurrencyCode::CNY, name: s!("Renminbi"),                                      digits: 2, countries: hash_set![] },
		CurrencyCode::COP: Currency { code: CurrencyCode::COP, name: s!("Colombian peso"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::COU: Currency { code: CurrencyCode::COU, name: s!("Unidad de Valor Real (UVR)"),                    digits: 2, countries: hash_set![] },
		CurrencyCode::CRC: Currency { code: CurrencyCode::CRC, name: s!("Costa Rican colon"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::CUP: Currency { code: CurrencyCode::CUP, name: s!("Cuban peso"),                                    digits: 2, countries: hash_set![] },
		CurrencyCode::CVE: Currency { code: CurrencyCode::CVE, name: s!("Cape Verdean escudo"),                           digits: 2, countries: hash_set![] },
		CurrencyCode::CZK: Currency { code: CurrencyCode::CZK, name: s!("Czech koruna"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::DJF: Currency { code: CurrencyCode::DJF, name: s!("Djiboutian franc"),                              digits: 0, countries: hash_set![] },
		CurrencyCode::DKK: Currency { code: CurrencyCode::DKK, name: s!("Danish krone"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::DOP: Currency { code: CurrencyCode::DOP, name: s!("Dominican peso"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::DZD: Currency { code: CurrencyCode::DZD, name: s!("Algerian dinar"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::EGP: Currency { code: CurrencyCode::EGP, name: s!("Egyptian pound"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::ERN: Currency { code: CurrencyCode::ERN, name: s!("Eritrean nakfa"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::ETB: Currency { code: CurrencyCode::ETB, name: s!("Ethiopian birr"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::EUR: Currency { code: CurrencyCode::EUR, name: s!("Euro"),                                          digits: 2, countries: hash_set![] },
		CurrencyCode::FJD: Currency { code: CurrencyCode::FJD, name: s!("Fiji dollar"),                                   digits: 2, countries: hash_set![] },
		CurrencyCode::FKP: Currency { code: CurrencyCode::FKP, name: s!("Falkland Islands pound"),                        digits: 2, countries: hash_set![] },
		CurrencyCode::GBP: Currency { code: CurrencyCode::GBP, name: s!("Pound sterling"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::GEL: Currency { code: CurrencyCode::GEL, name: s!("Georgian lari"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::GHS: Currency { code: CurrencyCode::GHS, name: s!("Ghanaian cedi"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::GIP: Currency { code: CurrencyCode::GIP, name: s!("Gibraltar pound"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::GMD: Currency { code: CurrencyCode::GMD, name: s!("Gambian dalasi"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::GNF: Currency { code: CurrencyCode::GNF, name: s!("Guinean franc"),                                 digits: 0, countries: hash_set![] },
		CurrencyCode::GTQ: Currency { code: CurrencyCode::GTQ, name: s!("Guatemalan quetzal"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::GYD: Currency { code: CurrencyCode::GYD, name: s!("Guyanese dollar"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::HKD: Currency { code: CurrencyCode::HKD, name: s!("Hong Kong dollar"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::HNL: Currency { code: CurrencyCode::HNL, name: s!("Honduran lempira"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::HTG: Currency { code: CurrencyCode::HTG, name: s!("Haitian gourde"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::HUF: Currency { code: CurrencyCode::HUF, name: s!("Hungarian forint"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::IDR: Currency { code: CurrencyCode::IDR, name: s!("Indonesian rupiah"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::ILS: Currency { code: CurrencyCode::ILS, name: s!("Israeli new shekel"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::INR: Currency { code: CurrencyCode::INR, name: s!("Indian rupee"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::IQD: Currency { code: CurrencyCode::IQD, name: s!("Iraqi dinar"),                                   digits: 3, countries: hash_set![] },
		CurrencyCode::IRR: Currency { code: CurrencyCode::IRR, name: s!("Iranian rial"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::ISK: Currency { code: CurrencyCode::ISK, name: s!("Icelandic króna"),                               digits: 0, countries: hash_set![] },
		CurrencyCode::JMD: Currency { code: CurrencyCode::JMD, name: s!("Jamaican dollar"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::JOD: Currency { code: CurrencyCode::JOD, name: s!("Jordanian dinar"),                               digits: 3, countries: hash_set![] },
		CurrencyCode::JPY: Currency { code: CurrencyCode::JPY, name: s!("Japanese yen"),                                  digits: 0, countries: hash_set![] },
		CurrencyCode::KES: Currency { code: CurrencyCode::KES, name: s!("Kenyan shilling"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::KGS: Currency { code: CurrencyCode::KGS, name: s!("Kyrgyzstani som"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::KHR: Currency { code: CurrencyCode::KHR, name: s!("Cambodian riel"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::KMF: Currency { code: CurrencyCode::KMF, name: s!("Comoro franc"),                                  digits: 0, countries: hash_set![] },
		CurrencyCode::KPW: Currency { code: CurrencyCode::KPW, name: s!("North Korean won"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::KRW: Currency { code: CurrencyCode::KRW, name: s!("South Korean won"),                              digits: 0, countries: hash_set![] },
		CurrencyCode::KWD: Currency { code: CurrencyCode::KWD, name: s!("Kuwaiti dinar"),                                 digits: 3, countries: hash_set![] },
		CurrencyCode::KYD: Currency { code: CurrencyCode::KYD, name: s!("Cayman Islands dollar"),                         digits: 2, countries: hash_set![] },
		CurrencyCode::KZT: Currency { code: CurrencyCode::KZT, name: s!("Kazakhstani tenge"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::LAK: Currency { code: CurrencyCode::LAK, name: s!("Lao kip"),                                       digits: 2, countries: hash_set![] },
		CurrencyCode::LBP: Currency { code: CurrencyCode::LBP, name: s!("Lebanese pound"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::LKR: Currency { code: CurrencyCode::LKR, name: s!("Sri Lankan rupee"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::LRD: Currency { code: CurrencyCode::LRD, name: s!("Liberian dollar"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::LSL: Currency { code: CurrencyCode::LSL, name: s!("Lesotho loti"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::LYD: Currency { code: CurrencyCode::LYD, name: s!("Libyan dinar"),                                  digits: 3, countries: hash_set![] },
		CurrencyCode::MAD: Currency { code: CurrencyCode::MAD, name: s!("Moroccan dirham"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::MDL: Currency { code: CurrencyCode::MDL, name: s!("Moldovan leu"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::MGA: Currency { code: CurrencyCode::MGA, name: s!("Malagasy ariary"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::MKD: Currency { code: CurrencyCode::MKD, name: s!("Macedonian denar"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::MMK: Currency { code: CurrencyCode::MMK, name: s!("Myanmar kyat"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::MNT: Currency { code: CurrencyCode::MNT, name: s!("Mongolian tögrög"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::MOP: Currency { code: CurrencyCode::MOP, name: s!("Macanese pataca"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::MRU: Currency { code: CurrencyCode::MRU, name: s!("Mauritanian ouguiya"),                           digits: 2, countries: hash_set![] },
		CurrencyCode::MUR: Currency { code: CurrencyCode::MUR, name: s!("Mauritian rupee"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::MVR: Currency { code: CurrencyCode::MVR, name: s!("Maldivian rufiyaa"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::MWK: Currency { code: CurrencyCode::MWK, name: s!("Malawian kwacha"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::MXN: Currency { code: CurrencyCode::MXN, name: s!("Mexican peso"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::MXV: Currency { code: CurrencyCode::MXV, name: s!("Mexican Unidad de Inversion (UDI)"),             digits: 2, countries: hash_set![] },
		CurrencyCode::MYR: Currency { code: CurrencyCode::MYR, name: s!("Malaysian ringgit"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::MZN: Currency { code: CurrencyCode::MZN, name: s!("Mozambican metical"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::NAD: Currency { code: CurrencyCode::NAD, name: s!("Namibian dollar"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::NGN: Currency { code: CurrencyCode::NGN, name: s!("Nigerian naira"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::NIO: Currency { code: CurrencyCode::NIO, name: s!("Nicaraguan córdoba"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::NOK: Currency { code: CurrencyCode::NOK, name: s!("Norwegian krone"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::NPR: Currency { code: CurrencyCode::NPR, name: s!("Nepalese rupee"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::NZD: Currency { code: CurrencyCode::NZD, name: s!("New Zealand dollar"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::OMR: Currency { code: CurrencyCode::OMR, name: s!("Omani rial"),                                    digits: 3, countries: hash_set![] },
		CurrencyCode::PAB: Currency { code: CurrencyCode::PAB, name: s!("Panamanian balboa"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::PEN: Currency { code: CurrencyCode::PEN, name: s!("Peruvian sol"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::PGK: Currency { code: CurrencyCode::PGK, name: s!("Papua New Guinean kina"),                        digits: 2, countries: hash_set![] },
		CurrencyCode::PHP: Currency { code: CurrencyCode::PHP, name: s!("Philippine peso"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::PKR: Currency { code: CurrencyCode::PKR, name: s!("Pakistani rupee"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::PLN: Currency { code: CurrencyCode::PLN, name: s!("Polish złoty"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::PYG: Currency { code: CurrencyCode::PYG, name: s!("Paraguayan guaraní"),                            digits: 0, countries: hash_set![] },
		CurrencyCode::QAR: Currency { code: CurrencyCode::QAR, name: s!("Qatari riyal"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::RON: Currency { code: CurrencyCode::RON, name: s!("Romanian leu"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::RSD: Currency { code: CurrencyCode::RSD, name: s!("Serbian dinar"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::RUB: Currency { code: CurrencyCode::RUB, name: s!("Russian ruble"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::RWF: Currency { code: CurrencyCode::RWF, name: s!("Rwandan franc"),                                 digits: 0, countries: hash_set![] },
		CurrencyCode::SAR: Currency { code: CurrencyCode::SAR, name: s!("Saudi riyal"),                                   digits: 2, countries: hash_set![] },
		CurrencyCode::SBD: Currency { code: CurrencyCode::SBD, name: s!("Solomon Islands dollar"),                        digits: 2, countries: hash_set![] },
		CurrencyCode::SCR: Currency { code: CurrencyCode::SCR, name: s!("Seychelles rupee"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::SDG: Currency { code: CurrencyCode::SDG, name: s!("Sudanese pound"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::SEK: Currency { code: CurrencyCode::SEK, name: s!("Swedish krona"),                                 digits: 2, countries: hash_set![] },
		CurrencyCode::SGD: Currency { code: CurrencyCode::SGD, name: s!("Singapore dollar"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::SHP: Currency { code: CurrencyCode::SHP, name: s!("Saint Helena pound"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::SLE: Currency { code: CurrencyCode::SLE, name: s!("Sierra Leonean leone (new leone)"),              digits: 2, countries: hash_set![] },
		CurrencyCode::SLL: Currency { code: CurrencyCode::SLL, name: s!("Sierra Leonean leone (old leone)"),              digits: 2, countries: hash_set![] },
		CurrencyCode::SOS: Currency { code: CurrencyCode::SOS, name: s!("Somali shilling"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::SRD: Currency { code: CurrencyCode::SRD, name: s!("Surinamese dollar"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::SSP: Currency { code: CurrencyCode::SSP, name: s!("South Sudanese pound"),                          digits: 2, countries: hash_set![] },
		CurrencyCode::STN: Currency { code: CurrencyCode::STN, name: s!("São Tomé and Príncipe dobra"),                   digits: 2, countries: hash_set![] },
		CurrencyCode::SVC: Currency { code: CurrencyCode::SVC, name: s!("Salvadoran colón"),                              digits: 2, countries: hash_set![] },
		CurrencyCode::SYP: Currency { code: CurrencyCode::SYP, name: s!("Syrian pound"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::SZL: Currency { code: CurrencyCode::SZL, name: s!("Swazi lilangeni"),                               digits: 2, countries: hash_set![] },
		CurrencyCode::THB: Currency { code: CurrencyCode::THB, name: s!("Thai baht"),                                     digits: 2, countries: hash_set![] },
		CurrencyCode::TJS: Currency { code: CurrencyCode::TJS, name: s!("Tajikistani somoni"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::TMT: Currency { code: CurrencyCode::TMT, name: s!("Turkmenistan manat"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::TND: Currency { code: CurrencyCode::TND, name: s!("Tunisian dinar"),                                digits: 3, countries: hash_set![] },
		CurrencyCode::TOP: Currency { code: CurrencyCode::TOP, name: s!("Tongan paʻanga"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::TRY: Currency { code: CurrencyCode::TRY, name: s!("Turkish lira"),                                  digits: 2, countries: hash_set![] },
		CurrencyCode::TTD: Currency { code: CurrencyCode::TTD, name: s!("Trinidad and Tobago dollar"),                    digits: 2, countries: hash_set![] },
		CurrencyCode::TWD: Currency { code: CurrencyCode::TWD, name: s!("New Taiwan dollar"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::TZS: Currency { code: CurrencyCode::TZS, name: s!("Tanzanian shilling"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::UAH: Currency { code: CurrencyCode::UAH, name: s!("Ukrainian hryvnia"),                             digits: 2, countries: hash_set![] },
		CurrencyCode::UGX: Currency { code: CurrencyCode::UGX, name: s!("Ugandan shilling"),                              digits: 0, countries: hash_set![] },
		CurrencyCode::USD: Currency { code: CurrencyCode::USD, name: s!("United States dollar"),                          digits: 2, countries: hash_set![] },
		CurrencyCode::USN: Currency { code: CurrencyCode::USN, name: s!("United States dollar (next day)"),               digits: 2, countries: hash_set![] },
		CurrencyCode::UYI: Currency { code: CurrencyCode::UYI, name: s!("Uruguay Peso en Unidades Indexadas (URUIURUI)"), digits: 0, countries: hash_set![] },
		CurrencyCode::UYU: Currency { code: CurrencyCode::UYU, name: s!("Uruguayan peso"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::UYW: Currency { code: CurrencyCode::UYW, name: s!("Unidad previsional"),                            digits: 4, countries: hash_set![] },
		CurrencyCode::UZS: Currency { code: CurrencyCode::UZS, name: s!("Uzbekistan sum"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::VED: Currency { code: CurrencyCode::VED, name: s!("Venezuelan digital bolívar"),                    digits: 2, countries: hash_set![] },
		CurrencyCode::VES: Currency { code: CurrencyCode::VES, name: s!("Venezuelan sovereign bolívar"),                  digits: 2, countries: hash_set![] },
		CurrencyCode::VND: Currency { code: CurrencyCode::VND, name: s!("Vietnamese đồng"),                               digits: 0, countries: hash_set![] },
		CurrencyCode::VUV: Currency { code: CurrencyCode::VUV, name: s!("Vanuatu vatu"),                                  digits: 0, countries: hash_set![] },
		CurrencyCode::WST: Currency { code: CurrencyCode::WST, name: s!("Samoan tala"),                                   digits: 2, countries: hash_set![] },
		CurrencyCode::XAF: Currency { code: CurrencyCode::XAF, name: s!("CFA franc BEAC"),                                digits: 0, countries: hash_set![] },
		CurrencyCode::XAG: Currency { code: CurrencyCode::XAG, name: s!("Silver (one troy ounce)"),                       digits: 0, countries: hash_set![] },
		CurrencyCode::XAU: Currency { code: CurrencyCode::XAU, name: s!("Gold (one troy ounce)"),                         digits: 0, countries: hash_set![] },
		CurrencyCode::XBA: Currency { code: CurrencyCode::XBA, name: s!("European Composite Unit (EURCO)"),               digits: 0, countries: hash_set![] },
		CurrencyCode::XBB: Currency { code: CurrencyCode::XBB, name: s!("European Monetary Unit (E.M.U.-6)"),             digits: 0, countries: hash_set![] },
		CurrencyCode::XBC: Currency { code: CurrencyCode::XBC, name: s!("European Unit of Account 9 (E.U.A.-9)"),         digits: 0, countries: hash_set![] },
		CurrencyCode::XBD: Currency { code: CurrencyCode::XBD, name: s!("European Unit of Account 17 (E.U.A.-17)"),       digits: 0, countries: hash_set![] },
		CurrencyCode::XCD: Currency { code: CurrencyCode::XCD, name: s!("East Caribbean dollar"),                         digits: 2, countries: hash_set![] },
		CurrencyCode::XDR: Currency { code: CurrencyCode::XDR, name: s!("Special drawing rights"),                        digits: 0, countries: hash_set![] },
		CurrencyCode::XOF: Currency { code: CurrencyCode::XOF, name: s!("CFA franc BCEAO"),                               digits: 0, countries: hash_set![] },
		CurrencyCode::XPD: Currency { code: CurrencyCode::XPD, name: s!("Palladium (one troy ounce)"),                    digits: 0, countries: hash_set![] },
		CurrencyCode::XPF: Currency { code: CurrencyCode::XPF, name: s!("CFP franc (franc Pacifique)"),                   digits: 0, countries: hash_set![] },
		CurrencyCode::XPT: Currency { code: CurrencyCode::XPT, name: s!("Platinum (one troy ounce)"),                     digits: 0, countries: hash_set![] },
		CurrencyCode::XSU: Currency { code: CurrencyCode::XSU, name: s!("SUCRE"),                                         digits: 0, countries: hash_set![] },
		CurrencyCode::XTS: Currency { code: CurrencyCode::XTS, name: s!("Code reserved for testing"),                     digits: 0, countries: hash_set![] },
		CurrencyCode::XUA: Currency { code: CurrencyCode::XUA, name: s!("ADB Unit of Account"),                           digits: 0, countries: hash_set![] },
		CurrencyCode::XXX: Currency { code: CurrencyCode::XXX, name: s!("No currency"),                                   digits: 0, countries: hash_set![] },
		CurrencyCode::YER: Currency { code: CurrencyCode::YER, name: s!("Yemeni rial"),                                   digits: 2, countries: hash_set![] },
		CurrencyCode::ZAR: Currency { code: CurrencyCode::ZAR, name: s!("South African rand"),                            digits: 2, countries: hash_set![] },
		CurrencyCode::ZMW: Currency { code: CurrencyCode::ZMW, name: s!("Zambian kwacha"),                                digits: 2, countries: hash_set![] },
		CurrencyCode::ZWL: Currency { code: CurrencyCode::ZWL, name: s!("Zimbabwean dollar (fifth)"),                     digits: 2, countries: hash_set![] },
	}
});



//		Enums

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
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::zero_prefixed_literal)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ToSchema)]
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

impl CurrencyCode {
	//		as_str																
	/// Returns a string representation of the `CurrencyCode` variant.
	/// 
	/// This method provides a way to obtain a static string slice corresponding
	/// to a variant of the `CurrencyCode` enum. The returned string slice is
	/// suitable for use in scenarios where a string representation of the enum
	/// variant is needed, such as serialization or logging.
	/// 
	/// It is potentially different from the [`Display`] implementation, which
	/// returns a human-readable string representation of the enum variant, and
	/// the [`Debug`] implementation, which returns a string representation of
	/// the enum variant that is suitable for debugging purposes.
	/// 
	#[must_use]
	pub const fn as_str(&self) -> &'static str {
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

impl Display for CurrencyCode {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl From<CurrencyCode> for u16 {
	//		from																
	fn from(code: CurrencyCode) -> Self {
		code as Self
	}
}

impl From<CurrencyCode> for String {
	//		from																
	fn from(code: CurrencyCode) -> Self {
		code.to_string()
	}
}

impl FromStr for CurrencyCode {
	type Err = String;
	
	//		from_str															
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

#[allow(clippy::zero_prefixed_literal)]
impl TryFrom<u16> for CurrencyCode {
	type Error = String;
	
	//		try_from															
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

impl TryFrom<String> for CurrencyCode {
	type Error = String;
	
	//		try_from															
	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.as_str().parse()
	}
}



//		Structs

//		Currency																
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
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[non_exhaustive]
pub struct Currency {
	//		Public properties													
	/// The name of the currency.
	pub name:      String,
	
	/// The currency code. For more information, see [`CurrencyCode`].
	pub code:      CurrencyCode,
	
	/// The number of digits after the decimal point.
	pub digits:    u8,
	
	/// The countries where the currency is used.
	pub countries: HashSet<CountryCode>,
}


