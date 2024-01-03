pub trait Similarity {
    fn name(&self) -> &str;
    fn start (&self, args:  &str) -> String;
}

