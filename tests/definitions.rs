#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

mod external_crate {
    use super::*;

    #[derive(Debug, Schematic)]
    #[rschema(defs)]
    pub struct PrivateStruct {
        prop_value: i32,
    }

    #[derive(Debug, Schematic)]
    #[rschema(defs)]
    pub struct Struct {
        prop_value: i32,

        prop_private: PrivateStruct,
    }
}

#[derive(Debug, Schematic)]
#[rschema(defs)]
struct NestedStruct {
    prop_value: i32,
}

#[derive(Debug, Schematic)]
#[rschema(defs)]
struct Struct {
    prop_value: i32,

    prop_nested_struct: NestedStruct,
}

#[derive(Debug, Schematic)]
#[rschema(defs)]
enum Enum {
    EmptyTupleVariant(),

    NewTypeVariant(i32),

    TupleVariant(String, bool),

    StructVariant {
        value: i32,
    }
}

#[derive(Debug, Schematic)]
#[rschema(defs = "CustomDefinition")]
struct NamedDefsStruct {
    prop_value: i32,
}

#[derive(Debug, Schematic)]
struct Definitions {
    prop_struct: Struct,

    // prop_enum: Enum, <- If this enum is not defined in any structure outside the Vec<Enum> below, it's removed from the generated definitions

    prop_external: external_crate::Struct,

    prop_named_defs_struct: NamedDefsStruct,
    
    prop_vector_enum: Vec<Enum>,
}

#[test]
fn it_tests_definitions() -> rschema::Result<()> {
    let schema_str = Schema::new::<Definitions>("Definitions")
        .to_string_pretty()?;
    let schema_str2 = r##"{
  "title": "Definitions",
  "type": "object",
  "properties": {
    "prop_struct": {
      "$ref": "#/$defs/definitions::Struct"
    },
    "prop_external": {
      "$ref": "#/$defs/definitions::external_crate::Struct"
    },
    "prop_named_defs_struct": {
      "$ref": "#/$defs/CustomDefinition"
    },
    "prop_vec_enum": {
      "type": "array",
      "items": [
        "$ref": "#/$defs/definitions::Enum"
      ]
    }
  },
  "additionalProperties": false,
  "$defs": {
    "definitions::Struct": {
      "type": "object",
      "properties": {
        "prop_value": {
          "type": "number"
        },
        "prop_nested_struct": {
          "$ref": "#/$defs/definitions::NestedStruct"
        }
      },
      "additionalProperties": false
    },
    "definitions::NestedStruct": {
      "type": "object",
      "properties": {
        "prop_value": {
          "type": "number"
        }
      },
      "additionalProperties": false
    },
    "definitions::Enum": {
      "anyOf": [
        {
          "type": "array",
          "items": [],
          "minItems": 0,
          "maxItems": 0
        },
        {
          "type": "number"
        },
        {
          "type": "array",
          "items": [
            {
              "type": "string"
            },
            {
              "type": "boolean"
            }
          ],
          "minItems": 2,
          "maxItems": 2
        },
        {
          "type": "object",
          "properties": {
            "value": {
              "type": "number"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "definitions::external_crate::Struct": {
      "type": "object",
      "properties": {
        "prop_value": {
          "type": "number"
        },
        "prop_private": {
          "$ref": "#/$defs/definitions::external_crate::PrivateStruct"
        }
      },
      "additionalProperties": false
    },
    "definitions::external_crate::PrivateStruct": {
      "type": "object",
      "properties": {
        "prop_value": {
          "type": "number"
        }
      },
      "additionalProperties": false
    },
    "CustomDefinition": {
      "type": "object",
      "properties": {
        "prop_value": {
          "type": "number"
        }
      },
      "additionalProperties": false
    }
  }
}"##;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}
