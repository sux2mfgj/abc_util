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
