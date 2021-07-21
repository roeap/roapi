use std::convert::TryFrom;

pub enum ContentType {
    Json,
    Csv,
    ArrowStream,
}

impl ContentType {
    pub fn to_str<'a>(&'a self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::Csv => "application/csv",
            ContentType::ArrowStream => "application/vnd.apache.arrow.stream",
        }
    }
}

impl TryFrom<&[u8]> for ContentType {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, ()> {
        match value {
            b"*/*" | b"application/json" => Ok(ContentType::Json),
            b"application/csv" => Ok(ContentType::Csv),
            b"application/arrow.stream" | b"application/vnd.apache.arrow.stream" => {
                Ok(ContentType::ArrowStream)
            }
            _ => Err(()),
        }
    }
}

pub mod arrow;
pub mod csv;
pub mod json;