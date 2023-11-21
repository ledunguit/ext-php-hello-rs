use ext_php_rs::zend::ExecutorGlobals;

/// Get a parameter from the $_GET array
pub fn get(param: &str) -> String {
    let value = ExecutorGlobals::get().symbol_table.get("_GET").and_then(|v| {
        if v.is_array() {
            let array = v.array();

            if let Some(array) = array {
                let result = array.get(&param);
                if let Some(result) = result {
                    if result.is_string() {
                        result.string()
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    });

    match value {
        Some(v) => v,
        None => "".to_owned(),
    }
}
