use std::fs;
use regex::Regex;

fn gen_corpus()->Vec<Vec<String>>{
    let paths = fs::read_dir("./text-files").unwrap();
    let mut corpus: Vec<Vec<String>> = Vec::new();

    for path in paths{
        let text = fs::read_to_string(path.unwrap().path()).unwrap();
        corpus.push(tokenize(&text));
        break;
    }

    corpus
}

fn tokenize(text: &String) -> Vec<String>{
    let tokens : Vec<String> = Vec::new();
    let re = Regex::new(r"[\w]+").unwrap();
    let m = re.find(&text.split("\n").collect::<Vec<&str>>()[0]).unwrap();
    println!("{:#?}", m);
    tokens
}

fn main() {
    let tokens = gen_corpus();
    println!("{:#?}", tokens);
}
