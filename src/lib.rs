mod record;

use std::io::{BufRead, BufReader, Lines, Read};
use std::str::FromStr;

use record::Record;

pub struct TacviewParser<R> {
    lines: Lines<BufReader<R>>,
}

impl<R> TacviewParser<R> {
    pub fn new(rd: R) -> Result<Self, ParseError>
    where
        R: Read,
    {
        let mut lines = BufReader::new(rd).lines();

        let file_type = lines.next().ok_or(ParseError::InvalidFileType)??;
        if file_type != "FileType=text/acmi/tacview"
            && file_type != "\u{feff}FileType=text/acmi/tacview"
        {
            return Err(ParseError::InvalidFileType);
        }

        let version = lines.next().ok_or(ParseError::InvalidVersion)??;
        if version.get(..version.len() - 1) != Some("FileVersion=2.")
            || !version
                .get(version.len() - 1..)
                .map(|s| s.chars().all(|c| c.is_ascii_digit()))
                .unwrap_or(false)
        {
            return Err(ParseError::InvalidVersion);
        }

        Ok(TacviewParser { lines })
    }
}

impl<R> Iterator for TacviewParser<R>
where
    R: Read,
{
    type Item = Result<Record, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self
                .lines
                .next()
                .filter(|r| r.as_ref().map(|l| !l.is_empty()).unwrap_or(true))?
                .map_err(ParseError::Io)
                .and_then(parse_line)
                .transpose();
            if next.is_some() {
                return next;
            }
        }
    }
}

fn parse_line(line: String) -> Result<Option<Record>, ParseError> {
    let mut chars = line.chars();
    match chars.next().ok_or(ParseError::Eol)? {
        '-' => {
            let id = u64::from_str_radix(&line[1..], 16)?;
            Ok(Some(Record::Remove(id)))
        }
        '#' => {
            let id = f64::from_str(&line[1..])?;
            Ok(Some(Record::Frame(id)))
        }
        '/' if chars.next() == Some('/') => Ok(None),
        _ => {
            let (id, rest) = line.split_once(',').ok_or(ParseError::Eol)?;

            Ok(Some(if id == "0" {
                let (name, value) = rest
                    .split_once('=')
                    .ok_or(ParseError::MissingDelimiter('='))?;
                if name == "Event" {
                    Record::Event(record::Event::from_str(value)?)
                } else {
                    Record::GlobalProperty(record::GlobalProperty::from_str(rest)?)
                }
            } else {
                Record::Update(record::Update::from_str(&line)?)
            }))
        }
    }
}

// TODO: line and position information for certain errors?
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("input is not a ACMI file")]
    InvalidFileType,
    #[error("invalid version, expected ACMI v2.x")]
    InvalidVersion,
    #[error("error reading input")]
    Io(#[from] std::io::Error),
    #[error("unexpected end of line")]
    Eol,
    #[error("object id is not a u64")]
    InvalidId(#[from] std::num::ParseIntError),
    #[error("expected numeric")]
    InvalidNumeric(#[from] std::num::ParseFloatError),
    #[error("could not find expected delimiter `{0}`")]
    MissingDelimiter(char),
    #[error("failed to parse event")]
    InvalidEvent,
    #[error("encountered invalid coordinate format")]
    InvalidCoordinateFormat,
}
