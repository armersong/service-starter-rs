use crate::defines::errcode::RC_FAIL;
use crate::{Error, ErrorKind, Result};
use serde_json::{Map, Value};

#[allow(dead_code)]
pub fn safe_json_str(s: String) -> String {
    let mut src = s;
    let bytes = unsafe { src.as_bytes_mut() };
    for i in 0..bytes.len() {
        if bytes[i] == b'\'' || bytes[i] == b'\"' {
            bytes[i] = b'|';
        }
    }
    src
}

pub fn get_json_str(root: &Map<String, Value>, field_name: &str) -> Result<String> {
    match root.get(field_name) {
        Some(v) => match v {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => Ok(format!("{}", n)),
            _ => Err(Error::from(ErrorKind::Custom(
                RC_FAIL,
                format!("field {} is not string", field_name),
            ))),
        },
        None => Err(Error::from(ErrorKind::Custom(
            RC_FAIL,
            format!("field {} not found", field_name),
        ))),
    }
}

pub fn get_json_integer(root: &Map<String, Value>, field_name: &str) -> Result<i64> {
    let v = match root.get(field_name) {
        Some(v) => match v {
            Value::String(s) => s.as_str().parse::<i64>().map_err(|e| {
                Error::from(ErrorKind::Custom(
                    RC_FAIL,
                    format!("field {} is not integer: {}", field_name, e),
                ))
            })?,
            Value::Number(n) => {
                if let Some(v) = n.as_i64() {
                    v
                } else if let Some(v) = n.as_f64() {
                    v as i64
                } else if let Some(v) = n.as_u64() {
                    v as i64
                } else {
                    0
                }
            }
            _ => {
                return Err(Error::from(ErrorKind::Custom(
                    RC_FAIL,
                    format!("field {} is not integer", field_name),
                )))
            }
        },
        None => {
            return Err(Error::from(ErrorKind::Custom(
                RC_FAIL,
                format!("field {} not found", field_name),
            )))
        }
    };
    Ok(v)
}

pub fn get_json_float(root: &Map<String, Value>, field_name: &str) -> Result<f64> {
    let v = match root.get(field_name) {
        Some(v) => match v {
            Value::String(s) => s.as_str().parse::<f64>().map_err(|e| {
                Error::from(ErrorKind::Custom(
                    RC_FAIL,
                    format!("field {} is not float: {}", field_name, e),
                ))
            })?,
            Value::Number(n) => {
                if let Some(v) = n.as_i64() {
                    v as f64
                } else if let Some(v) = n.as_f64() {
                    v
                } else if let Some(v) = n.as_u64() {
                    v as f64
                } else {
                    0.0f64
                }
            }
            _ => {
                return Err(Error::from(ErrorKind::Custom(
                    RC_FAIL,
                    format!("field {} is not float", field_name),
                )))
            }
        },
        None => {
            return Err(Error::from(ErrorKind::Custom(
                RC_FAIL,
                format!("field {} not found", field_name),
            )))
        }
    };
    Ok(v)
}
