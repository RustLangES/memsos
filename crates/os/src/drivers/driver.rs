pub trait Driver {
    type ReadOutput;

    fn read(&self) -> Self::ReadOutput;
}
