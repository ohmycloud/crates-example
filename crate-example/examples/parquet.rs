use parquet::{
    file::reader::{FileReader, SerializedFileReader},
    schema::printer::{print_file_metadata, print_parquet_metadata, print_schema},
};
use std::{fs::File, path::Path};

use serde_json::{Result, Value};

fn main() {
    let file = File::open(&Path::new("person.parquet")).unwrap();
    let reader = SerializedFileReader::new(file).unwrap();
    let mut iter = reader.get_row_iter(None).unwrap();
    while let Some(record) = iter.next() {
        println!("{}", record);
    }

    let parquet_metadata = reader.metadata();
    let key_value_metadata = &parquet_metadata.file_metadata().key_value_metadata();

    // match key_value_metadata {
    //     Some(meta) => println!("{:?}", meta),
    //     _ => println!("{:?}", "ok")
    // }

    if let Some(meta) = key_value_metadata {
        //println!("{:?}", meta);

        for kv in meta {
            if let Some(v) = &kv.value {
                println!("{:?}", v);

                let j: Value = serde_json::from_str(v).unwrap();

                // match &j["fields"] {
                //     Value::Array(x) => println!("{:?}", x),
                //     _ => println!("{:?}", "ok")
                // }

                for item in j["fields"].as_array() {
                    for i in item {
                        println!("{}", i["name"].to_string())
                    }
                }
            }
        }
    }

    //print_parquet_metadata(&mut std::io::stdout(), &parquet_metadata);

    //let columns = parquet_metadata.file_metadata().schema().get_fields();

    //println!("{:#?}",columns);
}
