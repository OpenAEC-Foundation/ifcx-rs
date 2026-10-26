use ifcx_core::IfcxDocument;
use serde_json::{json, Value};

fn envelope() -> Value {
    json!({"header":{"id":"urn:test:roundtrip","ifcxVersion":"ifcx_alpha","dataVersion":"1","author":"test","timestamp":"2026-10-26T00:00:00Z","foreignHeader":{"revision":7}},"imports":[{"uri":"urn:test:schema","integrity":"sha256-example","foreignImport":false}],"schemas":{},"data":[{"path":"/wall","attributes":{"foreign::payload":{"nested":[false,0,null,"keep"]}},"foreignNode":{"owner":"other-tool"}}],"foreignDocument":[1,2,3]})
}

#[test]
fn roundtrip_retains_foreign_fields_at_every_level() {
    let input = envelope();
    let document: IfcxDocument = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(serde_json::to_value(document).unwrap(), input);
}

#[test]
fn roundtrip_retains_null_relationship_markers() {
    let mut input = envelope();
    input["data"][0]["children"] = json!({"removed":null,"present":"/opening"});
    input["data"][0]["inherits"] = json!({"previousType":null});
    let document: IfcxDocument = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(serde_json::to_value(document).unwrap(), input);
}
