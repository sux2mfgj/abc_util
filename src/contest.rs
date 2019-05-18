use crate::extracter::atcoder;
use crate::task;

pub struct Contest {
    pub title: Option<String>,
    tasks: Option<Vec<task::Task>>,
    pub current_task_index: usize,
}

impl Contest {
    pub fn new() -> Contest {
        Contest {
            title: None,
            tasks: None,
            current_task_index: 0,
        }
    }

    pub fn set_title(&mut self, title: String) -> bool {
        let domain = "atcoder.jp".to_string();
        let path = format!("/contests/{0}/tasks", title).to_string();

        let result = atcoder::get_tasks(&domain, &path);
        let lang_ja = atcoder::Lang::Ja;

        let mut task_result = vec![];
        if let Some(tasks) = result {
            for task in tasks {
                task_result.push(task::Task::new(task.title, task.link, lang_ja));
            }

            self.tasks = Some(task_result);
            true
        } else {
            false
        }
    }

    pub fn get_task(&mut self) -> Option<&task::Task> {
        if let Some(task_vec) = &mut self.tasks {
            if self.current_task_index >= task_vec.len()
            {
                self.current_task_index = 0;
                None
            }
            else
            {
                task_vec[self.current_task_index].complete();
                let ret_task :&task::Task = &task_vec[self.current_task_index];
                Some(&ret_task)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_contest() {
        let _contest = Contest::new();
    }

    #[test]
    fn set_title() {
        let mut contest = Contest::new();
        let title = "abc120".to_string();

        println!("checking...");
        let is_exist = contest.set_title(title);
        assert!(is_exist);
    }

    #[test]
    fn get_task() {
        let mut contest = Contest::new();
        let title = "abc120".to_string();

        let is_exist = contest.set_title(title);
        assert!(is_exist);

        if let Some(t) = contest.get_task() {
            println!("{}", t.title);
        } else {
            assert!(false);
        }
    }

}

/*
use reqwest;

use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};

use crate::task;

#[derive(Debug, PartialEq)]
pub struct Contest {
    pub name: Option<String>,
    pub tasks: Vec<task::Task>,
    pub current_task_index: usize,
}

impl Contest {
    pub fn new() -> Contest {
        Contest {
            name: None,
            tasks: vec![],
            current_task_index: 0,
        }
    }

    pub fn set_name_and_get_tasks(
        &mut self,
        domain: &String,
        path: &String,
    ) -> Option<Vec<String>> {
        let url = &format!("https://{0}{1}", domain, path);
        println!("getting from {}", url);

        if let Ok(mut response) = reqwest::get(url) {
            // TODO
            let html_body = response.text().unwrap();
            let document = Html::parse_document(&html_body);

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

            if links.len() == 0 {
                None
            } else {
                let mut result = vec![];
                for i in 0..links.len() {
                    if i % 2 == 1 {
                        result.push(format!("{}: {}", names[i - 1].clone(), names[i]).to_string());
                        self.tasks.push(task::Task::new(
                            names[i].clone(),
                            domain.to_string(),
                            links[i].to_string(),
                        ));
                    }
                }

                Some(result)
            }
        } else {
            None
        }
    }

    pub fn show_current_task_info(&mut self)
    {
        if None == self.name {
            //Err("set the contest_name first".to_string())
        }

        if self.tasks.len() == 0 {
            //Err("set the contest_name first".to_string())
        }

        let current = &self.tasks[self.current_task_index];

    }
}
*/
