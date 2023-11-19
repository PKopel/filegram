use std::{
    fs::{self, File},
    io::{BufReader, Error, ErrorKind, Read},
    path::Path,
};

use clap::{Args, Parser, Subcommand};
use filegram::{
    decode, encode,
    encryption::{Cipher, Key},
};

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
    #[arg(short, long)]
    encrypted: bool,
}

impl CommandTrait for Encode {
    fn execute(&self) -> Result<(), Error> {
        let output = self.output.clone().unwrap_or_else(|| self.default_output());
        let file = File::open(self.file.clone())?;
        let file_size = file.metadata()?.len() as usize;
        let mut file = BufReader::new(file);
        let rgb = if self.encrypted {
            let cipher = Cipher::new();
            save_cipher_key(cipher.get_key_struct())?;
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            let data = cipher.encrypt(&data);
            encode::from_slice(&data)
        } else {
            encode::from_reader(&mut file, file_size)
        };
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
    #[arg(
        short,
        long,
        help = "path to key file",
        default_missing_value = "filegram.key"
    )]
    encrypted: Option<Option<String>>,
}

impl CommandTrait for Decode {
    fn execute(&self) -> Result<(), Error> {
        let output = self.output.clone().unwrap_or_else(|| self.default_output());
        let file = File::open(self.file.clone())?;
        let data = decode::from_file(BufReader::new(file));
        let data = if let Some(Some(path)) = &self.encrypted {
            let key_file = File::open(path)?;
            let key = load_cipher_key(key_file)?;
            let cipher = Cipher::load(key);
            cipher.decrypt(&data)
        } else {
            data
        };
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

fn save_cipher_key(key: Key) -> Result<(), Error> {
    let key_file = File::create("filegram.key")?;
    serde_json::to_writer(key_file, &key)?;
    Ok(())
}

fn load_cipher_key(file: File) -> Result<Key, Error> {
    let key: Key = serde_json::from_reader(file)?;
    Ok(key)
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    cli.execute()
}
