use std::collections::HashMap;
use std::result::Result;
use chrono::prelude::*;
use colored::Colorize;

struct Todo {
    map: HashMap<String, HashMap<String, String>>,
}

impl Todo {
    fn new(db_path: Option<&str>) -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(db_path.unwrap_or("db.json"))?;

        // serialize json as HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        let mut map = HashMap::new();
        let current_time = Utc::now().to_string();
        map.insert("status".to_string(), "Todo".to_string());
        map.insert("enqueued_at".to_string(), current_time.to_string());

        self.map.insert(key, map);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn list(&self) {
        for (key, value) in self.map.iter() {
            println!("{}:", key.bold().yellow());
            for (k, v) in value {
                if k == "status" {
                    if v == "Done" {
                        println!("\t{}: {}", k.bold().purple(), v.green().bold());
                    } else if v == "Started" {
                        println!("\t{}: {}", k.bold().purple(), v.bright_cyan().bold());
                    } else {
                        println!("\t{}: {}", k.bold().purple(), v.yellow().bold());
                    }

                } else {
                    println!("\t{}: {}", k.bold().purple(), v.green());
                }
            }
        }

        println!("total: {}", self.map.len().to_string().bold().yellow());
    }

    fn start(&mut self, key: &str) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => {
                *v = {
                    let mut map = v.clone();
                    let current_time = Utc::now().to_string();
                    map.insert("status".to_string(), "Started".to_string());
                    map.insert("started_at".to_string(), current_time.to_string());
                    map
                };
                Some(())
            },
            None => None,
        }
    }

    fn complete(&mut self, key: &str) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => {
                *v = {
                    let mut map = v.clone();
                    let current_time = Utc::now().to_string();

                    map.insert("status".to_string(), "Done".to_string());
                    map.insert("finished_at".to_string(), current_time.to_string());
                    map
                };
                Some(())
            },
            None => None,
        }
    }

    fn list_by_status(&self, status: &str) {
       if status == "all" {
           self.list();
       } else {
           for (key, value) in self.map.iter() {
               let mut count = 0;
               if value.get("status").unwrap() == status {
                   println!("{}:", key.bold().yellow());
                   for (k, v) in value {
                       count += 1;
                       println!("\t{}: {}", k.bold().purple(), v.green());
                   }
                   println!("total: {}", count.to_string().bold().yellow());
               }
           }
       }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new(None).expect("Initialisation of db failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "start" {
        match todo.start(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }

    } else if action == "list" {
        todo.list_by_status(&item);
    }
}

mod tests {
    #[test]
    fn test_new() {
        let todo = super::Todo::new(Some("test_db.json"));
        assert!(todo.is_ok());
    }

    #[test]
    fn test_insert() {
        let mut todo = super::Todo::new(Some("test_db.json")).unwrap();
        todo.insert("test".to_string());
        assert_eq!(todo.map.len(), 1);
        assert!(todo.map.contains_key("test"));
        assert_eq!(todo.map.get("test").unwrap().get("status").unwrap(), "Todo");
    }

    #[test]
    fn test_save() {
        let mut todo = super::Todo::new(Some("test_db.json")).unwrap();
        todo.insert("test".to_string());
        assert!(todo.save().is_ok());
    }

    #[test]
    fn test_complete() {
        let mut todo = super::Todo::new(Some("test_db.json")).unwrap();
        todo.insert("test".to_string());
        todo.complete("test");
        assert_eq!(todo.map.len(), 1);
        assert!(todo.map.contains_key("test"));
        assert_eq!(todo.map.get("test").unwrap().get("status").unwrap(), "Done");
    }

    #[test]
    fn test_start() {
        let mut todo = super::Todo::new(Some("test_db.json")).unwrap();
        todo.insert("test".to_string());
        todo.start("test");
        assert_eq!(todo.map.len(), 1);
        assert!(todo.map.contains_key("test"));
        assert_eq!(todo.map.get("test").unwrap().get("status").unwrap(), "Started");
    }

    #[test]
    fn test_list() {
        let mut todo = super::Todo::new(Some("test_db.json")).unwrap();
        todo.insert("test".to_string());
        todo.insert("test2".to_string());
        todo.list();
        assert_eq!(todo.map.len(), 2);
    }
}
