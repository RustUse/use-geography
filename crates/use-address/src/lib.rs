#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn trimmed_non_empty(value: impl AsRef<str>) -> Option<String> {
    let trimmed = value.as_ref().trim();

    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn is_ascii_safe(value: &str) -> bool {
    value
        .chars()
        .all(|character| character.is_ascii() && !character.is_ascii_control())
}

macro_rules! impl_trimmed_text_type {
    ($type_name:ident, $error_name:ident, $empty_message:literal) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $error_name {
            Empty,
        }

        impl fmt::Display for $error_name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::Empty => formatter.write_str($empty_message),
                }
            }
        }

        impl Error for $error_name {}

        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $type_name(String);

        impl $type_name {
            /// Creates a value from non-empty text.
            ///
            /// # Errors
            ///
            #[doc = concat!("Returns [`", stringify!($error_name), "::Empty`] when the trimmed value is empty.")]
            pub fn new(value: impl AsRef<str>) -> Result<Self, $error_name> {
                trimmed_non_empty(value).ok_or($error_name::Empty).map(Self)
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<str> for $type_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $type_name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $type_name {
            type Err = $error_name;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = $error_name;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

impl_trimmed_text_type!(
    AddressLine,
    AddressLineError,
    "address line cannot be empty"
);
impl_trimmed_text_type!(
    AddressLine1,
    AddressLine1Error,
    "address line 1 cannot be empty"
);
impl_trimmed_text_type!(
    AddressLine2,
    AddressLine2Error,
    "address line 2 cannot be empty"
);
impl_trimmed_text_type!(
    StreetNumber,
    StreetNumberError,
    "street number cannot be empty"
);
impl_trimmed_text_type!(StreetName, StreetNameError, "street name cannot be empty");
impl_trimmed_text_type!(
    UnitDesignator,
    UnitDesignatorError,
    "unit designator cannot be empty"
);
impl_trimmed_text_type!(UnitNumber, UnitNumberError, "unit number cannot be empty");
impl_trimmed_text_type!(Locality, LocalityError, "locality cannot be empty");
impl_trimmed_text_type!(
    AdministrativeArea,
    AdministrativeAreaError,
    "administrative area cannot be empty"
);
impl_trimmed_text_type!(
    CountrySubdivision,
    CountrySubdivisionError,
    "country subdivision cannot be empty"
);

impl From<AddressLine1> for AddressLine {
    fn from(value: AddressLine1) -> Self {
        Self(value.0)
    }
}

impl From<AddressLine2> for AddressLine {
    fn from(value: AddressLine2) -> Self {
        Self(value.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PostalCodeError {
    Empty,
    InvalidCharacter,
}

impl fmt::Display for PostalCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("postal code cannot be empty"),
            Self::InvalidCharacter => {
                formatter.write_str("postal code must contain only ASCII-safe characters")
            },
        }
    }
}

impl Error for PostalCodeError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PostalCode(String);

impl PostalCode {
    /// Creates a postal code from non-empty ASCII-safe text.
    ///
    /// # Errors
    ///
    /// Returns [`PostalCodeError::Empty`] when the trimmed value is empty.
    /// Returns [`PostalCodeError::InvalidCharacter`] when the value contains
    /// non-ASCII or control characters.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PostalCodeError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            return Err(PostalCodeError::Empty);
        }

        if !is_ascii_safe(trimmed) {
            return Err(PostalCodeError::InvalidCharacter);
        }

        Ok(Self(trimmed.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PostalCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PostalCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PostalCode {
    type Err = PostalCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for PostalCode {
    type Error = PostalCodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressCountryCodeError {
    Empty,
    InvalidLength,
    InvalidCharacter,
}

impl fmt::Display for AddressCountryCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("address country code cannot be empty"),
            Self::InvalidLength => formatter
                .write_str("address country code must contain exactly two alphabetic characters"),
            Self::InvalidCharacter => formatter
                .write_str("address country code must contain only ASCII alphabetic characters"),
        }
    }
}

impl Error for AddressCountryCodeError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AddressCountryCode(String);

impl AddressCountryCode {
    /// Creates an uppercase alpha-2 address country code.
    ///
    /// # Errors
    ///
    /// Returns [`AddressCountryCodeError::Empty`] when the trimmed value is
    /// empty.
    /// Returns [`AddressCountryCodeError::InvalidLength`] when the value does
    /// not contain exactly two characters.
    /// Returns [`AddressCountryCodeError::InvalidCharacter`] when the value
    /// contains non-ASCII alphabetic characters.
    pub fn new(value: impl AsRef<str>) -> Result<Self, AddressCountryCodeError> {
        let trimmed = value.as_ref().trim();
        let character_count = trimmed.chars().count();

        if trimmed.is_empty() {
            return Err(AddressCountryCodeError::Empty);
        }

        if character_count != 2 {
            return Err(AddressCountryCodeError::InvalidLength);
        }

        if !trimmed
            .chars()
            .all(|character| character.is_ascii_alphabetic())
        {
            return Err(AddressCountryCodeError::InvalidCharacter);
        }

        Ok(Self(trimmed.to_ascii_uppercase()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AddressCountryCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AddressCountryCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AddressCountryCode {
    type Err = AddressCountryCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for AddressCountryCode {
    type Error = AddressCountryCodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AddressComponentKind {
    Line1,
    Line2,
    StreetNumber,
    StreetName,
    UnitDesignator,
    UnitNumber,
    Locality,
    AdministrativeArea,
    PostalCode,
    Country,
    CountrySubdivision,
    Region,
    Landmark,
    CareOf,
    Attention,
    Organization,
    Other,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AddressFormatHint {
    #[default]
    Unknown,
    UnitedStatesLike,
    CanadaLike,
    UnitedKingdomLike,
    EuropeanLike,
    JapanLike,
    International,
    Custom,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AddressValidationLevel {
    #[default]
    Unvalidated,
    SyntaxChecked,
    ComponentChecked,
    PostalChecked,
    Geocoded,
    Verified,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AddressKind {
    #[default]
    Physical,
    Mailing,
    Billing,
    Shipping,
    Legal,
    Registered,
    Service,
    Other,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AddressUsageKind {
    Residential,
    Commercial,
    Industrial,
    Government,
    Educational,
    Healthcare,
    Mixed,
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AddressPrecision {
    #[default]
    Unknown,
    Country,
    Region,
    Locality,
    PostalCode,
    Street,
    Building,
    Unit,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StreetAddress {
    pub number: StreetNumber,
    pub name: StreetName,
    pub unit_designator: Option<UnitDesignator>,
    pub unit_number: Option<UnitNumber>,
}

impl StreetAddress {
    #[must_use]
    pub const fn new(number: StreetNumber, name: StreetName) -> Self {
        Self {
            number,
            name,
            unit_designator: None,
            unit_number: None,
        }
    }

    #[must_use]
    pub fn with_unit(mut self, designator: UnitDesignator, number: UnitNumber) -> Self {
        self.unit_designator = Some(designator);
        self.unit_number = Some(number);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AddressError {
    Line(AddressLineError),
    PostalCode(PostalCodeError),
    Locality(LocalityError),
    AdministrativeArea(AdministrativeAreaError),
    CountryCode(AddressCountryCodeError),
}

impl fmt::Display for AddressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line(error) => error.fmt(formatter),
            Self::PostalCode(error) => error.fmt(formatter),
            Self::Locality(error) => error.fmt(formatter),
            Self::AdministrativeArea(error) => error.fmt(formatter),
            Self::CountryCode(error) => error.fmt(formatter),
        }
    }
}

impl Error for AddressError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Line(error) => Some(error),
            Self::PostalCode(error) => Some(error),
            Self::Locality(error) => Some(error),
            Self::AdministrativeArea(error) => Some(error),
            Self::CountryCode(error) => Some(error),
        }
    }
}

impl From<AddressLineError> for AddressError {
    fn from(value: AddressLineError) -> Self {
        Self::Line(value)
    }
}

impl From<PostalCodeError> for AddressError {
    fn from(value: PostalCodeError) -> Self {
        Self::PostalCode(value)
    }
}

impl From<LocalityError> for AddressError {
    fn from(value: LocalityError) -> Self {
        Self::Locality(value)
    }
}

impl From<AdministrativeAreaError> for AddressError {
    fn from(value: AdministrativeAreaError) -> Self {
        Self::AdministrativeArea(value)
    }
}

impl From<AddressCountryCodeError> for AddressError {
    fn from(value: AddressCountryCodeError) -> Self {
        Self::CountryCode(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Address {
    pub lines: Vec<AddressLine>,
    pub locality: Option<Locality>,
    pub administrative_area: Option<AdministrativeArea>,
    pub postal_code: Option<PostalCode>,
    pub country_code: Option<AddressCountryCode>,
    pub kind: AddressKind,
    pub usage: AddressUsageKind,
    pub format_hint: AddressFormatHint,
    pub validation_level: AddressValidationLevel,
    pub precision: AddressPrecision,
}

impl Address {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            lines: Vec::new(),
            locality: None,
            administrative_area: None,
            postal_code: None,
            country_code: None,
            kind: AddressKind::Physical,
            usage: AddressUsageKind::Unknown,
            format_hint: AddressFormatHint::Unknown,
            validation_level: AddressValidationLevel::Unvalidated,
            precision: AddressPrecision::Unknown,
        }
    }

    #[must_use]
    pub fn with_line(mut self, line: AddressLine) -> Self {
        self.lines.push(line);
        self
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.lines.is_empty()
            && self.locality.is_none()
            && self.administrative_area.is_none()
            && self.postal_code.is_none()
            && self.country_code.is_none()
    }

    #[must_use]
    pub const fn has_country(&self) -> bool {
        self.country_code.is_some()
    }

    #[must_use]
    pub const fn has_postal_code(&self) -> bool {
        self.postal_code.is_some()
    }

    #[must_use]
    pub const fn has_locality(&self) -> bool {
        self.locality.is_some()
    }

    #[must_use]
    pub fn component_count(&self) -> usize {
        self.lines.len()
            + usize::from(self.locality.is_some())
            + usize::from(self.administrative_area.is_some())
            + usize::from(self.postal_code.is_some())
            + usize::from(self.country_code.is_some())
    }
}

impl Default for Address {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Address, AddressCountryCode, AddressCountryCodeError, AddressError, AddressLine,
        AddressLine1, AddressLine1Error, AddressLine2, AddressLine2Error, AddressLineError,
        AdministrativeArea, AdministrativeAreaError, CountrySubdivision, CountrySubdivisionError,
        Locality, LocalityError, PostalCode, PostalCodeError, StreetAddress, StreetName,
        StreetNameError, StreetNumber, StreetNumberError, UnitDesignator, UnitDesignatorError,
        UnitNumber, UnitNumberError,
    };

    macro_rules! non_empty_text_tests {
        ($valid_name:ident, $invalid_name:ident, $type_name:ty, $error_name:ty, $input:literal, $expected:literal) => {
            #[test]
            fn $valid_name() -> Result<(), $error_name> {
                let value = <$type_name>::new($input)?;

                assert_eq!(value.as_str(), $expected);
                Ok(())
            }

            #[test]
            fn $invalid_name() {
                assert_eq!(<$type_name>::new("   "), Err(<$error_name>::Empty));
            }
        };
    }

    non_empty_text_tests!(
        address_line_accepts_trimmed_text,
        address_line_rejects_empty_text,
        AddressLine,
        AddressLineError,
        " 123 Main St ",
        "123 Main St"
    );
    non_empty_text_tests!(
        address_line1_accepts_trimmed_text,
        address_line1_rejects_empty_text,
        AddressLine1,
        AddressLine1Error,
        " 456 Oak Ave ",
        "456 Oak Ave"
    );
    non_empty_text_tests!(
        address_line2_accepts_trimmed_text,
        address_line2_rejects_empty_text,
        AddressLine2,
        AddressLine2Error,
        " Suite 9 ",
        "Suite 9"
    );
    non_empty_text_tests!(
        street_number_accepts_trimmed_text,
        street_number_rejects_empty_text,
        StreetNumber,
        StreetNumberError,
        " 221B ",
        "221B"
    );
    non_empty_text_tests!(
        street_name_accepts_trimmed_text,
        street_name_rejects_empty_text,
        StreetName,
        StreetNameError,
        " Baker Street ",
        "Baker Street"
    );
    non_empty_text_tests!(
        unit_designator_accepts_trimmed_text,
        unit_designator_rejects_empty_text,
        UnitDesignator,
        UnitDesignatorError,
        " Apt ",
        "Apt"
    );
    non_empty_text_tests!(
        unit_number_accepts_trimmed_text,
        unit_number_rejects_empty_text,
        UnitNumber,
        UnitNumberError,
        " 4B ",
        "4B"
    );
    non_empty_text_tests!(
        locality_accepts_trimmed_text,
        locality_rejects_empty_text,
        Locality,
        LocalityError,
        " London ",
        "London"
    );
    non_empty_text_tests!(
        administrative_area_accepts_trimmed_text,
        administrative_area_rejects_empty_text,
        AdministrativeArea,
        AdministrativeAreaError,
        " Indiana ",
        "Indiana"
    );
    non_empty_text_tests!(
        country_subdivision_accepts_trimmed_text,
        country_subdivision_rejects_empty_text,
        CountrySubdivision,
        CountrySubdivisionError,
        " Ontario ",
        "Ontario"
    );

    #[test]
    fn postal_code_accepts_numeric_zip() -> Result<(), PostalCodeError> {
        let postal_code = PostalCode::new("46802")?;

        assert_eq!(postal_code.as_str(), "46802");
        Ok(())
    }

    #[test]
    fn postal_code_accepts_alphanumeric_code() -> Result<(), PostalCodeError> {
        let postal_code = PostalCode::new("SW1A 1AA")?;

        assert_eq!(postal_code.as_str(), "SW1A 1AA");
        Ok(())
    }

    #[test]
    fn postal_code_rejects_empty_text() {
        assert_eq!(PostalCode::new("   "), Err(PostalCodeError::Empty));
    }

    #[test]
    fn postal_code_rejects_non_ascii_text() {
        assert_eq!(
            PostalCode::new("〒100-0001"),
            Err(PostalCodeError::InvalidCharacter)
        );
    }

    #[test]
    fn country_code_accepts_uppercase_alpha2() -> Result<(), AddressCountryCodeError> {
        let country_code = AddressCountryCode::new("US")?;

        assert_eq!(country_code.as_str(), "US");
        Ok(())
    }

    #[test]
    fn country_code_normalizes_lowercase_input() -> Result<(), AddressCountryCodeError> {
        let country_code = AddressCountryCode::new("us")?;

        assert_eq!(country_code.as_str(), "US");
        Ok(())
    }

    #[test]
    fn country_code_rejects_invalid_length() {
        assert_eq!(
            AddressCountryCode::new("USA"),
            Err(AddressCountryCodeError::InvalidLength)
        );
    }

    #[test]
    fn country_code_rejects_invalid_characters() {
        assert_eq!(
            AddressCountryCode::new("1A"),
            Err(AddressCountryCodeError::InvalidCharacter)
        );
    }

    #[test]
    fn country_code_rejects_empty_text() {
        assert_eq!(
            AddressCountryCode::new("   "),
            Err(AddressCountryCodeError::Empty)
        );
    }

    #[test]
    fn display_and_from_str_round_trip_for_address_line() -> Result<(), AddressLineError> {
        let line = "123 Main St".parse::<AddressLine>()?;

        assert_eq!(line.to_string(), "123 Main St");
        Ok(())
    }

    #[test]
    fn display_and_from_str_round_trip_for_country_code() -> Result<(), AddressCountryCodeError> {
        let country_code = "us".parse::<AddressCountryCode>()?;

        assert_eq!(country_code.to_string(), "US");
        Ok(())
    }

    #[test]
    fn try_from_supports_validated_newtypes() -> Result<(), AddressCountryCodeError> {
        let country_code = AddressCountryCode::try_from("ca")?;

        assert_eq!(country_code.as_str(), "CA");
        Ok(())
    }

    #[test]
    fn address_line_variants_convert_to_address_line() -> Result<(), AddressLine1Error> {
        let line = AddressLine::from(AddressLine1::new("Apartment Lobby")?);

        assert_eq!(line.as_str(), "Apartment Lobby");
        Ok(())
    }

    #[test]
    fn address_line2_converts_to_address_line() -> Result<(), AddressLine2Error> {
        let line = AddressLine::from(AddressLine2::new("Floor 3")?);

        assert_eq!(line.as_str(), "Floor 3");
        Ok(())
    }

    #[test]
    fn street_address_builder_sets_unit_fields() -> Result<(), Box<dyn std::error::Error>> {
        let street_address =
            StreetAddress::new(StreetNumber::new("221B")?, StreetName::new("Baker Street")?)
                .with_unit(UnitDesignator::new("Apt")?, UnitNumber::new("4B")?);

        assert_eq!(street_address.number.as_str(), "221B");
        assert_eq!(street_address.name.as_str(), "Baker Street");
        assert_eq!(
            street_address
                .unit_designator
                .as_ref()
                .map(UnitDesignator::as_str),
            Some("Apt")
        );
        assert_eq!(
            street_address.unit_number.as_ref().map(UnitNumber::as_str),
            Some("4B")
        );
        Ok(())
    }

    #[test]
    fn empty_address_is_empty() {
        assert!(Address::new().is_empty());
    }

    #[test]
    fn address_with_one_line_is_not_empty() -> Result<(), AddressLineError> {
        let address = Address::new().with_line(AddressLine::new("123 Main St")?);

        assert!(!address.is_empty());
        Ok(())
    }

    #[test]
    fn address_helpers_report_present_components() -> Result<(), Box<dyn std::error::Error>> {
        let address = Address {
            locality: Some(Locality::new("Fort Wayne")?),
            postal_code: Some(PostalCode::new("46802")?),
            country_code: Some(AddressCountryCode::new("us")?),
            ..Address::new().with_line(AddressLine::new("123 Main St")?)
        };

        assert!(address.has_locality());
        assert!(address.has_postal_code());
        assert!(address.has_country());
        Ok(())
    }

    #[test]
    fn address_component_count_tracks_structural_components()
    -> Result<(), Box<dyn std::error::Error>> {
        let address = Address {
            locality: Some(Locality::new("Fort Wayne")?),
            administrative_area: Some(AdministrativeArea::new("Indiana")?),
            postal_code: Some(PostalCode::new("46802")?),
            country_code: Some(AddressCountryCode::new("us")?),
            ..Address::new().with_line(AddressLine::new("123 Main St")?)
        };

        assert_eq!(address.component_count(), 5);
        Ok(())
    }

    #[test]
    fn address_error_wraps_component_errors() {
        let error = AddressError::from(AddressCountryCodeError::InvalidCharacter);

        assert_eq!(
            error.to_string(),
            "address country code must contain only ASCII alphabetic characters"
        );
    }
}
