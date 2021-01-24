use std::fs::OpenOptions;
use std::fs::File;
use structopt::StructOpt;
use std::path::{Path, PathBuf};
use csv::{ReaderBuilder, WriterBuilder};
use std::ffi::OsStr;
use regex::Regex;

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

    for record in reader.records() {
        let _record = record.unwrap();
        let mut row: Vec<_> = _record.into_iter().collect();

        // Here is where the text would be converted, but we should find a way to get
        // the cells to affect from the user.
        let content = row[1].to_string();
        let converted = convert(content);
        row[1] = &converted;
        writer.write_record(&row)?;
    }

    writer.flush()?;
    Ok(())
}

fn convert(mut text: String) -> String {
    if !text.is_empty() {
        String::from("");
    }

    // Wordpress appends this - not sure why.
    text.push_str("\n");

    let re = Regex::new(r"<br\s*/?>\s*<br\s*/?>").unwrap();
    text = re.replace_all(&text, "\n\n").to_string();

    let re = Regex::new(r"<(?P<tag>table|thead|tfoot|caption|col|colgroup|tbody|tr|td|th|div|dl|dd|dt|ul|ol|li|pre|form|map|area|blockquote|address|math|style|p|h[1-6]|hr|fieldset|legend|section|article|aside|hgroup|header|footer|nav|figure|figcaption|details|menu|summary)[\s/>]").unwrap();
    text = re.replace_all(&text, "\n\n<$tag>").to_string();
    text = re.replace_all(&text, "<$tag>\n\n").to_string();

    text = Regex::new(r"(?P<tag><hr\s*?/?>)").unwrap().replace_all(&text, "$tag\n\n").to_string();

    text = Regex::new(r"(\\r\\n)").unwrap().replace_all(&text, "\n").to_string();
    text = Regex::new(r"(\\r)").unwrap().replace_all(&text, "\n").to_string();
    text
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