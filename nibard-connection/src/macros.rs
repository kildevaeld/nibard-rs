macro_rules! bind_value {
    ($val: expr, $query: expr) => {
        match $val {
            Value::Text(text) => $query.bind(text),
            Value::Int(i) => $query.bind(i),
            Value::SmallInt(i) => $query.bind(i),
            Value::BigInt(i) => $query.bind(i),
            Value::Bool(b) => $query.bind(b),
            #[cfg(feature = "time")]
            Value::Date(date) => $query.bind(date),
            #[cfg(feature = "time")]
            Value::DateTime(date) => $query.bind(date),
            Value::Binary(blob) => $query.bind(blob),
            // worm_types::Value::Null => $query.bind(None),
            _ => panic!("value not implemented {:?}", $val),
        }
    };
}

macro_rules! bind_values {
    ($values: expr, $query: expr) => {{
        let mut query = $query;
        for val in $values {
            query = bind_value!(val, query);
        }
        query
    }};
}

macro_rules! query_and_bind {
    ($exec: expr) => {{
        let mut q = sqlx::query($exec.sql());
        if let Some(values) = $exec.args() {
            q = bind_values!(values, q);
        }
        q
    }};
}
