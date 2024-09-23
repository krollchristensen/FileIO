/*
Eksperiment: Læse og skrive store filer ved hjælp af BufReader for at f
orbedre effektiviteten. BufReader læser filer i små bidder, hvilket er mere
effektivt, når du arbejder med store filer.

 */
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

fn main() -> io::Result<()> {
    // Åbn en stor fil
    let file = File::open("large_input.txt")?;

    // Læs filen i små bidder ved hjælp af BufReader
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0; 1024]; // Læser 1 KB ad gangen

    let mut output_file = File::create("output.txt")?;
    let mut writer = BufWriter::new(output_file);

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        writer.write_all(&buffer[..bytes_read])?;
    }

    println!("Filen er læst og skrevet i små bidder.");
    Ok(())
}
/*
Brug af BufReader og BufWriter til at læse og skrive store filer i
små bidder kan forbedre effektiviteten, især når du arbejder med meget
store filer. Dette reducerer hukommelsesforbruget, da data ikke behøver at
blive holdt i hukommelsen på én gang.
 */