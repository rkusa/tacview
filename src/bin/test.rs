use std::fs::File;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).expect("missing filename");

    let start = Instant::now();

    let mut file = File::open(filename)?;
    let parser = tacview::Parser::new_compressed(&mut file)?;
    for record in parser {
        println!("{:?}", record?);
    }

    println!("Took: {:.4}s", start.elapsed().as_secs_f64());

    Ok(())
}
