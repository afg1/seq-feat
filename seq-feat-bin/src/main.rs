// use anyhow::Ok;
use anyhow::Result;
use bio::io::fasta;
use bio::io::fasta::Record;
use clap::Parser;
use core::f64::NAN;
use polars::prelude::*;
use seq_feat::{stats, utils};
use std::fs;
use std::io;
use std::sync::Mutex;
use std::thread;
use std::time;

#[derive(Parser, Debug)]
struct Args {
    /// Where to read the fasta file from
    input: String,
    /// Where to write the output parquet
    output: String,
    /// Number of IDs to extract per chunk (default is all, a Bad Idea)
    num_ids: Option<usize>,

    /// Number of threads to use to process chunks, default 1
    num_threads: Option<usize>,
}

fn chunk_get_features(records: Vec<Record>) -> Result<DataFrame> {
    let len = records.len();
    let mut ids = Vec::with_capacity(len);
    ids.resize(len, "".to_owned());
    let mut gc_cont = vec![NAN; len];
    let mut fickett = vec![NAN; len];
    let mut shannon_1 = vec![NAN; len];
    let mut shannon_2 = vec![NAN; len];
    let mut shannon_3 = vec![NAN; len];
    let mut shannon_4 = vec![NAN; len];
    let mut tsallis_1 = vec![NAN; len];
    let mut tsallis_2 = vec![NAN; len];
    let mut tsallis_3 = vec![NAN; len];
    let mut tsallis_4 = vec![NAN; len];

    let mut idx: usize = 0;

    for record in records {
        let id = record.id().to_owned();
        let seq = std::str::from_utf8(record.seq())?;
        let seq_clean = match utils::seq::clean(seq) {
            Ok(clean_seq) => clean_seq,
            Err(_cs) => {
                idx += 1;
                continue;
            } // Skip processing this one -
        };

        let gc = stats::counts::gc_content(&seq_clean);
        let fickett_seq = stats::fickett::score(&seq_clean);
        let se = stats::entropy::shannon(&seq_clean, 4);
        let te = stats::entropy::tsallis(&seq_clean, 4);

        ids[idx] = id;
        gc_cont[idx] = gc;
        fickett[idx] = fickett_seq;
        shannon_1[idx] = se[0];
        shannon_2[idx] = se[1];
        shannon_3[idx] = se[2];
        shannon_4[idx] = se[3];
        tsallis_1[idx] = te[0];
        tsallis_2[idx] = te[1];
        tsallis_3[idx] = te[2];
        tsallis_4[idx] = te[3];
        idx += 1;
    }

    let prefiltered_features = df!("id" => ids,
        "gc_cont"   =>  gc_cont  ,
        "fickett"   =>  fickett  ,
        "shannon_1" =>  shannon_1,
        "shannon_2" =>  shannon_2,
        "shannon_3" =>  shannon_3,
        "shannon_4" =>  shannon_4,
        "tsallis_1" =>  tsallis_1,
        "tsallis_2" =>  tsallis_2,
        "tsallis_3" =>  tsallis_3,
        "tsallis_4" =>  tsallis_4,
    )?;
    let mask = prefiltered_features.column("gc_cont")?.is_not_nan()?;
    Ok(prefiltered_features.filter(&mask)?) // should filter sequences with invalid sequences
}

fn main() -> Result<()> {
    let cli = Args::parse();

    let max_ids: usize = cli.num_ids.unwrap_or(usize::MAX);
    let num_threads = cli.num_threads.unwrap_or(1);

    // Use traits to allow reading from stdin or a file - wrapped in a buffered reader, both have the BufRead trait
    let input: Box<dyn io::BufRead> = if cli.input == "-" {
        Box::new(io::BufReader::new(io::stdin()))
    } else {
        Box::new(io::BufReader::new(fs::File::open(cli.input).unwrap()))
    };

    let output = ParquetWriter::new(fs::File::create(cli.output)?);
    let mut reader = fasta::Reader::new(input).records();

    let feature_df = Arc::new(Mutex::new(DataFrame::default()));

    let mut sequences_remain = true;
    let mut idx: usize = 0;
    let mut thread_count = 0;
    let mut handles = vec![];
    while sequences_remain {
        let mut record_chunk: Vec<Record> = Vec::new();
        record_chunk.resize_with(max_ids, Record::new);

        while let Some(Ok(record)) = reader.next() {
            record_chunk[idx] = record;
            idx += 1;
            if idx == max_ids {
                idx = 0;
                break;
            }
        }

        // idx is not zero, so not reset, but also less than max = we left loop while in tail of sequences
        if idx != 0 && idx < max_ids {
            sequences_remain = false;
        }
        if thread_count < num_threads {
            let accum_feats = Arc::clone(&feature_df);
            let handle = thread::spawn(move || {
                let feature_result = chunk_get_features(record_chunk).ok().unwrap();
                let mut features = accum_feats.lock().unwrap();
                features.vstack_mut(&feature_result).unwrap();
            });
            handles.push(handle);
            thread_count += 1;
        } else {
            // for handle in handles {
            //     handle.join().unwrap();
            // }
            let last_thread = handles.last().unwrap();
            while !last_thread.is_finished() {
                thread::sleep(time::Duration::from_millis(10000));
            }
            thread_count = 0;
            handles.clear();
            println!(
                "{:?}",
                feature_df.lock().unwrap().column("id")?.unique()?.len()
            );
        }
    }

    feature_df.lock().unwrap().rechunk();
    output.finish(&mut feature_df.lock().unwrap())?;
    Ok(())
}
