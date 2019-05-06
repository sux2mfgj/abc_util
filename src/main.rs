use reqwest;
use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};
use std::process::{Command, Stdio};
use std::io::Write;

#[derive(Debug)]
struct SampleIO {
    input: String,
    output: String,
}

#[derive(Debug)]
struct Task {
    problem_statement: String,
    sample_ios: Vec<SampleIO>,
}

impl Task {
    fn new(html_body: String, lang: String) -> Task {
        let document = Html::parse_document(&html_body);
        let sel_lang = Selector::parse(&format!("span.lang-{}", lang)).unwrap();
        let task_statement_root = document.select(&sel_lang).next().unwrap();

        let sample_ios = Task::get_samples(&task_statement_root);

        Task {
            problem_statement: Task::get_problem_statement(&task_statement_root),
            sample_ios: sample_ios,
        }
    }

    fn get_problem_statement(html: &ElementRef) -> String {
        let selector_div1 = Selector::parse("div.part").unwrap();
        let selector_p = Selector::parse("p").unwrap();

        let problem_v: Vec<_> = html
            .select(&selector_div1)
            .flat_map(|item| item.select(&selector_p))
            .map(|item| item.inner_html())
            .collect();

        problem_v
            .concat()
            .replace("<var>", "")
            .replace("</var>", "")
            .replace("<br>", "")
            .replace("<code>", "")
            .replace("</code>", "")
    }

    fn get_samples(html: &ElementRef) -> Vec<SampleIO> {
        let sel_sample = Selector::parse("div.part").unwrap();
        let sel_section = Selector::parse("section").unwrap();
        let sel_pre = Selector::parse("pre").unwrap();

        let samples: Vec<_> = html
            .select(&sel_sample)
            .flat_map(|item| item.select(&sel_section))
            .flat_map(|item| item.select(&sel_pre))
            /* .flat_map(|item| item.select(&sel_presample)) */
            .map(|item| item.inner_html())
            .collect();

        let mut iter = samples[1..].iter();

        let mut io_samples = Vec::new();
        while let Some(input) = iter.next() {
            if let Some(output) = iter.next() {
                let sample = SampleIO {
                    input: input.to_string(),
                    output: output.to_string(),
                };
                io_samples.push(sample);
            } else {
                assert!(false, "I/Os must be coresponded.");
            }
        }

        io_samples
    }
}

fn main() {
    let cmd = "./a.out";

    let mut response = reqwest::get("https://atcoder.jp/contests/abc088/tasks/abc088_a").unwrap();

    println!("status {}", response.status());
    let buffer = response.text().unwrap();

    let task = Task::new(buffer, "ja".to_string());
    println!("{}", task.problem_statement);
    for (i, sample) in task.sample_ios.iter().enumerate() {

        println!("--- sample{} ---", i);

        let mut process = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().ok().expect("failed to start the program");
        process.stdin.as_mut().unwrap().write(sample.input.as_bytes()).unwrap();
        let output = process.wait_with_output().unwrap();
        let result_str = String::from_utf8(output.stdout).unwrap();
        if result_str == sample.output
        {
            println!("passed");
        }
        else
        {
            println!("faild");
            print!("correct answer is\n{}", sample.output);
            print!("your answer is \n{}", result_str);
        }
    }
}
