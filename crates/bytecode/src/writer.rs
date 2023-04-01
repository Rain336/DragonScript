use crate::Instruction;

pub struct ByteCode(Box<[Instruction]>);

pub struct ByteCodeWriter(Vec<Instruction>);

impl ByteCodeWriter {
    pub fn new() -> Self {
        ByteCodeWriter(Vec::new())
    }

    pub fn finish(self) -> ByteCode {
        ByteCode(self.0.into_boxed_slice())
    }
}

impl Default for ByteCodeWriter {
    fn default() -> Self {
        ByteCodeWriter::new()
    }
}
