use ifcx_core::parser;
use serde_json::{json, Value};

fn envelope() -> Value {
    json!({
        "imports": [], "schemas": {}, "foreignDocument": {"owner": "another-tool"},
        "data": [{"path": "wall", "children": {"removed": null},
            "inherits": {"oldType": null}, "foreignNode": [1, null],
            "attributes": {"foreign::payload": {"enabled": false}}}]
    })
}

fn save_and_reopen(input: &Value) -> Value {
    let document = parser::from_str(&input.to_string()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("saved.ifcx");
    parser::to_file(&document, &path).unwrap();
    let first: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    let reopened = parser::from_file(&path).unwrap();
    let second: Value = serde_json::from_str(&parser::to_string(&reopened).unwrap()).unwrap();
    assert_eq!(
        second, first,
        "a second save must preserve the first result"
    );
    first
}

#[test]
fn save_keeps_an_absent_header_absent() {
    let input = envelope();
    assert_eq!(save_and_reopen(&input), input);
}

#[test]
fn save_keeps_an_explicit_empty_header() {
    let mut input = envelope();
    input["header"] = json!({});
    assert_eq!(save_and_reopen(&input), input);
}

#[test]
fn save_keeps_partial_headers_and_explicit_empty_strings() {
    for field in ["id", "ifcxVersion", "dataVersion", "author", "timestamp"] {
        for value in ["", "provided-value"] {
            let mut input = envelope();
            input["header"] = json!({field: value, "foreignHeader": {"nested": [null, 7]}});
            assert_eq!(save_and_reopen(&input), input, "{field}={value:?}");
        }
    }
}

#[test]
fn missing_header_does_not_generate_metadata() {
    let output = save_and_reopen(&json!({"data": []}));
    assert!(output.get("header").is_none());
    // The existing normalization of imports and schemas is independent of header presence.
    assert_eq!(output["imports"], json!([]));
    assert_eq!(output["schemas"], json!({}));
}

#[test]
fn explicit_null_or_wrong_header_types_are_not_treated_as_absence() {
    for header in [Value::Null, json!(3), json!("header")] {
        assert!(parser::from_str(&json!({"header": header, "data": []}).to_string()).is_err());
    }
    for field in ["id", "ifcxVersion", "dataVersion", "author", "timestamp"] {
        for value in [Value::Null, json!(3), json!({})] {
            let input = json!({"header": {field: value}, "data": []});
            assert!(parser::from_str(&input.to_string()).is_err(), "{input}");
        }
    }
}

#[test]
fn edited_metadata_can_be_added_emptied_and_removed() {
    let mut document = parser::from_str(r#"{"data":[]}"#).unwrap();
    document.header = Some(ifcx_core::IfcxHeader {
        author: Some("Ada".into()),
        ..Default::default()
    });
    assert_eq!(
        serde_json::to_value(&document).unwrap()["header"],
        json!({"author": "Ada"})
    );
    let header = document.header.as_mut().unwrap();
    header.author = Some(String::new());
    header.id = Some("added-id".into());
    assert_eq!(
        serde_json::to_value(&document).unwrap()["header"],
        json!({"author": "", "id": "added-id"})
    );
    document.header.as_mut().unwrap().author = None;
    assert_eq!(
        serde_json::to_value(&document).unwrap()["header"],
        json!({"id": "added-id"})
    );
    document.header = None;
    assert!(serde_json::to_value(&document)
        .unwrap()
        .get("header")
        .is_none());
}

#[test]
fn generated_schema_describes_optional_but_non_null_metadata() {
    let schema = serde_json::to_value(schemars::schema_for!(ifcx_core::IfcxDocument)).unwrap();
    assert_eq!(schema["required"], json!(["data"]));
    let property = &schema["properties"]["header"];
    // Schemars may wrap a reference in allOf to attach the field's documentation.
    let reference = property
        .get("$ref")
        .or_else(|| property.pointer("/allOf/0/$ref"));
    assert_eq!(
        reference.and_then(Value::as_str),
        Some("#/definitions/IfcxHeader")
    );
    let header = &schema["definitions"]["IfcxHeader"];
    assert_eq!(header["type"], "object");
    assert!(header.get("required").is_none());
    for field in ["id", "ifcxVersion", "dataVersion", "author", "timestamp"] {
        assert_eq!(header["properties"][field]["type"], "string");
    }
}
