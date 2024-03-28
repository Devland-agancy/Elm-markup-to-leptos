use super::element_text::ElementText;
use super::helpers::*;
use std::str::Lines;

#[derive(Debug, Default)]
pub struct TagInfo {
    pub name: String,
    pub indent: usize,
    pub is_self_closing: bool,
    pub in_props: bool,
}

#[derive(Debug, Default)]
pub struct AutoWrapper {
    pub wrap_children_with: &'static str,
    pub tags: Vec<&'static str>,
    pub enable_manual_wrap: bool, // do not wrap if child is already wrapped
}

#[derive(Debug)]
pub struct Transformer {
    pub self_closing_tags: Vec<&'static str>,
    pub tags_before_non_indents: Vec<&'static str>,
    pub no_x_padding_tags: Vec<&'static str>,
    pub tags_with_non_indent_first_child: Vec<&'static str>,
    pub tags_with_no_indents: Vec<&'static str>,
    pub auto_wrappers: Vec<AutoWrapper>,
    track_line_delta: usize,
}

impl Transformer {
    pub fn new(
        self_closing_tags: Vec<&'static str>,
        auto_wrappers: Vec<AutoWrapper>,
        no_x_padding_tags: Vec<&'static str>,
        tags_before_non_indents: Vec<&'static str>,
        tags_with_non_indent_first_child: Vec<&'static str>,
        tags_with_no_indents: Vec<&'static str>,
    ) -> Transformer {
        Transformer {
            self_closing_tags,
            auto_wrappers,
            tags_with_non_indent_first_child,
            tags_before_non_indents,
            no_x_padding_tags,
            tags_with_no_indents,
            track_line_delta: 0,
        }
    }

    pub fn pre_process_exercises(&mut self, elm: &String) -> String {
        let mut lines: Vec<String> = elm.lines().map(|s| s.to_string()).collect();
        let binding = lines.clone();

        /* Wrap exercises inside Exercises component */
        /* Right now this works only if there are consuctive exercises */
        let mut exercises = binding
            .iter()
            .enumerate()
            .filter(|(_, line)| line.trim() == "|> Exercise");

        if let Some(exo) = exercises.nth(0) {
            // add prop line which is like labels=vec!["0", "1", "2", "3"]
            let mut props_string = "    labels=vec![\"0\"".to_string();
            for i in 1..exercises.clone().count() + 1 {
                props_string += &format!(",\"{}\"", i);
            }
            props_string += "]";

            lines.insert(exo.0 - 1, props_string);
        }
        lines.join("\n")
    }

    pub fn auto_increamental_title(
        &mut self,
        elm: String,
        tag_name: &str,
        title_label: &str,
        wrapper: Option<&str>,
        wrapper_break_on: Option<&str>,
    ) -> String {
        let mut lines: Vec<String> = elm.lines().map(|s| s.to_string()).collect();
        let binding = lines.clone();
        let mut jump = 2;

        let _ = binding
            .iter()
            .enumerate()
            .filter(|(_, line)| line.trim() == format!("|> {}", tag_name))
            .enumerate()
            .for_each(|(idx, ex)| {
                // Suppose there are not props for Example , we add title at 3rd line
                let indent = ex.1.len() - ex.1.trim_start().len();
                let mut indent_string = "    ".to_string();
                for _ in 0..indent {
                    indent_string += " ";
                }
                if let Some(wrapper) = wrapper {
                    lines.insert(
                        ex.0 + jump + idx,
                        format!("{}|> {}", indent_string, wrapper),
                    );
                    lines.insert(ex.0 + jump + 1 + idx, "".to_string());
                    indent_string += "    ";
                    jump += 2;

                    let mut i = ex.0 + jump + idx;

                    while i < lines.len()
                        && lines[i].trim() != format!("|> {}", wrapper_break_on.unwrap())
                        && (lines[i].is_empty()
                            || lines[i].chars().all(char::is_whitespace)
                            || lines[i].len() - lines[i].trim_start().len() > indent)
                    {
                        i += 1;
                    }

                    for i in ex.0 + jump + idx..i {
                        if lines[i].is_empty() || lines[i].chars().all(char::is_whitespace) {
                            continue;
                        };
                        lines[i] = format!("    {}", lines[i]);
                    }
                }
                lines.insert(
                    ex.0 + jump + idx,
                    format!("{}*{} {}.*", indent_string, title_label, idx + 1),
                );
            });
        lines.join("\n")
    }

    pub fn transform(&mut self, elm: String, start_index: usize) -> String {
        let mut output = String::new();
        let mut tag_stack: Vec<TagInfo> = Vec::new();
        let lines = elm.lines();
        let mut lines_to_skip: u32 = 0;
        let mut track_line_index;

        for (index, line) in lines.clone().enumerate() {
            if lines_to_skip > 0 {
                lines_to_skip = lines_to_skip - 1;
                continue;
            }
            track_line_index = index + start_index - self.track_line_delta;
            let trimmed_line = line.trim_start();
            let indent = get_line_indent(line, trimmed_line);
            check_indent_size(indent, track_line_index);
            if let Some(last) = tag_stack.last() {
                check_extra_spaces(indent, last.indent, track_line_index);
            }
            if trimmed_line.starts_with("|> ") {
                let tag_name = trimmed_line[3..].trim().to_string();
                tag_loop(&mut tag_stack, &mut output, &indent);
                output.push_str(&format!("<{}\n", tag_name));
                tag_stack.push(TagInfo {
                    name: tag_name.clone(),
                    indent,
                    is_self_closing: self.self_closing_tags.contains(&tag_name.as_str()),
                    in_props: true,
                });

                if tag_name == "Exercises" {
                    self.track_line_delta += 1
                }

                if tag_name == "Example" {
                    self.track_line_delta += 1
                }

                if tag_name == "Exercise" {
                    self.track_line_delta += 3
                }

                continue;
            }
            if trimmed_line.is_empty() // end of props
          && tag_stack
              .last()
              .map_or(false, |tag| tag.in_props && !tag.is_self_closing)
            {
                output.push_str(">\n\"\" ");
                if let Some(last) = tag_stack.last_mut() {
                    last.in_props = false;
                }
                continue;
            }
            if !trimmed_line.is_empty() {
                tag_loop(&mut tag_stack, &mut output, &indent);

                let last = tag_stack.last().expect(&format!(
                    "There is no parent tag . line {}",
                    track_line_index
                ));
                if last.in_props {
                    // tag props
                    let mut prop_line = line.to_string();
                    // add quotes to prop value if it's a string
                    let prop_value = line.split("=").nth(1);
                    if let Some(prop_value) = prop_value {
                        let is_number = prop_value.trim().parse::<f32>();
                        let is_bool = prop_value.trim() == "false" || prop_value.trim() == "true";
                        let is_vec = prop_value.trim().starts_with("vec[");

                        if is_number.is_err() && !is_bool && !is_vec {
                            prop_line = format!(
                                "{} = \"{}\"",
                                line.split("=").nth(0).unwrap().trim(),
                                prop_value.trim()
                            )
                        }
                    }
                    output.push_str(&format!("{}\n", prop_line));
                    continue;
                }
                // tag content
                let mut nodes: Vec<Vec<(String, bool)>> = Vec::new(); // if there is an inline element in the text , the text should be seperated to sub text before the element ( bool is added to know if the node is an element ) and the subtext after the element . it shouldn't be handeled as 1 block ,
                nodes.push(Vec::new());
                let mut text_node = String::new();
                let mut inner_lines_to_skip: u32 = 0;

                for (j, text_line) in lines.clone().skip(index).enumerate() {
                    if inner_lines_to_skip > 0 {
                        inner_lines_to_skip = inner_lines_to_skip - 1;
                        continue;
                    }

                    let inner_trimmed_line = text_line.trim_start();
                    let indent = get_line_indent(text_line, inner_trimmed_line);
                    check_indent_size(indent, track_line_index + j);
                    check_extra_spaces(
                        indent,
                        tag_stack.last().unwrap().indent,
                        track_line_index + j,
                    );
                    // break if next line is new ( not nested ) element
                    if let Some(next_line) = lines.clone().nth(index + j + 1) {
                        let next_line_trimmed = next_line.trim_start();
                        let next_line_indent = get_line_indent(next_line, next_line_trimmed);

                        if next_line_indent <= tag_stack.last().unwrap().indent
                            && next_line_trimmed.starts_with("|>")
                        {
                            if text_node != "" {
                                nodes.last_mut().unwrap().push((text_node.clone(), false));
                            }
                            break;
                        }
                    }

                    if inner_trimmed_line.is_empty() {
                        if text_node != "" {
                            nodes.last_mut().unwrap().push((text_node.clone(), false));
                            text_node = "".to_string();
                        }
                        lines_to_skip += 1;
                        nodes.push(Vec::new());
                        continue;
                    }

                    if indent < tag_stack.last().unwrap().indent {
                        if text_node != "" {
                            nodes.last_mut().unwrap().push((text_node.clone(), false));
                        }
                        break;
                    }

                    if !inner_trimmed_line.starts_with("|>") {
                        text_node += &format!(" {}", inner_trimmed_line);
                        lines_to_skip += 1;
                        continue;
                    }
                    if text_node != "" {
                        nodes.last_mut().unwrap().push((text_node, false));
                        text_node = "".to_string();
                    }

                    // handle inline element that can't be handled by delimiters ( they need props )
                    let (element, skips) =
                        self.handle_inline_element(lines.clone(), j + index, indent);

                    inner_lines_to_skip += skips;
                    lines_to_skip += skips + 1;

                    nodes
                        .last_mut()
                        .unwrap()
                        .push((format!("\"#{}r#\"", element), true));
                }
                let mut processed_text = String::new();
                for (node_idx, sub_nodes) in nodes.iter().enumerate() {
                    if sub_nodes.len() == 0 {
                        continue;
                    }
                    // subnodes are seperated with empty line
                    let mut sub_node_text = "".to_string();

                    for (sub_node_idx, (text, is_element)) in sub_nodes.iter().enumerate() {
                        if *is_element {
                            sub_node_text += &text;
                            continue;
                        }

                        let no_paragraph_indent = self.handle_paragraph_indent(
                            &tag_stack,
                            &nodes,
                            &sub_nodes,
                            node_idx,
                            sub_node_idx,
                            &text,
                        );

                        if no_paragraph_indent {
                            sub_node_text += &ElementText::new(&text).handle_delimeters();
                        } else {
                            let handled_text = &format!(
                                "\"#<Indent>r#\"{}\"#</Indent>r#\"",
                                &ElementText::new(&text).handle_delimeters()
                            );
                            sub_node_text += handled_text;
                        }
                    }

                    let auto_wrapper = self.auto_wrappers.iter().find(|wrapper| {
                        wrapper
                            .tags
                            .contains(&tag_stack.last().unwrap().name.as_str())
                    });

                    if let Some(a_w) = auto_wrapper {
                        if a_w.enable_manual_wrap
                            && get_tag_from_string(&sub_node_text) == a_w.wrap_children_with
                        {
                            processed_text += sub_node_text.as_str();
                            continue;
                        }

                        let no_padding = self
                            .no_x_padding_tags
                            .contains(&tag_stack.last().unwrap().name.as_str());

                        processed_text += &format!(
                            "\"#<{} {}>r#\"{}\"#</{}>r#\"",
                            a_w.wrap_children_with,
                            if no_padding { "no_padding = true" } else { "" },
                            sub_node_text,
                            a_w.wrap_children_with
                        );
                        continue;
                    }
                    processed_text += sub_node_text.as_str();
                }
                nodes = vec![];
                output.push_str(&concat_ignore_spaces("r#\"", &processed_text, "\"#\n"));
            }
        }

        while let Some(last_tag_info) = tag_stack.pop() {
            if last_tag_info.is_self_closing {
                output.push_str("/>\n");
                continue;
            }
            output.push_str(&format!("</{}>\n", last_tag_info.name));
        }

        output
            .replace("\n", " ")
            .replace("r#\"\"#", "")
            .replace("r#\" \"#", " ")
    }

    fn handle_inline_element(
        &mut self,
        lines: Lines<'_>,
        start_from: usize,
        initial_indent: usize,
    ) -> (String, u32) {
        let mut element = "".to_string();
        let mut end_line = start_from;
        let mut inner_indent = initial_indent + 4;
        let mut skips: u32 = 0;
        let mut prev_line = "";

        while inner_indent > initial_indent
            || lines.clone().nth(end_line).unwrap().is_empty()
            || lines
                .clone()
                .nth(end_line)
                .unwrap()
                .chars()
                .all(char::is_whitespace)
        {
            if lines.clone().nth(end_line + 1).is_none() {
                break;
            }

            let inner_line = lines.clone().nth(end_line + 1).unwrap();
            let inner_trimmed_line = inner_line.trim_start();
            inner_indent = get_line_indent(inner_line, inner_trimmed_line);

            if inner_indent > initial_indent
                || inner_line.is_empty()
                || inner_line.chars().all(char::is_whitespace)
            {
                end_line += 1;
                skips += 1;
                continue;
            }
            break;
        }
        // do not skip empty line at the end
        let prev_line = lines.clone().nth(end_line).unwrap();

        if prev_line.is_empty() || prev_line.chars().all(char::is_whitespace) {
            skips -= 1
        }

        let mut i = start_from;
        for i in start_from..=end_line {
            element += &(lines.clone().nth(i).unwrap().to_string() + "\n");
        }
        element += &("".to_string() + "\n");
        element = self.transform(element, start_from);
        (element, skips)
    }

    fn handle_paragraph_indent(
        &self,
        tag_stack: &Vec<TagInfo>,
        nodes: &Vec<Vec<(String, bool)>>,
        sub_nodes: &Vec<(String, bool)>,
        node_idx: usize,
        sub_node_idx: usize,
        text: &str,
    ) -> bool {
        // For |> Indent the emitter’s rule is that a paragraph has an indent by default except:
        /* ------------- */
        // the first paragraph of every section, exercise, or example does not have an indent
        if self
            .tags_with_no_indents
            .contains(&tag_stack.last().unwrap().name.as_str())
        {
            return true;
        }
        if node_idx == 0
            && sub_node_idx == 0
            && self
                .tags_with_non_indent_first_child
                .contains(&tag_stack.last().unwrap().name.as_str())
        {
            return true;
        }
        // paragraph starting with xx. ( a note or point in a list )
        // deprecated , Use |> Pause before the paragraph
        /* let re = Regex::new(r"^ *\w+(\.|[\s_*])").unwrap();
        if re.is_match(text) {
            return true;
        } */

        // $$, __,  |_ paragraphs
        let centering_delimiters = vec!["$$", "__", "|_", "_|"];
        if text.len() > 4 && centering_delimiters.contains(&&get_slice(text, 1, 3).unwrap()) {
            return true;
        }

        // a paragraph that follows a paragraph ending with the $$, __,  |_ delimeter does not have an indent
        if node_idx > 0 {
            if let Some(first_prev_sub_node) = nodes[node_idx - 1].first() {
                let prev_text = &first_prev_sub_node.0;
                if prev_text.len() > 1
                    && centering_delimiters.contains(&&prev_text[prev_text.len() - 2..])
                {
                    return true;
                }
            }
        }
        // a paragraph that follows tags defined in tags_before_non_indents
        let mut prev_tag = "".to_string();
        if sub_node_idx > 0 {
            prev_tag = get_tag_from_string(sub_nodes[sub_node_idx - 1].0.as_str());
        }
        if node_idx > 0 {
            prev_tag = get_tag_from_string(nodes[node_idx - 1].last().unwrap().0.as_str());
        }
        if self.tags_before_non_indents.contains(&prev_tag.as_str()) {
            return true;
        }

        false
    }
}
