#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct SchemaKey {
    pub name: String,
    pub version: String,
    pub did: String
}
