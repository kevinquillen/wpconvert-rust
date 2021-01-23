use std::fs::OpenOptions;
use std::fs::File;
use structopt::StructOpt;
use std::path::{Path, PathBuf};
use csv::{ReaderBuilder, WriterBuilder};
use std::ffi::OsStr;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(
        short = "i",
        long = "in",
        help = "The CSV file to parse and convert WP text to WP html.",
    )]
    pub infile: PathBuf,

    #[structopt(
        short = "o",
        long = "out",
        help = "The file to save the result to.",
    )]
    pub outfile: PathBuf
}

fn main() -> Result<(), csv::Error> {
    let opt = Opt::from_args();
    let infile_path = opt.infile.to_str().unwrap();
    let outfile_path = opt.outfile.to_str().unwrap();

    file_is_csv(infile_path);
    file_is_csv(outfile_path);

    let infile = File::open(infile_path)?;
    let outfile = OpenOptions::new().write(true).create(true).open(outfile_path)?;

    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(outfile);

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(infile);

    let headers: Vec<_> = reader.headers().unwrap().into_iter().collect();
    writer.write_record(headers)?;

    // Write the rows
    // Here is where the text would be converted, but we should find a way to get
    // the cells to affect from the user.
    for record in reader.records() {
        let _record = record.unwrap();
        let row: Vec<_> = _record.into_iter().collect();
        writer.write_record(&row)?;
    }

    writer.flush()?;
    Ok(())
}

/// Crude method to check if the file is a csv file.
fn file_is_csv(filename: &str) {
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);

    if extension != Some("csv") {
        panic!("{:?} must be a CSV file.", filename);
    }
}