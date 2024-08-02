use std::io::{self, Seek, Write};

use zip::write::SimpleFileOptions;
use zip::ZipWriter;

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
        zip.start_file("track.txt.acmi", SimpleFileOptions::default())?;
        Writer::new(zip)
    }

    pub fn write(&mut self, record: impl Into<Record>) -> Result<(), io::Error> {
        writeln!(self.wr, "{}", record.into())?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.wr
    }
}
