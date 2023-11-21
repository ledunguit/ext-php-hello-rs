use ext_php_rs::zend::ExecutorGlobals;

/// Get a parameter from the $_GET array
pub fn get(param: &str) -> String {
    ExecutorGlobals::get()
        .symbol_table
        .get("_GET")
        .and_then(|v| v.array())
        .and_then(|array| array.get(&param))
        .and_then(|result| result.string())
        .unwrap_or_else(|| "".to_owned())
}
