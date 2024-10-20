#[derive(Debug, Default, Clone)]
pub struct TagInfo {
    pub id: usize,
    pub name: String,
    pub indent: usize,
    pub is_self_closing: bool,
    pub in_props: bool,
}

pub fn tag_stack_pop(tag_stack: &mut Vec<TagInfo>, indent: &usize) {
    while let Some(last_tag_info) = tag_stack.last() {
        if *indent <= last_tag_info.indent {
            /* if last_tag_info.is_self_closing {
                output.push_str("/>\n");
            } else {
                output.push_str(&format!("</{}>\n", last_tag_info.name));
            } */
            tag_stack.pop();
        } else {
            break;
        }
    }
}
