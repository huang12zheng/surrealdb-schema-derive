use surrealdb::sql::Value;

pub trait IntoValue {
    fn into(self) -> Value;
}

impl<T: Into<Value>> IntoValue for Vec<T> {
    fn into(self) -> Value {
        Value::from(self.into_iter().map(|e| e.into()).collect::<Vec<Value>>())
    }
}
