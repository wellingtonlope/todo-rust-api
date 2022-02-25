use chrono::{DateTime, Utc};
use std::io::{Error, ErrorKind, Result};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn new(title: String, description: String, date: Option<DateTime<Utc>>) -> Result<Todo> {
        if title.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "Title musn't be empty"));
        }

        Ok(Todo {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            created_date: Todo::get_date_or_current(date),
            updated_date: None,
        })
    }

    fn get_date_or_current(date: Option<DateTime<Utc>>) -> DateTime<Utc> {
        if date.is_some() {
            return date.unwrap();
        }

        Utc::now()
    }

    pub fn update(
        &mut self,
        title: String,
        description: String,
        date: Option<DateTime<Utc>>,
    ) -> Option<Error> {
        if title.is_empty() {
            return Some(Error::new(ErrorKind::InvalidInput, "Title musn't be empty"));
        }

        self.title = title;
        self.description = description;
        self.updated_date = Some(Todo::get_date_or_current(date));

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_todo() {
        let expected_title = String::from("title");
        let expected_description = String::from("description");
        let expected_created_date = Utc::now();

        let got = Todo::new(
            expected_title.clone(),
            expected_description.clone(),
            Some(expected_created_date.clone()),
        );

        if got.is_err() {
            panic!("Shouldn't be an error!")
        }

        let got = got.unwrap();

        assert_ne!(String::new(), got.id);
        assert_eq!(expected_title, got.title);
        assert_eq!(expected_description, got.description);
        assert_eq!(expected_created_date, got.created_date);
        assert_eq!(None, got.updated_date);
    }

    #[test]
    fn shouldnt_create_todo() {
        let got = Todo::new(String::new(), String::new(), None);

        if got.is_ok() {
            panic!("Shouldn't be OK!")
        }

        assert_eq!(ErrorKind::InvalidInput, got.unwrap_err().kind());
    }

    #[test]
    fn should_update_todo() {
        let mut todo = Todo::new(String::from("title"), String::from("description"), None).unwrap();

        let expected_title = String::from("update_title");
        let expected_description = String::from("update_description");
        let expected_updated_date = Some(Utc::now());

        let got_err = todo.update(
            expected_title.clone(),
            expected_description.clone(),
            expected_updated_date.clone(),
        );

        if got_err.is_some() {
            panic!("Shouldn't be an error!")
        }

        assert_eq!(expected_title, todo.title);
        assert_eq!(expected_description, todo.description);
        assert_eq!(expected_updated_date, todo.updated_date);
    }

    #[test]
    fn shouldnt_update_todo() {
        let mut todo = Todo::new(String::from("title"), String::from("description"), None).unwrap();

        let got_err = todo.update(String::new(), String::new(), None);

        if got_err.is_none() {
            panic!("Should be an error!")
        }

        assert_eq!(ErrorKind::InvalidInput, got_err.unwrap().kind());
    }
}
