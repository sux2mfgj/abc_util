pub mod atcoder {
    use reqwest;

    use scraper::{Html, Selector};
    use scraper::element_ref::ElementRef;

    use crate::task;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Lang {
        Ja,
        En,
    }

    #[derive(Debug, PartialEq)]
    pub struct TaskData {
        pub title: String,
        pub link: String,
    }

    pub fn get_tasks(domain: &String, path: &String) -> Option<Vec<TaskData>> {
        //println!("{}", domain);
        //println!("{}", path);
        let url = &format!("https://{0}{1}", domain, path);
        println!("getting from {}", url);

        if let Ok(mut response) = reqwest::get(url) {
            let html_body = response.text().unwrap();
            let document = Html::parse_document(&html_body);
            //println!("{:?}", html_body);

            // TODO refactoring
            let task_sel = Selector::parse("td").unwrap();
            let task_a = Selector::parse("a").unwrap();
            let links: Vec<_> = document
                .select(&task_sel)
                .flat_map(|item| item.select(&task_a))
                .map(|item| item.value().attr("href").unwrap())
                .collect();

            let names: Vec<_> = document
                .select(&task_sel)
                .flat_map(|item| item.select(&task_a))
                .map(|item| item.inner_html())
                .collect();

            if links.len() == 0 || names.len() == 0 {
                println!("cannot found any tasks in the link.");
                println!("{0:?} {1:?}", links, names);
                None
            } else {
                let mut result: Vec<TaskData> = vec![];
                for i in 0..links.len() {
                    if i % 2 == 1 {
                        result.push(TaskData {
                            title: format!("{}: {}", names[i - 1].clone(), names[i]).to_string(),
                            link: format!("https://{}{}", domain, links[i].to_string()),
                        });
                    }
                }
                println!("done");
                Some(result)
            }
        } else {
            None
        }
    }

    pub fn set_task_info(task: &mut task::Task) -> bool
    {

        let mut response = reqwest::get(&task.url).unwrap();
        let html_body = response.text().unwrap();

        let document = Html::parse_document(&html_body);
        let lang = &format!("span.lang-ja");

        let sel_lang = Selector::parse(lang).unwrap();;
        let task_statement_root = document.select(&sel_lang).next().unwrap();

        let (sample_ios, io_style) = get_samples(&task_statement_root);
        let p_state = get_problem_statement(&task_statement_root);

        task.sample_ios = Some(sample_ios);
        task.io_style = Some(io_style);
        task.problem_statement = Some(p_state);

        true
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

    fn get_samples(html: &ElementRef) -> (Vec<task::SampleIO>, String) {
        let sel_sample = Selector::parse("div.part").unwrap();
        let sel_section = Selector::parse("section").unwrap();
        let sel_pre = Selector::parse("pre").unwrap();

        let samples: Vec<_> = html
            .select(&sel_sample)
            .flat_map(|item| item.select(&sel_section))
            .flat_map(|item| item.select(&sel_pre))
            .map(|item| item.inner_html())
            .collect();

        let io_style = &samples[0].replace("<var>", "").replace("</var>", "");
        let mut iter = samples[1..].iter();

        let mut io_samples = Vec::new();
        while let Some(input) = iter.next() {
            if let Some(output) = iter.next() {
                let sample = task::SampleIO {
                    input: input.to_string(),
                    output: output.to_string(),
                };
                io_samples.push(sample);
            } else {
                assert!(false, "I/Os must be coresponded.");
            }
        }

        (io_samples, io_style.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_of_test() {
        assert!(true);
    }

    #[test]
    #[should_panic]
    fn failed_test() {
        assert!(false)
    }

    #[test]
    fn get_tasks() {
        //https://atcoder.jp/contests/abc125/
        let domain = "atcoder.jp".to_string();
        let path = "/contests/abc125/tasks".to_string();

        if let Some(tasks) = atcoder::get_tasks(&domain, &path) {
            println!("{:?}", tasks);

            assert_eq!(
                tasks,
                vec![
                    atcoder::TaskData {
                        title: "A: Biscuit Generator".to_string(),
                        link: "https://atcoder.jp/contests/abc125/tasks/abc125_a".to_string(),
                    },
                    atcoder::TaskData {
                        title: "B: Resale".to_string(),
                        link: "https://atcoder.jp/contests/abc125/tasks/abc125_b".to_string(),
                    },
                    atcoder::TaskData {
                        title: "C: GCD on Blackboard".to_string(),
                        link: "https://atcoder.jp/contests/abc125/tasks/abc125_c".to_string(),
                    },
                    atcoder::TaskData {
                        title: "D: Flipping Signs".to_string(),
                        link: "https://atcoder.jp/contests/abc125/tasks/abc125_d".to_string(),
                    },
                ]
            );
        } else {
            panic!("failed to get the tasks");
        }
    }

    #[test]
    fn get_tasks_invalid_url() {
        //https://atcoder.jp/contests/abc125/
        let domain = "atcoder.jp".to_string();
        let path = "/contests/agc125/tasks".to_string();

        let result = atcoder::get_tasks(&domain, &path);

        assert_eq!(result, None);
    }

    //TODO add more tests for older contests.

#[test]
    fn set_task_info() {
        let url = "https://atcoder.jp/contests/abc125/tasks/abc125_a".to_string();
        // TODO
    }
}
