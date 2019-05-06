use std::process::{Command, Stdio};
use std::io::Write;
use std::str::FromStr;

#[macro_use]
extern crate clap;
use clap::App;

use reqwest;

use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};

#[derive(Debug)]
struct SampleIO {
    input: String,
    output: String,
}

#[derive(Debug)]
struct Task {
    problem_statement: String,
    sample_ios: Vec<SampleIO>,
    lang:   String,
}

impl Task {
    fn new(html_body: String, lang: String) -> Task {
        let document = Html::parse_document(&html_body);
        let lang = &format!("span.lang-{}", lang);
        /* let sel_lang = Selector::parse(&format!("span.lang-{}", lang)).unwrap(); */
        let sel_lang = Selector::parse(lang).unwrap();;
        let task_statement_root = document.select(&sel_lang).next().unwrap();

        let sample_ios = Task::get_samples(&task_statement_root);

        Task {
            problem_statement: Task::get_problem_statement(&task_statement_root),
            sample_ios: sample_ios,
            lang: lang.to_string(),
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

struct Verbose {
    level: u64
}

impl Verbose {
    fn new(level: u64) -> Verbose {
        if level >= 1
        {
            println!("- current verboes level:  {}", level);
        }
        Verbose{
            level: level
        }
    }

    fn output(&self, text: &str) {
        print!("{}", text);
    }

    fn info(&self, text: &str) {
        if self.level >= 1
        {
            print!("{}", text);
        }
    }

    fn debug(&self, text: &str) {
        if self.level >= 2
        {
            print!("{}", text);
        }
    }
}

enum TaskLevels {
    A,
    B,
    C,
    D,
}

impl FromStr for TaskLevels {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "a" => Ok(TaskLevels::A),
            "B" | "b" => Ok(TaskLevels::B),
            "C" | "c" => Ok(TaskLevels::C),
            "D" | "d" => Ok(TaskLevels::D),
            _ => Err("No match")
        }
    }
}

fn main() {
    /*
     * parse the command line arugmnet
     */
    let yaml = load_yaml!("../arguments.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let verbose_level = matches.occurrences_of("verbose");
    let verbose = Verbose::new(verbose_level);

    let cmd = matches.value_of("exec").unwrap_or("./a.out");
    let abc_number = matches.value_of("number").unwrap();

    let t = value_t!(matches, "level", TaskLevels).unwrap();
    let task_level = match t {
        TaskLevels::A => "a",
        TaskLevels::B => "b",
        TaskLevels::C => "c",
        TaskLevels::D => "d",
    };

    let task_name = &format!("abc{0:<03}", abc_number.parse::<i32>().unwrap());
    let url = &format!(
            "https://atcoder.jp/contests/{0}/tasks/{0}_{1}",
            task_name, task_level);

    verbose.info(&format!("- execution file : {}\n", cmd));
    verbose.info(&format!("- problem number : {}\n", task_name));
    verbose.info(&format!("- http request url:\n{}\n", url));

    /*
     *  request HTTP GET to the url
     */
    let mut response = reqwest::get(url).unwrap();

    verbose.info(&format!("- status         : {}\n", response.status()));

    let buffer = response.text().unwrap();
    verbose.debug(&format!("- html text      :\n{}\n", buffer));

    let task = Task::new(buffer, "ja".to_string());
    verbose.debug(&format!("- langurage info : {}\n", task.lang));

    verbose.output(&format!("- problem        :\n{}\n", task.problem_statement));

    let mut exit_code: i32 = 0;
    if matches.is_present("statement") {
        std::process::exit(exit_code);
    }

    for (i, sample) in task.sample_ios.iter().enumerate() {

        verbose.output(&format!("--- sample{:<02} ---\n", i));

        let mut process = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().ok().expect("failed to start the program");
        process.stdin.as_mut().unwrap().write(sample.input.as_bytes()).unwrap();
        let output = process.wait_with_output().unwrap();
        let result_str = String::from_utf8(output.stdout).unwrap();
        if result_str == sample.output
        {
            /* println!("passed"); */
            verbose.output(&format!("passed!"));
        }
        else
        {
            verbose.output(&format!("failed"));
            /* println!("faild"); */
            verbose.output(&format!("correct answer is\n{}", sample.output));
            verbose.output(&format!("your answer is \n{}", result_str));
            exit_code += 1;
        }
    }
    std::process::exit(exit_code);
}
