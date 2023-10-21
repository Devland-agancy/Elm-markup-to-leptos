
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
   
    fn trf(elm: &str) -> String {
        // Create a path to the desired file
        let path = Path::new(elm);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {
                let mut output = String::new();
                //stack str for tagname , usize for indent level, bool if it's self closing
                let mut tag_stack: Vec<(&str, usize, bool)> = Vec::new();
                let lines = s.lines();
            
                for line in lines {
                    let trimmed_line = line.trim();
                    let indent = line.len() - trimmed_line.len();
            
                    if trimmed_line.starts_with("|> ") {
                        let tag_name = &trimmed_line[3..];
                        while let Some((last_tag, last_indent, is_self_closing)) = tag_stack.last().cloned() {
                            if indent <= last_indent {
                                if is_self_closing {
                                    output.push_str(&format!("/>\n"));
                                }else{
                                    output.push_str(&format!("</{}>\n", last_tag));
                                }
                                tag_stack.pop();
                            } else {
                                break;
                            }
                        }
                        output.push_str(&format!("<{}>\n", tag_name));
                        tag_stack.push((tag_name, indent, false));
                    }else if trimmed_line.starts_with("|>_ ") {
                        //self closing tag
                        let tag_name = &trimmed_line[4..];
                        while let Some((last_tag, last_indent, is_self_closing)) = tag_stack.last().cloned() {
                            if indent <= last_indent {
                                if is_self_closing {
                                    output.push_str(&format!("/>\n"));
                                }else{
                                    output.push_str(&format!("</{}>\n", last_tag));
                                }
                                tag_stack.pop();
                            } else {
                                break;
                            }
                        }
                        output.push_str(&format!("<{} \n", tag_name));
                        tag_stack.push((tag_name, indent, true));
                    } else if !trimmed_line.is_empty() {
                        while let Some((last_tag, last_indent, is_self_closing)) = tag_stack.last().cloned() {
                          

                            println!("indent{}", indent);
                            println!("last_indent{}", last_indent);

                            if indent <= last_indent {

                                if is_self_closing {
                                    output.push_str(&format!("/>\n"));
                                }else{
                                    output.push_str(&format!("</{}>\n", last_tag));
                                }
                                tag_stack.pop();
                            } else {
                                break;
                            }
                        }
                        output.push_str(&format!("{}\n", line));
                    }
                }
            
                while let Some((last_tag, _, is_self_closing)) = tag_stack.pop() {
                    if is_self_closing {
                        output.push_str(&format!("/>\n"));
                    }else{
                        output.push_str(&format!("</{}>\n", last_tag));
                    }
                }
            
                output
            },
        }
    }

    

    let html_code = trf("/home/chaker/code/lp/elm_to_view/src/elm.emu");
   println!("{}", html_code);
}
