# Isosphere

![Rust](https://img.shields.io/badge/Rust-1.81%2B-b7410e?style=flat&logo=rust&logoColor=white&labelColor=b7410e)
[![Crate version](https://img.shields.io/crates/v/isosphere?style=flat)](https://crates.io/crates/isosphere)
[![CI](https://img.shields.io/github/actions/workflow/status/danwilliams/isosphere/ci.yml?style=flat&logo=github&logoColor=white&label=build%2Ftest)](https://github.com/danwilliams/isosphere/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/docsrs/isosphere?style=flat&logo=docs.rs&logoColor=white)](https://docs.rs/crate/isosphere/latest)
![License](https://img.shields.io/github/license/danwilliams/isosphere?style=flat)

The Isosphere crate is a library of ISO standard data types, helpers, and
related utilities for Rust projects.

This crate was created with the intention of being a comprehensive, accurate,
and fully-featured source of ISO and similar standard data in intuitive and
easy-to-use form. Although there are a number of other crates already in
existence that perform a similar role, they were all found to be lacking in some
way: either features, approach, accuracy, or consistency. Additionally, those
crates that do exist are isolated, do not interact with each other, and for the
most part are not maintained. This crate aims to address all of these issues.

The name "Isosphere" is a portmanteau of "ISO" and "sphere", and is intended to
convey the idea of an all-encompassing sphere of ISO data. As in, "everything in
the ISO sphere".

There is a [roadmap for the project](ROADMAP.md), which sets out the planned
releases and their associated functionality, and indicates current status
according to the intended goals.

There is full support for (de)serialisation via [Serde](https://crates.io/crates/serde),
and OpenAPI via [Utoipa](https://crates.io/crates/utoipa).

Currently, the following modules are provided:

  - [`country`](#country)
  - [`currency`](#currency)
  - [`language`](#language)


## country

The [`country`](https://docs.rs/isosphere/latest/isosphere/country/index.html)
module provides ISO 3166-1 countries with alpha2/alpha3/numeric codes and basic
names. The countries are related to the currencies and languages that are
officially used by them, according to the ISO and Wikipedia.

  - [`Country`](https://docs.rs/isosphere/latest/isosphere/country/enum.Country.html) -
    This enum represents a country, and provides access to its properties. It is
    the central means of interaction with country data.

  - [`CountryCode`](https://docs.rs/isosphere/latest/isosphere/country/enum.CountryCode.html) -
    This enum represents a country code, in alpha2/alpha3/numeric form, suitable
    for (de)serialisation.


## currency

The [`currency`](https://docs.rs/isosphere/latest/isosphere/currency/index.html)
module provides ISO 4217 currencies with alpha3/numeric codes and basic names.
The currencies are related to the countries that officially use them, according
to the ISO and Wikipedia.

  - [`Currency`](https://docs.rs/isosphere/latest/isosphere/currency/enum.Currency.html) -
    This enum represents a currency, and provides access to its properties. It
    is the central means of interaction with currency data.

  - [`CurrencyCode`](https://docs.rs/isosphere/latest/isosphere/currency/enum.CurrencyCode.html) -
    This enum represents a currency code, in alpha3/numeric form, suitable for
    (de)serialisation.


## language

The [`language`](https://docs.rs/isosphere/latest/isosphere/language/index.html)
module provides ISO 639-1 languages with alpha2 codes and basic names. The
languages are related to the countries that officially use them, according to
Wikipedia (the ISO does not provide this information).

  - [`Language`](https://docs.rs/isosphere/latest/isosphere/language/enum.Language.html) -
    This enum represents a language, and provides access to its properties. It
    is the central means of interaction with language data.

  - [`LanguageCode`](https://docs.rs/isosphere/latest/isosphere/language/enum.LanguageCode.html) -
    This enum represents a language code, in alpha2 form, suitable for
    (de)serialisation.


