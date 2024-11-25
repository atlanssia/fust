use std::{collections::HashMap, error::Error, fs::read_to_string};

use toml::{map::Map, Value};

use crate::field::Field;

// RdsConfig is the configuration for a PostgreSQL, MySQL, SQLite, Oracle, or Microsoft SQL Server database
#[derive(Debug, Clone)]
pub struct RdsConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

// KafkaConfig is the configuration for a Kafka connector
#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
}

// NatsConfig is the configuration for a NATS connector
#[derive(Debug, Clone)]
pub struct NatsConfig {
    pub url: String,
    pub topic: String,
}

// ConnectorConfig is the configuration for a connector
#[derive(Debug, Clone)]
pub enum ConnectorConfig {
    Rds(RdsConfig),
    Kafka(KafkaConfig),
    Nats(NatsConfig),
}

// SourceConfig is the configuration for a source connector
#[derive(Debug, Clone)]
pub struct SourceConfig {
    pub connector: ConnectorConfig,
    pub config: HashMap<String, String>,
    pub fields: Vec<Field>,
}

// SinkConfig is the configuration for a sink connector
#[derive(Debug, Clone)]
pub struct SinkConfig {
    pub connector: ConnectorConfig,
    pub config: HashMap<String, String>,
}

// ConfigSpec is the configuration specification
#[derive(Debug, Clone)]
pub struct ConfigSpec {
    pub name: String,
    pub description: String,
    pub version: String,
    pub connectors: HashMap<String, ConnectorConfig>,
    pub sources: HashMap<String, SourceConfig>,
    pub sinks: HashMap<String, SinkConfig>,
    pub pipeline: Vec<String>,
}

impl ConfigSpec {
    pub fn from_file(config_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(config_path)?;
        let value: Value = toml::from_str(&contents)?;

        // common information
        let name = value["name"].as_str().unwrap().to_string();
        let description = value["description"].as_str().unwrap().to_string();
        let version = value["version"].as_str().unwrap().to_string();

        // connectors
        let connectors_table = value["connectors"].as_table().unwrap();
        let connectors = from_connectors(connectors_table);

        // sources
        let sources_table = value["sources"].as_table().unwrap();
        let sources = from_sources(&connectors, sources_table);

        // sinks
        let sinks_table = value["sinks"].as_table().unwrap();
        let sinks = from_sinks(&connectors, sinks_table);

        // pipeline if not specified, use "system" as default
        let pipeline = value["pipeline"]
            .as_array()
            .unwrap_or(&vec![Value::String("system".to_string())])
            .iter()
            .map(|s| s.as_str().unwrap().to_string())
            .collect();

        Ok(ConfigSpec {
            name,
            description,
            version,
            connectors,
            sources,
            sinks,
            pipeline,
        })
    }
}

fn from_sinks(
    connectors: &HashMap<String, ConnectorConfig>,
    sinks_table: &Map<String, Value>
) -> HashMap<String, SinkConfig> {
    let mut sinks = HashMap::new();
    for (sink_name, sink_value) in sinks_table {
        let connector_name = sink_value["connector"].as_str().unwrap();
        let connector_config = connectors.get(connector_name).cloned().unwrap();

        let mut config = HashMap::new();
        for (key, value) in sink_value.as_table().unwrap() {
            if key != "connector" {
                config.insert(key.clone(), value.as_str().unwrap_or_default().to_string());
            }
        }
        sinks.insert(
            sink_name.clone(),
            SinkConfig {
                connector: connector_config,
                config,
            },
        );
    }
    sinks
}

fn from_sources(
    connectors: &HashMap<String, ConnectorConfig>,
    sources_table: &Map<String, Value>,
) -> HashMap<String, SourceConfig> {
    let mut sources = HashMap::new();
    for (source_name, source_value) in sources_table {
        let connector_name = source_value["connector"].as_str().unwrap();
        let connector_config = connectors.get(connector_name).cloned().unwrap();

        let mut config = HashMap::new();
        for (key, value) in source_value.as_table().unwrap() {
            if key != "connector" && key != "fields" {
                config.insert(key.clone(), value.as_str().unwrap_or_default().to_string());
            }
        }
        sources.insert(
            source_name.clone(),
            SourceConfig {
                connector: connector_config,
                config,
                fields: vec![/* ... */],
            },
        );
    }
    sources
}

fn from_connectors(connectors_table: &Map<String, Value>) -> HashMap<String, ConnectorConfig> {
    let mut connectors = HashMap::new();
    for (connector_name, connector_value) in connectors_table {
        let connector_type = connector_value["type"].as_str().unwrap();
        let connector_config = match connector_type {
            "postgres" | "mysql" | "mssql" | "oracle" | "sqlite" => {
                let host = connector_value["host"].as_str().unwrap().to_string();
                let port = connector_value["port"].as_integer().unwrap() as u16;
                let user = connector_value["user"].as_str().unwrap().to_string();
                let password = connector_value["password"].as_str().unwrap().to_string();

                ConnectorConfig::Rds(RdsConfig {
                    host,
                    port,
                    user,
                    password,
                })
            }
            "kafka" => {
                let brokers = connector_value["brokers"].as_str().unwrap().to_string();
                ConnectorConfig::Kafka(KafkaConfig { brokers })
            }
            "nats" => {
                let url = connector_value["url"].as_str().unwrap().to_string();
                let topic = connector_value["topic"].as_str().unwrap().to_string();
                ConnectorConfig::Nats(NatsConfig { url, topic })
            }
            _ => {
                eprintln!("Unknown connector type: {}", connector_type);
                continue;
            }
        };
        connectors.insert(connector_name.to_string(), connector_config);
    }
    connectors
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};
    use toml::map::Map;
    use toml::Value;

    #[test]
    fn test_from_connectors_postgres() {
        let mut connectors_table: Map<String, Value> = Map::new();
        connectors_table.insert(
            "postgres".to_string(),
            toml::from_str(
                r#"
                type = "postgres"
                host = "localhost"
                port = 5432
                user = "user"
                password = "password"
                "#,
            )
            .unwrap(),
        );

        let result = from_connectors(&connectors_table);
        assert!(result.contains_key("postgres"));
        if let Some(config) = result.get("postgres") {
            if let ConnectorConfig::Rds(rds_config) = config {
                assert_eq!(rds_config.host, "localhost");
                assert_eq!(rds_config.port, 5432);
                assert_eq!(rds_config.user, "user");
                assert_eq!(rds_config.password, "password");
            } else {
                panic!("Expected Rds config for postgres");
            }
        } else {
            panic!("Postgres config not found");
        }
    }

    // Unit test for `from_file` function
    #[test]
    fn from_file_test() {
        // Define a dummy TOML content as a string.
        let dummy_toml_content = r#"
        name = "Test Config"
        description = "A description for testing"
        version = "0.1.0"

        [connectors]
        # connector definitions

        [sources]
        # source definitions

        [sinks]
        # sink definitions

        [pipeline]
        step1 = "source1"
        step2 = "connector1"
        step3 = "sink1"
    "#;

        // Write the dummy content to a temporary file
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        write!(file, "{}", dummy_toml_content).expect("Failed to write to temp file");
        let config_path = file.path().to_str().expect("Failed to get file path");

        // Call the function under test
        let result = ConfigSpec::from_file(config_path);

        // Assert the results
        assert!(
            result.is_ok(),
            "from_file returned an error: {:?}",
            result.err()
        );
        let config = result.unwrap();

        assert_eq!(config.name, "Test Config");
        assert_eq!(config.description, "A description for testing");
        assert_eq!(config.version, "0.1.0");
        // Add more assertions as necessary to validate the entire structure
    }

    #[test]
    fn new_config_spec_creates_valid_instance() {
        // Arrange (准备阶段)

        let current_dir = env::current_dir().expect("Unable to get current directory");
        let config_path = current_dir.join("tests/config.toml");
        let config_contents = fs::read_to_string(&config_path).expect("Unable to read config file");

        let name = "test_name".to_string();
        let description = "test_description".to_string();
        let version = "1.0".to_string();
        let pipeline = vec!["stage1".to_string(), "stage2".to_string()];

        // Act (执行阶段)
        // let config_spec = ConfigSpec::new(name, description, version, pipeline);

        // // Assert (断言阶段)
        // assert_eq!(config_spec.name, "test_name");
        // assert_eq!(config_spec.description, "test_description");
        // assert_eq!(config_spec.version, "1.0");
        // assert_eq!(config_spec.pipeline, vec!["stage1", "stage2"]);
    }
}
