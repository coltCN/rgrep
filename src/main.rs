use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use anyhow::Result;
use clap::Parser;
#[derive(Parser, Debug)]
#[clap[version = "1.0", author = "coltCN"]]
struct Opts {
    /// 需要查找的单词
    word: String,
    /// 目标文件
    file: String,
}
fn main() {
    let opts: Opts = Opts::parse();
    // println!("{:?}", opts);

    match read_line(&opts.file) {
        Ok(lines) => match_word(lines, &opts.word),
        Err(e) => println!("Err:{}", e),
    }
}

fn read_line(filename: &str) -> Result<Lines<BufReader<File>>> {
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}

fn match_word(lines: Lines<BufReader<File>>, word: &str) {
    let mut line_num = 0;
    for line in lines {
        line_num += 1;
        if let Ok(l) = line {
            if let Some(col) = l.find(word) {
                println!("{}:{}\t {}", line_num, col, l.trim());
            }
        }
    }
}
