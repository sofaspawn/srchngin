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

    fn tfidf(&self, term: String)->HashMap<String, f32>{
        let corpus = self.gen_corpus();
        //tf
        let mut term_frequencies = HashMap::new();
        let mut tcount = 0;
        for (file, tokens) in corpus{
            for token in &tokens{
                if *token==term{
                    tcount+=1;
                }
            }
            let tf = (tcount as f32)/(tokens.len() as f32);
            term_frequencies.entry(file).or_insert(tf);
        }

        //idf
        let mut docs_containing_term = 0;
        for val in term_frequencies.values(){
            if *val != 0.0{
                docs_containing_term+=1;
            }
        }

        let idf = f32::log((term_frequencies.len() as f32 / docs_containing_term as f32), 10.0);

        let mut tfidf = term_frequencies.clone();
        for (k, v) in term_frequencies{
            tfidf.insert(k, v*idf);
        }

        return tfidf.clone();
    }
}

fn main() {
    let dir_name = "./text-files/";
    let corpus = Corpus::new(dir_name.to_string());

    println!("{:#?}", corpus.tfidf(String::from("a")));
}
