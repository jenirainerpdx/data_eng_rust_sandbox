use chrono::prelude::*;
use polars::df;
use polars::frame::DataFrame;
use polars::io::SerWriter;
use polars::prelude::*;
use std::fs::File;

fn main() {
    let mut df: DataFrame = df!(
      "name" => ["John", "Fred", "Betty", "Rose"],
        "birthdate" => [
            NaiveDate::from_ymd_opt(1969, 6,12).unwrap(),
            NaiveDate::from_ymd_opt(1970, 2, 14).unwrap(),
            NaiveDate::from_ymd_opt(1975, 4, 1).unwrap(),
            NaiveDate::from_ymd_opt(1980, 9, 10).unwrap(),
        ],
        "weight" => [59.9, 72.5, 64.3, 63.8],
        "height" => [1.82, 1.69, 1.95, 1.76],
        "is_male" => [true, true, false, false]
    )
    .unwrap();
    println!("{:?}", df);

    let mut file = File::create("src/data/output.csv").expect("Unable to create file");
    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df)
        .expect("Things went terribly awry.");
    // Read the CSV back into a DataFrame
    let polars_result = CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
        .try_into_reader_with_file_path(Some("src/data/output.csv".into()));
    let df_csv = polars_result.unwrap().finish().unwrap();
    println!("{:?}", df_csv);

    let result = df
        .clone()
        .lazy()
        .select([
            col("name"),
            col("birthdate").dt().year().alias("birth_year"),
            (col("weight") / col("height").pow(2)).alias("bmi"),
        ])
        .collect();
    println!("{:?}", result);

    // expression expansion and suffixes
    let result2 = df
        .clone()
        .lazy()
        .select([
            col("name"),
            (cols(["weight", "height"]).as_expr() * lit(0.95))
                .round(2, RoundMode::default())
                .name()
                .suffix("-5%"),
        ])
        .collect();
    println!("{:?}", result2);

    // with columns
    let result3 = df
        .clone()
        .lazy()
        .with_columns([
            col("birthdate").dt().year().alias("birth_year"),
            (col("weight") / col("height").pow(2)).alias("bmi"),
        ])
        .collect();
    println!("{:?}", result3);

    let result4 = df
        .clone()
        .lazy()
        .filter(
            col("birthdate")
                .is_between(
                    lit(NaiveDate::from_ymd_opt(1960, 12, 31).unwrap()),
                    lit(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
                    ClosedInterval::Both,
                )
                .and(col("is_male").eq(lit(true))),
        )
        .collect();
    println!("{:?}", result4);
}
