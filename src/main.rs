use std::fs;
use regex::Regex;

fn gen_corpus(dir_name: &str)->Vec<Vec<String>>{
    let paths = fs::read_dir(dir_name).unwrap();
    let mut corpus: Vec<Vec<String>> = Vec::new();

    for path in paths{
        let fd = path.unwrap().path();
        let text = fs::read_to_string(&fd).unwrap();
        corpus.push(tokenize(&text));
    }
    corpus
}

fn tokenize(text: &String) -> Vec<String>{
    let mut tokens : Vec<String> = Vec::new();
    let re = Regex::new(r"[\w]+").unwrap();
    for line in text.split('\n').into_iter(){
        for word in line.split_ascii_whitespace(){
            let m = re.find(&word);
            let mut find = "";
            match m{
                Some(x) => find = x.as_str(),
                None => ()
            };
            tokens.push(find.to_string());
        }
    }
    tokens
}

fn main() {
    let dir_name = "./text-files";
    let tokens = gen_corpus(&dir_name);
    for token in &tokens{
        println!("{}", token.len());
    }
    //println!("{:#?}", tokens);
}
