use std::io::{self, Write};

use crate::record::Record;

pub struct Writer<W> {
    wr: W,
}

impl<W> Writer<W>
where
    W: Write,
{
    pub fn new(mut wr: W) -> Result<Self, io::Error> {
        writeln!(wr, "FileType=text/acmi/tacview")?;
        writeln!(wr, "FileVersion=2.2")?;
        Ok(Self { wr })
    }

    pub fn write(&mut self, record: impl Into<Record>) -> Result<(), io::Error> {
        writeln!(self.wr, "{}", record.into().to_string())?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.wr
    }
}
