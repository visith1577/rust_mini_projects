pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

pub struct CreateTable;
impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create table"
    }

    fn rollback(&self) -> &str {
        "drop table"
    }
}

pub struct AddField;
impl Migration for AddField {
    fn execute(&self) -> &str {
        "add field"
    }

    fn rollback(&self) -> &str {
        "remove field"
    }
}

pub struct Schema {
    command: Vec<Box<dyn Migration>>,
}

impl Schema {
    pub fn new() -> Self {
        Self { command: Vec::new() }
    }

    pub fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.command.push(cmd);
    }

    pub fn execute(&self) -> Vec<&str> {
        self.command.iter().map(|cmd| cmd.execute()).collect()
    }

    pub fn rollback(&self) -> Vec<&str> {
        self.command.iter().rev().map(|cmd| cmd.rollback()).collect()
    }
}
