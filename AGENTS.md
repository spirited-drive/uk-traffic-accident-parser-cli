# UK Traffic Accident Importer

A CLI written in Rust that imports UK traffic and accident data from CSVs into a PostgreSQL databaase.

## Project overview

Code is located inside the /src directory with main.rs as the starting point. This is a CLI console app written in Rust using the Tokio async runtime. SQL schema scripts for creating the data can be found in the /sql directory.

The CSVs are located in /data directory, but don't try to read these as these are incredibly large multi-gigabyte files. Instead use the code to infer their data structure.

## Formatting

Indentations should be 4 spaces. Write `else {` and `else if {` on their own line. CSS declarations should be on their own line.