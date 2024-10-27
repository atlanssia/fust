
pub struct Transform {
    pipeline: Pipeline,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            pipeline: Pipeline::new(),
        }
    }

    pub fn add_processor(&mut self, processor: Processor) {
        self.pipeline.add_processor(processor);
    }

    pub fn execute(&self, input: Vec<String>) -> Vec<String> {
        self.pipeline.execute(input)
    }
}