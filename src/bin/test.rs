use std::fs::File;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).expect("missing filename");

    let start = Instant::now();

    let file = File::open(filename)?;
    let mut zip = zip::ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        let parser = tacview::Parser::new(file)?;
        for record in parser {
            println!("{:?}", record?);
        }
    }

    println!("Took: {:.4}s", start.elapsed().as_secs_f64());

    Ok(())
}
