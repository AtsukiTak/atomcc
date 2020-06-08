pub mod op;
pub mod reg;

pub use op::*;
pub use reg::*;

pub trait Instruction {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write;

    fn print(&self) {
        self.write(&mut std::io::stdout()).unwrap()
    }
}
