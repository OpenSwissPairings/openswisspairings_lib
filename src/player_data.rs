use std::error::Error;

trait Line {
    fn get_data_identification_number() -> DataIdentificationNumber;

    fn get_fields() -> Vec<Box<impl Field>>;
}

trait Field: TryFrom<String> + Into<String> {
    fn get_size() -> Option<u8>; // None means variable size but can only be at the end of a line
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
    fn get_size() -> Option<u8> {
        return Some(3);
    }
}

struct PlayerSection {}

impl Line for PlayerSection {
    fn get_data_identification_number() -> DataIdentificationNumber {
        return DataIdentificationNumber(3);
    }

    fn get_fields() -> Vec<Box<impl Field>> {
        let fields: Vec<Box<_>> = Vec::new();

        return fields;
    }
}
