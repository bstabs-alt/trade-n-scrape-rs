use std::fs::File;
use std::io::Write;

pub fn save_html_to_file(html_content: &str){
    let mut file = File::create("trademe.html").expect("Could not create file!");
    file.write_all(html_content.as_bytes()).expect("could not write to file!");
}