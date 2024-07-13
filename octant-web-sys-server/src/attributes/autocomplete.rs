use marshal::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub enum AutocompleteAddressType {
    Shipping,
    Billing,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AutocompleteContactKind {
    Home,
    Work,
    Mobile,
    Fax,
    Pager,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AutocompleteContactField {
    Tel,
    TelCountryCode,
    TelNational,
    TelAreaCode,
    TelLocal,
    TelLocalPrefix,
    TelLocalSuffix,
    TelExtension,
    Email,
    Impp,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AutocompleteField {
    Name,
    HonorificPrefix,
    GivenName,
    AdditionalName,
    FamilyName,
    HonorificSuffix,
    Nickname,
    Username,
    NewPassword,
    CurrentPassword,
    OneTimeCode,
    OrganizationTitle,
    Organization,
    StreetAddress,
    AddressLine1,
    AddressLine2,
    AddressLine3,
    AddressLevel4,
    AddressLevel3,
    AddressLevel2,
    AddressLevel1,
    Country,
    CountryName,
    PostalCode,
    CcName,
    CcGivenName,
    CcAdditionalName,
    CcFamilyName,
    CcNumber,
    CcExp,
    CcExpMonth,
    CcExpYear,
    CcCsc,
    CcType,
    TransactionCurrency,
    TransactionAmount,
    Language,
    Bday,
    BdayDay,
    BdayMonth,
    BdayYear,
    Sex,
    Url,
    Photo,
    Contact {
        kind: Option<AutocompleteContactKind>,
        field: AutocompleteContactField,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Autocomplete {
    Off,
    On,
    Tokens {
        section: Option<String>,
        address_type: Option<AutocompleteAddressType>,
        field: AutocompleteField,
        webauthn: bool,
    },
}

impl Display for AutocompleteAddressType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AutocompleteAddressType::Shipping => write!(f, "shipping"),
            AutocompleteAddressType::Billing => write!(f, "billing"),
        }
    }
}

impl Display for AutocompleteField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AutocompleteField::Name => write!(f, "name")?,
            AutocompleteField::HonorificPrefix => write!(f, "honorific-prefix")?,
            AutocompleteField::GivenName => write!(f, "given-name")?,
            AutocompleteField::AdditionalName => write!(f, "additional-name")?,
            AutocompleteField::FamilyName => write!(f, "family-name")?,
            AutocompleteField::HonorificSuffix => write!(f, "name")?,
            AutocompleteField::Nickname => write!(f, "nickname")?,
            AutocompleteField::Username => write!(f, "username")?,
            AutocompleteField::NewPassword => write!(f, "password")?,
            AutocompleteField::CurrentPassword => write!(f, "current-password")?,
            AutocompleteField::OneTimeCode => write!(f, "one-time-code")?,
            AutocompleteField::OrganizationTitle => write!(f, "organization-title")?,
            AutocompleteField::Organization => write!(f, "organization")?,
            AutocompleteField::StreetAddress => write!(f, "street-address")?,
            AutocompleteField::AddressLine1 => write!(f, "address-line1")?,
            AutocompleteField::AddressLine2 => write!(f, "address-line2")?,
            AutocompleteField::AddressLine3 => write!(f, "address-line3")?,
            AutocompleteField::AddressLevel4 => write!(f, "address-level4")?,
            AutocompleteField::AddressLevel3 => write!(f, "address-level3")?,
            AutocompleteField::AddressLevel2 => write!(f, "address-level2")?,
            AutocompleteField::AddressLevel1 => write!(f, "address-level1")?,
            AutocompleteField::Country => write!(f, "country")?,
            AutocompleteField::CountryName => write!(f, "country-name")?,
            AutocompleteField::PostalCode => write!(f, "postal-code")?,
            AutocompleteField::CcName => write!(f, "cc-name")?,
            AutocompleteField::CcGivenName => write!(f, "cc-given-name")?,
            AutocompleteField::CcAdditionalName => write!(f, "cc-additional-name")?,
            AutocompleteField::CcFamilyName => write!(f, "cc-family-name")?,
            AutocompleteField::CcNumber => write!(f, "cc-number")?,
            AutocompleteField::CcExp => write!(f, "cc-exp")?,
            AutocompleteField::CcExpMonth => write!(f, "cc-exp-month")?,
            AutocompleteField::CcExpYear => write!(f, "cc-exp-year")?,
            AutocompleteField::CcCsc => write!(f, "cc-csc")?,
            AutocompleteField::CcType => write!(f, "cc-type")?,
            AutocompleteField::TransactionCurrency => write!(f, "transaction-currency")?,
            AutocompleteField::TransactionAmount => write!(f, "transaction-amount")?,
            AutocompleteField::Language => write!(f, "language")?,
            AutocompleteField::Bday => write!(f, "bday")?,
            AutocompleteField::BdayDay => write!(f, "bday-day")?,
            AutocompleteField::BdayMonth => write!(f, "bday-month")?,
            AutocompleteField::BdayYear => write!(f, "bday-year")?,
            AutocompleteField::Sex => write!(f, "sex")?,
            AutocompleteField::Url => write!(f, "url")?,
            AutocompleteField::Photo => write!(f, "photo")?,
            AutocompleteField::Contact { .. } => write!(f, "contact")?,
        }
        Ok(())
    }
}

impl Display for Autocomplete {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Autocomplete::Off => write!(f, "off")?,
            Autocomplete::On => write!(f, "on")?,
            Autocomplete::Tokens {
                section,
                address_type,
                field,
                webauthn,
            } => {
                if let Some(section) = section {
                    write!(f, "{} ", section)?;
                }
                if let Some(address_type) = address_type {
                    write!(f, "{} ", address_type)?;
                }
                write!(f, "{}", field)?;
                if *webauthn {
                    write!(f, " webauthn")?;
                }
            }
        }
        Ok(())
    }
}

