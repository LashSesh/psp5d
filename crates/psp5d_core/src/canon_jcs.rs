use crate::errors::CoreError;
use crate::rd::{FloatPolicy, RunDescriptor};
use serde_json::{Map, Number, Value};

pub fn canonicalize_jcs(value: &Value) -> Result<Vec<u8>, CoreError> {
    let mut out = String::new();
    write_jcs(value, &mut out)?;
    Ok(out.into_bytes())
}

pub fn canonicalize_with_rd(value: &Value, rd: &RunDescriptor) -> Result<Vec<u8>, CoreError> {
    let normalized = normalize_value(value, &rd.canon.float_policy)?;
    canonicalize_jcs(&normalized)
}

fn normalize_value(value: &Value, float_policy: &FloatPolicy) -> Result<Value, CoreError> {
    match value {
        Value::Object(map) => {
            let mut out = Map::new();
            for (k, v) in map {
                out.insert(k.clone(), normalize_value(v, float_policy)?);
            }
            Ok(Value::Object(out))
        }
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(normalize_value(item, float_policy)?);
            }
            Ok(Value::Array(out))
        }
        Value::Number(n) => normalize_number(n, float_policy),
        _ => Ok(value.clone()),
    }
}

fn normalize_number(number: &Number, float_policy: &FloatPolicy) -> Result<Value, CoreError> {
    if let Some(i) = number.as_i64() {
        return Ok(Value::Number(Number::from(i)));
    }
    if let Some(u) = number.as_u64() {
        return Ok(Value::Number(Number::from(u)));
    }

    let f = number.as_f64().ok_or(CoreError::NonFiniteNumber)?;
    if !f.is_finite() {
        return Err(CoreError::NonFiniteNumber);
    }

    let rounded = match float_policy {
        FloatPolicy::Q16_16RoundHalfEven => round_half_even(f * 65_536.0) / 65_536.0,
    };

    let normalized = Number::from_f64(rounded).ok_or(CoreError::NumberCanonicalizationFailed)?;
    Ok(Value::Number(normalized))
}

fn round_half_even(value: f64) -> f64 {
    let floor = value.floor();
    let frac = value - floor;
    if frac < 0.5 {
        floor
    } else if frac > 0.5 {
        floor + 1.0
    } else if (floor as i64) % 2 == 0 {
        floor
    } else {
        floor + 1.0
    }
}

fn write_jcs(value: &Value, out: &mut String) -> Result<(), CoreError> {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if !f.is_finite() {
                    return Err(CoreError::NonFiniteNumber);
                }
            }
            out.push_str(&n.to_string());
        }
        Value::String(s) => out.push_str(&serde_json::to_string(s)?),
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_jcs(item, out)?;
            }
            out.push(']');
        }
        Value::Object(map) => {
            out.push('{');
            let mut keys: Vec<&str> = map.keys().map(String::as_str).collect();
            keys.sort_unstable();
            for (index, key) in keys.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push_str(&serde_json::to_string(key)?);
                out.push(':');
                write_jcs(&map[*key], out)?;
            }
            out.push('}');
        }
    }
    Ok(())
}
