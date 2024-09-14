use std::fs;

fn read_files()->Vec<String>{
    let paths = fs::read_dir("./text-files").unwrap();
    let mut texts: Vec<String> = Vec::new();

    for path in paths{
        let text = fs::read_to_string(path.unwrap().path()).unwrap();
        texts.push(text);
    }

    texts
}

fn main() {
    let texts = read_files();
    for text in texts{
        println!("{}",text);
    }
}
