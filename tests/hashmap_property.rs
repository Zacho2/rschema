#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

use std::collections::HashMap;

#[derive(Debug, Schematic)]
enum Enum {
    UnitVariant,

    EmptyTupleVariant(),

    TupleVariant(i32, String),

    StructVariant {
        #[rschema(title = "i32")]
        value: i32,
    }
}

#[derive(Debug, Schematic)]
struct HashMapProperty {
    #[rschema(title = "HashMap<String, u32>")]
    prop_hashmap_simgle: HashMap<String, u32>,

    #[rschema(title = "HashMap<String, Enum>")]
    prop_hashmap_complex: HashMap<String, Enum>,
}

#[test]
fn it_generates_null_schema() -> rschema::Result<()> {
    let schema_str = Schema::new::<HashMapProperty>("HashMap Property")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "HashMap Property",
  "type": "object",
  "properties": {
    "prop_hashmap_simgle": {
      "title": "HashMap<String, u32>",
      "type": "object",
      "properties": {},
      "additionalProperties": {
        "type": "number"
      }
    },
    "prop_hashmap_complex": {
      "title": "HashMap<String, Enum>",
      "type": "object",
      "properties": {},
      "additionalProperties": {
        "anyOf": [
          {
            "type": "array",
            "items": []
          },
          {
            "type": "array",
            "items": [
              {
                "type": "number"
              },
              {
                "type": "string"
              }
            ]
          },
          {
            "type": "object",
            "properties": {
              "value": {
                "title": "i32",
                "type": "number"
              }
            },
            "additionalProperties": false
          },
          {
            "type": "string",
            "enum": [
              "UnitVariant"
            ]
          }
        ]
      }
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}