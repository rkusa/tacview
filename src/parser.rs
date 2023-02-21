use std::io::{BufReader, Read};
use std::str::FromStr;

use zip::read::ZipFile;
use zip::result::ZipError;

use crate::record::{self, Record};

pub struct Parser<R> {
    lines: lines::Lines<BufReader<R>>,
}

impl<R> Parser<R> {
    pub fn new(rd: R) -> Result<Self, ParseError>
    where
        R: Read,
    {
        let mut lines = lines::Lines::new(BufReader::new(rd));

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

        Ok(Parser { lines })
    }

    pub fn new_compressed(rd: &mut R) -> Result<Parser<ZipFile<'_>>, ParseError>
    where
        R: Read,
    {
        let file = zip::read::read_zipfile_from_stream(rd)?
            .ok_or(ParseError::Zip(ZipError::FileNotFound))?;
        Parser::new(file)
    }
}

impl<R> Iterator for Parser<R>
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

mod lines {
    use std::io::BufRead;

    /// An iterator over the non-escaped lines of an instance of `BufRead`.
    #[derive(Debug)]
    pub struct Lines<B> {
        buf: B,
    }

    impl<B> Lines<B> {
        pub fn new(buf: B) -> Self {
            Self { buf }
        }
    }

    impl<B: BufRead> Iterator for Lines<B> {
        type Item = std::io::Result<String>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut buf = String::new();
            loop {
                match self.buf.read_line(&mut buf) {
                    Ok(0) => {
                        if buf.is_empty() {
                            return None;
                        } else {
                            return Some(Ok(buf));
                        }
                    }
                    Ok(_n) => {
                        if buf.ends_with("\\\n") {
                            buf.remove(buf.len() - 2);
                            continue;
                        }
                        if buf.ends_with("\\\r\n") {
                            buf.remove(buf.len() - 3);
                            continue;
                        }
                        if buf.ends_with('\n') {
                            buf.pop();
                            if buf.ends_with('\r') {
                                buf.pop();
                            }
                        }
                        return Some(Ok(buf));
                    }
                    Err(e) => return Some(Err(e)),
                }
            }
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
    #[error("error reading zip compressed input")]
    Zip(#[from] zip::result::ZipError),
}

#[test]
fn test_multi_line_comment() {
    let acmi = r#"FileType=text/acmi/tacview
FileVersion=2.2
0,Comments=1\
2\
\
3
0,Title=Test"#;
    let p = Parser::new(acmi.as_bytes()).unwrap();
    let records = p.collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(
        records,
        vec![
            Record::GlobalProperty(record::GlobalProperty::Comments("1\n2\n\n3".to_string())),
            Record::GlobalProperty(record::GlobalProperty::Title("Test".to_string()))
        ]
    );
}
