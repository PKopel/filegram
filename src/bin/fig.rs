use std::{
    fs::{self, File},
    io::{BufReader, Error, ErrorKind},
    path::Path,
};

use clap::{Args, Parser, Subcommand};
use filegram::{decode, encode};
use image::ImageFormat;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    fn execute(self) -> Result<(), Error> {
        let command: Box<dyn CommandTrait> = match self.command {
            Command::Encode(encode) => Box::new(encode),
            Command::Decode(decode) => Box::new(decode),
        };
        command.execute()
    }
}

#[derive(Subcommand)]
enum Command {
    Encode(Encode),
    Decode(Decode),
}

trait CommandTrait {
    fn execute(&self) -> Result<(), Error>;
    fn default_output(&self) -> String;
}

#[derive(Args)]
struct Encode {
    #[arg(short, long)]
    file: String,
    #[arg(short, long)]
    output: Option<String>,
}

impl CommandTrait for Encode {
    fn execute(&self) -> Result<(), Error> {
        let output = self.output.clone().unwrap_or_else(|| self.default_output());
        let file = File::open(self.file.clone())?;
        let file_size = file.metadata()?.len() as usize;
        let mut file = BufReader::new(file);
        let rgb = encode::to_rgb(&mut file, file_size);
        let path = Path::new(&output);
        rgb.save(path)
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        Ok(())
    }

    fn default_output(&self) -> String {
        self.file.clone() + ".png"
    }
}

#[derive(Args)]
struct Decode {
    #[arg(short, long)]
    file: String,
    #[arg(short, long)]
    output: Option<String>,
}

impl CommandTrait for Decode {
    fn execute(&self) -> Result<(), Error> {
        let output = self.output.clone().unwrap_or_else(|| self.default_output());
        let file = File::open(self.file.clone())?;
        let data = decode::from_file(BufReader::new(file), ImageFormat::Png);
        fs::write(output, data)?;
        Ok(())
    }

    fn default_output(&self) -> String {
        match self.file.strip_suffix(".png") {
            Some(output) => output.to_string(),
            None => self.file.clone() + ".decoded",
        }
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    cli.execute()
}
