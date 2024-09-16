use std::{collections::HashMap, fs};
use regex::Regex;

struct Corpus{
    dir: String,
}

impl Corpus{
    fn new(dir_name: String)->Self{
        Corpus{
            dir:dir_name
        }
    }

    fn gen_corpus(&self)->HashMap<String, Vec<String>>{
        let paths = fs::read_dir(&self.dir).unwrap();
        //let mut corpus: Vec<Vec<String>> = Vec::new();
        let mut corpus: HashMap<String, Vec<String>> = HashMap::new();
    
        for path in paths{
            let fd = path.unwrap().path();
            let text = fs::read_to_string(&fd).unwrap();
            corpus.insert(fd.display().to_string(), self.tokenize(&text));
        }
        corpus
    }
    
    fn tokenize(&self, text: &String) -> Vec<String>{
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
                tokens.push(find.to_string().to_ascii_lowercase());
            }
        }
        tokens
    }
    
    fn get_tokens_from_file(&self, filename: &str)->Vec<String>{
        let fd = self.dir.clone()+filename;
        let text = fs::read_to_string(fd).unwrap();
        let ret_tokens = self.tokenize(&text);
        ret_tokens
    }
    fn term_frequency(&self, term: String, filename: &str)->f32{
        let mut tocc = 0.0;
        let tokens = self.get_tokens_from_file(filename);
        for token in &tokens{
            if term.to_ascii_lowercase()==*token{
                tocc+=1.0;
            }
        }
        let tf = tocc/tokens.len() as f32;
        tf
    }

    fn tfidf(){

    }
}

fn main() {
    let dir_name = "./text-files/";
    let corpus = Corpus::new(dir_name.to_string());

    //println!("{:#?}", corpus.get_tokens_from_file("agatha_complete.txt").len());

    let term = "curious";
    println!("term frequency of \"{term}\" is: {}", corpus.term_frequency(term.to_string(), "long.txt"));
    println!("{:#?}", corpus.get_tokens_from_file("long.txt"));
}
