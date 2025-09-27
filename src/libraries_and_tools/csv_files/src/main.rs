use csv::Writer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // create tuples for veggies with pricing.
    let veggies = [
        ("cucumber", 1.5),
        ("tomato", 2.0),
        ("lettuce", 0.5),
        ("onion", 0.75),
        ("carrot", 0.5),
        ("broccoli", 0.5),
        ("spinach", 0.5),
        ("peas", 0.5),
        ("asparagus", 0.5),
        ("celery", 0.5),
    ];

    let mut writer = Writer::from_path("veggies.csv")?;

    writer.write_record(["veggie", "price"])?;
    for (veggie, price) in veggies {
        writer.write_record([veggie, &price.to_string()])?;
    }
    writer.flush().expect("Unable to flush writer");
    create_discounted_products("veggies.csv", "discounted_veggies.csv", 0.1);

    Ok(())
}

fn create_discounted_products(input_filename: &str, output_filename: &str, discount: f64) {
    let mut file_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_filename)
        .expect("Unable to read file");
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(output_filename)
        .expect("Unable to write file");
    let header_data = ["product", "price", "discounted price"];
    writer
        .write_record(header_data)
        .expect("Unable to write header");
    for result in file_reader.records() {
        let record = result.expect("Unable to read record");
        let price = record[1].parse::<f64>().expect("Unable to parse price");
        let discounted_price = price * (1.0 - discount);
        let out_record = [
            record[0].to_string(),
            price.to_string(),
            discounted_price.to_string(),
        ];
        writer
            .write_record(&out_record)
            .expect("Unable to write record");
    }
    writer
        .flush()
        .expect("Unable to flush writer for discounted products.");
}
