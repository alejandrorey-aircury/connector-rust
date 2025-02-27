use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Definition {
    pub source: Source,
    pub target: Target,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub url: String,
    model: SourceModel,
}

#[derive(Debug, Deserialize)]
struct SourceModel {
    tables: HashMap<String, SourceTable>,
}

#[derive(Debug, Deserialize)]
struct SourceTable {
    schema: Option<String>,
    columns: HashMap<String, String>,
    keys: Option<Vec<String>>,
    resource_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    pub url: String,
    model: TargetModel,
}

#[derive(Debug, Deserialize)]
struct TargetModel {
    tables: HashMap<String, TargetTable>,
}

#[derive(Debug, Deserialize)]
struct TargetTable {
    schema: Option<String>,
    columns: Option<HashMap<String, String>>,
    keys: Option<Vec<String>>,
    resource_name: Option<String>,
    inherit: Option<String>,
    source_table: Option<String>,
}
