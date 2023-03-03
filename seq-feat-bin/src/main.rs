use anyhow::Result;
use bio::io::fasta;
use clap::Parser;
use polars::datatypes::DataType::*;
use polars::prelude::*;
use seq_feat::{stats, utils};
use std::fs;
use std::io;

#[derive(Parser, Debug)]
struct Args {
    /// Where to read the fasta file from
    input: String,
    // Where to write the output parquet
    output: String,
    // Number of IDs to extract per chunk (default is all, a Bad Idea)
    num_ids: Option<i32>,
}

fn get_features(id: &str, seq: &str) -> Result<DataFrame> {
    let seq_clean = utils::seq::clean(seq)?;
    let gc = stats::counts::gc_content(&seq_clean);
    let fickett = stats::fickett::score(&seq_clean);
    let se = stats::entropy::shannon(&seq_clean, 4);
    let te = stats::entropy::tsallis(&seq_clean, 4);

    Ok(df!("id" => &[id],
            "gc_cont" => &[gc], 
            "fickett" => [fickett], 
            "shannon_1" => [se[0]], 
            "shannon_2" => [se[1]], 
            "shannon_3" => [se[2]],  
            "shannon_4" => [se[3]], 
            "tsallis_1" => [te[0]], 
            "tsallis_2" => [te[1]], 
            "tsallis_3" => [te[2]], 
            "tsallis_4" => [te[3]])?)
}

fn main() -> Result<()> {
    let cli = Args::parse();

    let max_ids = cli.num_ids.unwrap_or(i32::MAX);

    // Use traits to allow reading from stdin or a file - wrapped in a buffered reader, both have the BufRead trait
    let input: Box<dyn io::BufRead> = if cli.input == "-" {
        Box::new(io::BufReader::new(io::stdin()))
    } else {
        Box::new(io::BufReader::new(fs::File::open(cli.input).unwrap()))
    };

    let mut parquet_schema = Schema::new();
    parquet_schema.with_column("ID".to_string(), Utf8);
    parquet_schema.with_column("gc_cont".to_string(), Float64);
    parquet_schema.with_column("fickett".to_string(), Float64);
    parquet_schema.with_column("shannon_1".to_string(), Float64);
    parquet_schema.with_column("shannon_2".to_string(), Float64);
    parquet_schema.with_column("shannon_3".to_string(), Float64);
    parquet_schema.with_column("shannon_4".to_string(), Float64);
    parquet_schema.with_column("tsallis_1".to_string(), Float64);
    parquet_schema.with_column("tsallis_2".to_string(), Float64);
    parquet_schema.with_column("tsallis_3".to_string(), Float64);
    parquet_schema.with_column("tsallis_4".to_string(), Float64);

    let mut output = ParquetWriter::new(fs::File::create(cli.output)?).batched(&parquet_schema)?;
    let mut reader = fasta::Reader::new(input).records();

    let mut feature_df = DataFrame::default();
    let mut count = 0;

    while let Some(Ok(record)) = reader.next() {
        let feature_result =
            get_features(record.id(), std::str::from_utf8(record.seq()).unwrap()).ok();
        match feature_result {
            Some(features) => feature_df.vstack_mut(&features).ok().unwrap(),
            None => continue,
        };
        count += 1;

        if count >= max_ids {
            feature_df.rechunk();
            println!("{:?}", feature_df);
            output.write_batch(&feature_df)?;
            feature_df = DataFrame::default();
            count = 0;
        }
    }
    // catch the tail of the execution:
    if count > 0 {
        feature_df.rechunk();
        output.write_batch(&feature_df)?;
    }

    output.finish()?;
    Ok(())
}
