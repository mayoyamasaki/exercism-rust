use std::collections::HashMap;

pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {roster: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.roster.contains_key(&grade) {
            let v: Vec<String> = Vec::new();
            self.roster.insert(grade, v);
        }

        if let Some(v) =  self.roster.get_mut(&grade) {
            v.push(student.to_string());
            v.sort();
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut v = self.roster.keys().cloned().collect::<Vec<u32>>();
        v.sort();
        v
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if let Some(v) = self.roster.get(&grade) {
            Some(v.iter().cloned().collect::<Vec<String>>())
        } else {
            None
        }
    }
}
