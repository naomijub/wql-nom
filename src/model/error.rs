use nom::error::VerboseError;

#[derive(Debug, PartialEq)]
pub enum WqlError<'b> {
    Plain(String),
    Parse(nom::Err<VerboseError<&'b str>>),
}

impl<'b> From<nom::Err<VerboseError<&'b str>>> for WqlError<'b> {
    fn from(e: nom::Err<VerboseError<&'b str>>) -> Self {
        WqlError::Parse(e)
    }
}
