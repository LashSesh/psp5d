use jsonschema::JSONSchema;
use serde_json::Value;

use crate::errors::CoreError;

pub fn validate_against_schema(schema_name: &str, instance: &Value) -> Result<(), CoreError> {
    let schema_json = schema_for_name(schema_name)?;
    let compiled =
        JSONSchema::compile(&schema_json).map_err(|err| CoreError::SchemaValidation {
            schema_name: schema_name.to_string(),
            message: err.to_string(),
        })?;

    if let Err(errors) = compiled.validate(instance) {
        let message = errors.map(|e| e.to_string()).collect::<Vec<_>>().join("; ");
        return Err(CoreError::SchemaValidation {
            schema_name: schema_name.to_string(),
            message,
        });
    }

    Ok(())
}

fn schema_for_name(schema_name: &str) -> Result<Value, CoreError> {
    let raw = match schema_name {
        "rd" => include_str!("../../../spec/schemas/rd.schema.json"),
        "uir" => include_str!("../../../spec/schemas/uir.schema.json"),
        "state" => include_str!("../../../spec/schemas/state.schema.json"),
        "trace" => include_str!("../../../spec/schemas/trace.schema.json"),
        "evidence" => include_str!("../../../spec/schemas/evidence.schema.json"),
        "manifest" => include_str!("../../../spec/schemas/manifest.schema.json"),
        "operator_registry" => include_str!("../../../spec/schemas/operator_registry.schema.json"),
        "block" => include_str!("../../../spec/schemas/block.schema.json"),
        "triton_profile" => include_str!("../../../spec/schemas/triton_profile.schema.json"),
        "triton_gate_evidence" => {
            include_str!("../../../spec/schemas/triton_gate_evidence.schema.json")
        }
        _ => return Err(CoreError::UnsupportedSchema(schema_name.to_string())),
    };

    serde_json::from_str(raw).map_err(CoreError::from)
}
