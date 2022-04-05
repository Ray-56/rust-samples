use anyhow::{Result};
use jwalk::WalkDirGeneric;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender};

use crate::{Config, Message, PathItem};

pub fn search(entry: PathBuf, config: Config, tx: Sender<Message>) -> Result<()> {
    let walk_dir = WalkDirGeneric::<((), Option<Option<String>>)>::new(entry.clone())
        .skip_hidden(false)
        .process_read_dir(move |_depth, _path, _state, children| {
            let mut checker = Checker::new(&config);
            for dir_entry in children.iter().flatten() {
                if let Some(name) = dir_entry.file_name.to_str() {
                    checker.check(name);
                }
            }
        });

    Ok(())
}

pub fn ls(rx: Receiver<Message>) -> Result<()> {
    for message in rx {
        match message {
            Message::AddPath(path) => {
                println!("{}", path.path.display());
            }
            Message::DoneSearch => break,
            _ => {}
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Checker<'a, 'b> {
    matches: HashMap<&'a str, (HashSet<&'b str>, HashSet<&'b str>)>,
    config: &'a Config,
}

impl<'a, 'b> Checker<'a, 'b> {
    fn new(config: &'a Config) -> Self {
        Self {
            config,
            matches: Default::default(),
        }
    }

    fn check(&mut self, name: &'b str) {
        for rule in &self.config.rules {
            let (purge_matches, check_matches) = self.matches.entry(rule.get_id()).or_default();
            if rule.test_purge(name) {
                purge_matches.insert(name);
            }
            if rule.test_check(name) {
                check_matches.insert(name);
            }
        }
    }

    fn to_matches(&self) -> HashMap<String, &'a str> {
        let mut matches: HashMap<String, &'a str> = HashMap::new();
        for (rule_id, (purge_matches, check_matches)) in &self.matches {
            if !purge_matches.is_empty()
                && (!check_matches.is_empty() || self.config.is_rule_no_check(rule_id))
            {
                for name in purge_matches {
                    if !matches.contains_key(*name) {
                        matches.insert(name.to_string(), rule_id);
                    }
                }
            }
        }
        matches
    }
}

fn du(path: &Path) -> Result<u64> {
    let mut total: u64 = 0;

    for dir_entry_result in WalkDirGeneric::<((), Option<u64>)>::new(path)
        .skip_hidden(false)
        .process_read_dir(|_, _, _, dir_entry_results| {
            dir_entry_results.iter_mut().for_each(|dir_entry_result| {
                if let Ok(dir_entry) = dir_entry_result {
                    if !dir_entry.file_type.is_dir() {
                        dir_entry.client_state =
                            Some(dir_entry.metadata().map(|m| m.len()).unwrap_or_default());
                    }
                }
            })
        })
    {

    }

    Ok(total)
}
