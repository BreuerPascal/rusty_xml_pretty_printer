use std::fs;
use unicode_segmentation::UnicodeSegmentation;

const TAB_SIZE: usize = 2;

fn main() {
    let path = fs::read_to_string("test_input.xml");
    let content = if let Ok(content) = path {
        content
    }else {
        panic!("unable to open file! Did you miss typed it?")
    };

    let split_content = split_xml(&content);
    println!("content: {}", split_content.join("\n"));
    println!("\n\n\n");
    let pretty_printed_content = pretty_print(&split_content, 0);
    println!("{}", pretty_printed_content);

}

fn split_xml(xml: &str) -> Vec<String> {
    let mut result = String::new();
    for current_char in xml.graphemes(true) {
        if current_char == "<" {
            result.push('\n');
        }
        result.push_str(current_char);
        if current_char == ">" {
            result.push('\n');
        }
    }

    result.lines().map(String::from).filter(|s| !s.is_empty()).collect()
}

fn pretty_print(lines: &Vec<String>, depth: usize) -> String {
    let mut result = String::new();
    let mut iter = lines.iter().enumerate().peekable();
    let s = iter.peek();
    if let Some((_, xml_decl)) = s {
        if xml_decl.starts_with("<?xml")
        {
            let (_, xml_decl) = iter.next().unwrap();
            result.push_str(&format!("{}\n", xml_decl));
        }
    }
    let mut depth : usize = 0;
    for (index, line) in iter {
        let is_tag_end = line.starts_with("</");
        let is_tag_start = !is_tag_end && line.starts_with("<");
        if is_tag_end{
            depth -= 1;
        }
        result.push_str(&format!("{}{}\n", " ".repeat(TAB_SIZE * depth), line));
        if is_tag_start {
            depth += 1;
        }
    }
    result
}