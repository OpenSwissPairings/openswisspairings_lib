use std::error::Error;

trait Line: Sized {
    fn get_data_identification_number() -> DataIdentificationNumber;

    fn get_fields() -> Vec<FieldEnum>;
}

impl Into<String> for Line {
    fn into(self) -> String {
        let mut output: String = Line::get_data_identification_number().into();

        return output;
    }
}

trait Field: Into<String> + TryFrom<String> {
    const SIZE: Option<u8> = None;
}

struct DataIdentificationNumber(u16);

impl TryFrom<String> for DataIdentificationNumber {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed: u16 = value.parse::<u16>()?;

        if parsed >= 0 && parsed <= 999 {
            Ok(Self(parsed))
        } else {
            Err(Box::from(
                "Data identification number must be a number between 0 and 999",
            ))
        }
    }
}

impl Into<String> for DataIdentificationNumber {
    fn into(self) -> String {
        format!("{:0>3}", self.0.to_string())
    }
}

impl Field for DataIdentificationNumber {
    const SIZE: Option<u8> = Some(3);
}

struct StartingRankNumber(u16);

impl TryFrom<String> for StartingRankNumber {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed: u16 = value.parse::<u16>()?;

        if parsed >= 1 && parsed <= 9999 {
            Ok(Self(parsed))
        } else {
            Err(Box::from(
                "Starting rank must be a number between 1 and 9999",
            ))
        }
    }
}

impl Into<String> for StartingRankNumber {
    fn into(self) -> String {
        format!("{:0>4}", self.0.to_string())
    }
}

impl Field for StartingRankNumber {
    const SIZE: Option<u8> = Some(4);
}

struct PlayerSection {
    starting_rank: StartingRankNumber,
}

enum FieldEnum {
    StartingRank(StartingRankNumber),
}

impl Line for PlayerSection {
    fn get_data_identification_number() -> DataIdentificationNumber {
        return DataIdentificationNumber(3);
    }

    fn get_fields() -> Vec<FieldEnum> {
        let fields: Vec<FieldEnum> = vec![FieldEnum::StartingRank(StartingRankNumber(12))];

        return fields;
    }
}
