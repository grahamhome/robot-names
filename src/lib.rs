use rand::{thread_rng, Rng};
use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(static ROBOT_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

#[derive(Default)]
pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Robot(Robot::new_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    fn new_name() -> String {
        ROBOT_NAMES.with(|un| {
            let mut rng = thread_rng();
            let mut used_names = un.borrow_mut();
            loop {
                let name = format!(
                    "{}{:03}",
                    (0..2).map(|_| rng.gen_range('A'..='Z')).collect::<String>(),
                    rng.gen_range(0..1000)
                );
                if !used_names.contains(&name) {
                    used_names.insert(name.clone());
                    return name;
                }
            }
        })
    }

    pub fn reset_name(&mut self) {
        ROBOT_NAMES.with(|un| un.borrow_mut().remove(&self.0));
        self.0 = Robot::new_name();
    }
}

#[test]
fn test_many_different_robots_have_different_names() {
    use crate::*;
    use std::collections::HashSet;
    // In 3,529 random robot names, there is ~99.99% chance of a name collision
    let vec: Vec<_> = (0..3529).map(|_| Robot::new()).collect();
    let set: HashSet<_> = vec.iter().map(|robot| robot.name()).collect();
    let number_of_collisions = vec.len() - set.len();
    assert_eq!(number_of_collisions, 0);
}
