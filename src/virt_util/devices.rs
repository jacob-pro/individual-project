#[derive(Clone, Debug)]
pub struct Disk {}

#[derive(Clone, Debug)]
pub enum Device {
    Disk(Disk),
}
