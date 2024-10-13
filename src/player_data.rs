use std::error::Error;

pub struct DataIdentificationNumber(u16);

impl TryFrom<u16> for DataIdentificationNumber {
    type Error = Box<dyn Error>;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if 0 <= value && value <= 999 {
            return Ok(Self(value));
        } else {
            return Err(Box::from(
                "A data identification number must be a 3 digit code",
            ));
        }
    }
}

impl Into<u16> for DataIdentificationNumber {
    fn into(self: &Self) -> u16 {
        return self.0;
    }
}

impl TryFrom<String> for DataIdentificationNumber {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(str::parse::<u16>(value.as_str())?)
    }
}

impl Into<String> for DataIdentificationNumber {
    fn into(self: &Self) -> String {
        self.0.to_string()
    }
}

impl Field for DataIdentificationNumber {
    const SIZE: u8 = 3;
}

trait Line {
    fn get_data_identification_number(self) -> DataIdentificationNumber;

    fn get_fields(self) -> Vec<&'static dyn Field>;
}

impl Into<String> for Line {
    fn into(self: Self) -> String {}
}

trait Field: Into<String> + TryFrom<String> {
    fn get_size(self) -> u8;
}

pub struct StartingRankNumber(u16);

impl TryFrom<u16> for StartingRankNumber {
    type Error = Box<dyn Error>;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if 1 <= value && value <= 9999 {
            return Ok(Self(value));
        } else {
            return Err(Box::from(
                "A starting rank number must be between 1 and 9999",
            ));
        }
    }
}

impl Into<u16> for StartingRankNumber {
    fn into(self: Self) -> u16 {
        return self.0;
    }
}

enum Sex {
    Man,
    Woman,
}

impl Sex {
    const MAN: char = 'm';
    const WOMAN: char = 'w';
}

impl TryFrom<char> for Sex {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            Self::MAN => Ok(Self::Man),
            Self::WOMAN => Ok(Self::Woman),
            _ => Err(Box::from("Sex must be 'm' or 'w'")),
        }
    }
}

impl Into<char> for Sex {
    fn into(self: Self) -> char {
        match self {
            Self::Man => Self::MAN,
            Self::Woman => Self::WOMAN,
        }
    }
}

pub enum Title {
    GM,
    IM,
    WGM,
    FM,
    WIM,
    CM,
    WFM,
    WCM,
}

pub struct Name {
    first_name: String,
    last_name: String,
}

pub struct PlayerData {
    starting_rank_number: StartingRankNumber,
    sex: Option<Sex>,
    title: Option<Title>,
    fide_rating: Option<u16>,
    fide_federation: Option<String>, // 3 characters country code
}
