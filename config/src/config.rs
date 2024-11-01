// ConfigSpec is the configuration file for a plugin
pub struct ConfigSpec {
    pub name: String,
    pub description: String,
    pub version: String,
    pub pipeline: Vec<String>,
}

impl ConfigSpec {
    pub fn new(name: String, description: String, version: String, pipeline: Vec<String>) -> Self {
        ConfigSpec {
            name,
            description,
            version,
            pipeline,
        }
    }
}