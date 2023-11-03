pub mod command;
pub mod strategy;

fn main() {
    let mut schema = command::Schema::new();

    let cmd = Box::new(command::CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(command::AddField);
    schema.add_migration(cmd);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
