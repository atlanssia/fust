pub mod pipeline;

pub struct Pipeline {
    name: String,
    source: Box<dyn Source>,
    processors: Vec<Box<dyn Processor>>,
}