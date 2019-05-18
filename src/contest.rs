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

            self.title = Some(title);
            self.tasks = Some(task_result);
            true
        } else {
            false
        }
    }

    pub fn get_task(&mut self) -> Option<&task::Task> {
        if let Some(task_vec) = &mut self.tasks {
            if self.current_task_index >= task_vec.len() {
                self.current_task_index = 0;
                None
            } else {
                task_vec[self.current_task_index].complete();
                let ret_task: &task::Task = &task_vec[self.current_task_index];
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
