//! Currency-related types.



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
	
	/// The countries where the currency is used.
	pub countries: HashSet<CountryCode>,
}


