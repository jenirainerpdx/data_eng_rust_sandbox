//command-line tool that reads a CSV file and prints the contents of the file as a DataFrame
use clap::{Parser, Subcommand};
use polars::prelude::*;
const CSV_FILE: &str = "src/data/global-life-expt-2022.csv";

#[derive(Parser)]
//add extended help
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Print {
        #[arg(long, default_value = CSV_FILE)]
        path: String,
        #[arg(long, default_value = "10")]
        rows: usize,
    },
    Describe {
        #[arg(long, default_value = CSV_FILE)]
        path: String,
    },
    Schema {
        #[arg(long, default_value = CSV_FILE)]
        path: String,
    },
    Shape {
        #[arg(long, default_value = CSV_FILE)]
        path: String,
    },
    Sort {
        #[arg(long, default_value = CSV_FILE)]
        path: String,
        #[arg(long, default_value = "2020")]
        year: String,
        #[arg(long, default_value = "10")]
        rows: usize,
        #[arg(long, default_value = "true")]
        order: bool,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Print { path, rows }) => {
            let df = polars_stuff::read_csv(&path);
            println!("{:?}", df.head(Some(rows)));
        }
        Some(Commands::Describe { path }) => {
            let df = polars_stuff::read_csv(&path);
            println!("{:?}", df);
        }
        Some(Commands::Schema { path }) => {
            let df = polars_stuff::read_csv(&path);
            println!("{:?}", df.schema());
        }
        Some(Commands::Shape { path }) => {
            let df = polars_stuff::read_csv(&path);
            println!("{:?}", df.shape());
        }
        Some(Commands::Sort {
            path,
            year,
            rows,
            order,
        }) => {
            let df = polars_stuff::read_csv(&path);
            let country_column_name = "Country Name";
            //select the country column and the year string passed in and return a new dataframe
            let columns = [country_column_name, &year];
            let vs = columns
                .iter()
                .map(|col| df.column(col).unwrap().clone())
                .collect::<Vec<_>>();
            //convert the Vec<Series> to a DataFrame
            let df2: DataFrame = DataFrame::new(vs).unwrap();
            //drop any rows with null values and return a new dataframe
            // sort the dataframe by the year column and by order passed in
            let sort_options = SortMultipleOptions::new().with_order_descending(order);
            let df2_sorted = df2.sort([&year], sort_options).unwrap();

            //print the first "rows" of the dataframe
            println!("{:?}", df2_sorted.head(Some(rows)));
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
