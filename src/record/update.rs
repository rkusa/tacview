use std::fmt::Display;
use std::str::FromStr;

use super::Property;
use crate::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub struct Update {
    pub id: u64,
    pub props: Vec<Property>,
}

impl FromStr for Update {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, mut rest) = line.split_once(',').ok_or(ParseError::Eol)?;
        let id = u64::from_str_radix(id, 16)?;
        let mut props = Vec::new();

        let mut prev = None;
        let mut offset = 0;
        for (i, ch) in rest.char_indices() {
            if ch == ',' && prev != Some('\\') {
                let (kv, r) = rest.split_at(i - offset);
                rest = r.strip_prefix(',').unwrap_or(rest);
                offset = i + 1;

                props.push(Property::from_str(kv)?);
            }

            prev = Some(ch);
        }

        if !rest.is_empty() {
            props.push(Property::from_str(rest)?);
        }

        Ok(Update { id, props })
    }
}

impl Display for Update {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)?;
        for p in &self.props {
            write!(f, ",{p}")?;
        }
        Ok(())
    }
}
