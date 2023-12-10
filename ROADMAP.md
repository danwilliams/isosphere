# Isosphere roadmap

## Version 0.1.0

  - [x] Basics of country, currency, and language
      - [x] ISO 3166-1 countries with alpha2/alpha3/num codes and basic names
      - [x] ISO 4217 currencies with alpha3/num codes, names, and decimal places
      - [x] ISO 639-1 languages with alpha2/num codes and names
  - [x] Relationships between countries, currencies, and languages, according to
        ISO and Wikipedia
      - [x] Which countries officially use which currencies
      - [x] Which currencies and languages are officially used by which
            countries
      - [x] Which countries officially use which languages
  - [x] Basic implementations
      - [x] `AsStr`
      - [x] `Clone`
      - [x] `Copy`
      - [x] `Debug`
      - [x] `Deserialize`
      - [x] `Display`
      - [x] `Eq`
      - [x] `From`
      - [x] `FromStr`
      - [x] `Hash`
      - [x] `PartialEq`
      - [x] `Serialize`
      - [x] `ToSchema`
      - [x] `TryFrom`
  - [x] Basic methods for accessing, checking, and converting
      - [x] Property getters
      - [x] `CountryCode.country()`
      - [x] `CurrencyCode.currency()`
      - [x] `LanguageCode.language()`
      - [x] `::all()`
      - [x] `CountryCode.is_alpha2()` and `.is_alpha3()`
      - [x] `CountryCode.to_alpha2()` and `.to_alpha3()`
  - [x] Tests
  - [x] Documentation


## Version 0.2.0

  - [ ] Additional basic properties
      - [ ] Country
          - [ ] Independent
          - [ ] Disputed
      - [ ] Currency
          - [ ] Symbol
          - [ ] Category: currency or fund
          - [ ] Pegged to another currency
      - [ ] Language
          - [ ] Macrolanguage
  - [ ] Additional basic methods
      - [ ] `Currency.is_used_officially() `to denote whether it's (officially)
            used by any country
      - [ ] `Currency.is_pegged()` and/or `Currency.pegged_to()`/`pegged_by()`
      - [ ] `Language.is_used_officially()` to denote whether it's (officially)
            used by any country
  - [ ] Reduce lines of boilerplate code


## Version 0.3.0

  - [ ] Extended country information
      - [ ] ISO 3166-2 subdivisions
      - [ ] Regions
      - [ ] Additional names - full name vs short name


## Version 0.4.0

  - [ ] Extended language information
      - [ ] ISO 639-2/T and 639-2/B codes
      - [ ] ISO 639-3 and 639-5 codes
  - [ ] Extended language-related country information
      - [ ] Regional languages for countries
      - [ ] Minority languages for countries


## Version 0.5.0

  - [ ] Extended currency information
      - [ ] Non-fiat, e.g. blockchain
  - [ ] Extended currency-related country information
      - [ ] Differentiation between official ISO-recognised currencies and those
            that are widely used/accepted/circulated, but not ISO
  - [ ] Methods
      - [ ] `Country.iso_currencies()`
      - [ ] `Currency.is_fiat()`


## Version 0.6.0

  - [ ] Timezones
      - [ ] Timezone list
      - [ ] `Timezones.countries`
  - [ ] Telephone calling codes
      - [ ] ITU-T E.164 calling code list
      - [ ] `CallingCode.country`
  - [ ] Internet TLDs
      - [ ] TLD list
      - [ ] `TLD.country`
  - [ ] Additional country information
      - [ ] `Country.capital`
      - [ ] `Country.timezones`
      - [ ] `Country.calling_code`
      - [ ] `Country.tld`


## Version 0.7.0

  - [ ] Historic information
      - [ ] Countries - ISO 3166-3 historic list
      - [ ] Currencies - historic list from ISO 4217
  - [ ] Methods
      - [ ] `Currency.is_active()`/`.is_historic()`


## Version 0.8.0

  - [ ] Translations
      - [ ] Translations of names into other languages


## Version 0.9.0

  - [ ] Feature flags
      - [ ] `country`: Basic country information
      - [ ] `currency`: Basic currency information
      - [ ] `language`: Basic language information
      - [ ] `tlds`: Internet TLD information
      - [ ] `timezones`: Timezone information
      - [ ] `calling-codes`: Telephone calling code information
      - [ ] `country-iso3166-2`: Subdivision information
      - [ ] `language-iso639-2`: Extended language information
      - [ ] `historic`: Historic information for countries/currencies/languages
      - [ ] `lang-X`: Translations for language code
  - [ ] Performance
      - [ ] Examine crate dependency tree
      - [ ] Assess possibility of adopting `no-std`


