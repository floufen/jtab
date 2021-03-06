use std::error::Error;
use std::io;

use structopt::StructOpt;

use printer::{Printer, TablePrinter};
use reader::{OneShotValueReader, StreamingValueReader, ValueReader};

use crate::printer::{ColorizeSpec, JsonTable, TableHeader};

mod printer;
mod reader;

#[derive(Debug, StructOpt)]
#[structopt(about = "Print any json data as a table from the command line")]
struct Command {
    #[structopt(long, help = "receive one json per line")]
    streaming: bool,

    #[structopt(long, short, help = "select a subset of fields")]
    fields: Option<Vec<String>>,

    #[structopt(long, short, help = "add a color spec to a column in the form of: 'col:value:spec'")]
    colorize: Vec<String>,

    #[structopt(long, help = "limit the number of printed elements")]
    take: Option<usize>,
}

type GenericResult<T> = Result<T, Box<dyn Error>>;

fn main() -> GenericResult<()> {
    let command: Command = Command::from_args();
    let stdin = io::stdin();

    let data =
        if command.streaming {
            StreamingValueReader::new(stdin.lock()).read_value(command.take)?
        } else {
            OneShotValueReader::new(stdin).read_value(command.take)?
        };

    let colorize: Vec<_> =
        command.colorize.iter().map(|c| ColorizeSpec::parse(c)).collect::<Result<_, _>>()?;

    let given_headers = match command.fields {
        Some(fields) => Some(TableHeader::NamedFields { fields }),
        None => None
    };

    let table = JsonTable::new(given_headers, &data);
    TablePrinter::new(colorize).print(&table)?;

    Ok(())
}