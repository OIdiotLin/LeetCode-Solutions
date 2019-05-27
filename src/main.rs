#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod problem;

use std::env;
use std::fs;
use std::path::{Path};
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("请提供题号");
    }
    let id = &args[1];
    let id = id.parse::<u32>().expect(&format!("not a number: {}", id));

    let problem = problem::get_problem(id)
        .expect(&format!("problem #{} not found", id));
    let code = problem.code_definition.iter()
        .filter(|&d| { d.value == "rust" })
        .next()
        .expect("该问题不支持 rust");

    let file_name = format!("n{:04}_{}", id, problem.title_slug.replace("-", "_"));
    let file_path = Path::new("./src/solutions").join(format!("{}.rs", file_name));
    if file_path.exists() {
        panic!("文件已存在");
    }

    let template = fs::read_to_string("./template.rs").unwrap();
    let source = template
        .replace("__PROBLEM_TITLE__", &problem.title)
        .replace("__PROBLEM_DESC__", &build_desc(&problem.content))
        .replace("__PROBLEM_DEFAULT_CODE__", &code.default_code)
        .replace("__PROBLEM_ID__", &format!("{}", id))
        .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    file.write_all(source.as_bytes()).unwrap();
    drop(file);

    let mut lib_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./src/solutions/mod.rs")
        .unwrap();
    writeln!(lib_file, "mod {};", file_name);
}

fn parse_extra_use(code: &str) -> String {
    let mut extra_use_line = String::new();
    // a linked-list problem
    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("\nuse super::super::utils::linked_list::{ListNode, to_list};")
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("\nuse super::super::utils::tree::{TreeNode, to_tree};")
    }
    if code.contains("pub struct Point") {
        extra_use_line.push_str("\nuse super::super::utils::point::Point;")
    }
    extra_use_line
}

fn build_desc(content: &str) -> String {
    // TODO: fix this shit
    content
        .replace("<strong>", "")
        .replace("</strong>", "")
        .replace("<em>", "")
        .replace("</em>", "")
        .replace("</p>", "")
        .replace("<p>", "")
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("<pre>", "")
        .replace("</pre>", "")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", "")
        .replace("</li>", "")
        .replace("<code>", "")
        .replace("</code>", "")
        .replace("<i>", "")
        .replace("</i>", "")
        .replace("<sub>", "")
        .replace("</sub>", "")
        .replace("</sup>", "")
        .replace("<sup>", "^")
        .replace("&nbsp;", " ")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"")
        .replace("&minus;", "-")
        .replace("&#39;", "'")
        .replace("\n\n", "\n")
        .replace("\n", "\n * ")
}
