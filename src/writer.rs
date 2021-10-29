use std::io::{self, Seek, Write};

use zip::{write::FileOptions, ZipWriter};

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

    pub fn new_compressed(wr: W) -> Result<Writer<impl Write>, io::Error>
    where
        W: Seek,
    {
        let mut zip = ZipWriter::new(wr);
        zip.start_file("track.txt.acmi", FileOptions::default())?;
        Writer::new(zip)
    }

    pub fn write(&mut self, record: impl Into<Record>) -> Result<(), io::Error> {
        writeln!(self.wr, "{}", record.into().to_string())?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.wr
    }
}
