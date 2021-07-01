use nom::error::VerboseError;

#[derive(Debug, PartialEq)]
pub enum WqlError<'b> {
    Plain(String),
    UuidParse(uuid::Error),
    Parse(nom::Err<VerboseError<&'b str>>),
}

impl<'b> From<nom::Err<VerboseError<&'b str>>> for WqlError<'b> {
    fn from(e: nom::Err<VerboseError<&'b str>>) -> Self {
        WqlError::Parse(e)
    }
}

impl<'b> From<uuid::Error> for WqlError<'b> {
    fn from(e: uuid::Error) -> Self {
        WqlError::UuidParse(e)
    }
}
