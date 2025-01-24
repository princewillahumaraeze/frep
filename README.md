# frep

A fast and simple grep-style search program written in Rust. frep allows you to search for text patterns in files, with support for both case-sensitive and case-insensitive searches.

## Features

- Text pattern searching in files
- Case-sensitive and case-insensitive search options
- Simple and intuitive command-line interface
- Fast performance with Rust implementation

## Installation

To install frep, you'll need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

Then, you can clone and build the project:

```bash
git clone https://github.com/princewillahumaraeze/frep.git
cd frep
cargo build --release
```

## Usage

* ####  Basic Usage
``` bash
frep <search_pattern> <file_path>
```

* #### Example Usage
```bash
frep 'hello' example.txt
```

* #### Enviroment Variables
    * IGNORE_CASE: Set this enviroment variable to enable case-insensitive search
    ```bash
    # On Unix-like systems
    IGNORE_CASE=1 frep <search_pattern> <file_path>

    # On Windows PowerShell
    $env:IGNORE_CASE=1; frep <search_pattern> <  file_path>
    ```

## Project Structure


* src/main.rs: Contains the command-line interface and program entry point
* src/lib.rs: Contains the core search functionality and configuration handling


## Error Handling

##### frep provides clear error messages for common issues:

* Missing search pattern or file path
* File not found or inaccessible
* Invalid UTF-8 content in files
