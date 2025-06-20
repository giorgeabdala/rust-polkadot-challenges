#![allow(dead_code)]
use std::collections::HashMap;


fn longest_config_name<'a>(c1: &'a Config, c2: &'a Config) -> &'a str {
    if c1.values.len() >= c2.values.len() {
        return c1.name
    }
    c2.name
}

#[derive(Debug, PartialEq)]
enum ConfigValue<'a> {
    Text(&'a str),
    Number(i64),
    Boolean(bool),
}

struct Config<'a> {
    name: &'a str,
    values: HashMap<&'a str, ConfigValue<'a>>
}
impl<'a> Config<'a> {
    
    fn new(name: &'a str) -> Self {
        Config{name, values: HashMap::new()}
    }
    fn set(&mut self, key:  &'a str, value: ConfigValue<'a>) {
        self.values.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Option<&ConfigValue<'a>> {
        self.values.get(key)
    }
    
    fn get_text(&self, key: &str) -> Option<&str> {
        match self.get(key)? {
            ConfigValue::Text(s) => Some(s),
            _ => None,
        }
    }
}

mod tests {
    use crate::medium::challenge_03::{longest_config_name, Config, ConfigValue};

    #[test]
    fn test_config_with_static_lifetimes() {
        let mut config = Config::new("test");
        config.set("app", ConfigValue::Text("MyApp"));
        config.set("version", ConfigValue::Number(1));
        
        assert_eq!(config.get_text("app"), Some("MyApp"));
        //assert_eq!(config.get_text("version"), Some(1));
    }

    #[test]
    fn test_get_config_test() {
        let mut config = Config::new("test");
        config.set("app", ConfigValue::Text("MyApp"));
        let config_value_opt = config.get("app");
        assert!(config_value_opt.is_some());
        let config_value = config_value_opt.unwrap();
        assert_eq!(config_value, &ConfigValue::Text("MyApp"));
        
    }

    #[test]
    fn test_longest_config_name() {
        let mut config1 = Config::new("short");
        config1.set("key1", ConfigValue::Boolean(true));
        
        let mut config2 = Config::new("longer_name");
        config2.set("key1", ConfigValue::Number(1));
        config2.set("key2", ConfigValue::Text("value"));
        
        assert_eq!(longest_config_name(&config1, &config2), "longer_name");
    }
    
    
}