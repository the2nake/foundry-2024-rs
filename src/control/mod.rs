pub mod pid;

pub trait Settles {
    fn settled(&self) -> bool;
}
