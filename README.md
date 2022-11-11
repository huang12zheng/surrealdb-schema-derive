* please check
impl/src/lib.rs

* usage
```rs
#[derive(Debug, Clone)]
pub struct Relation {
    pub id: Thing,
    pub from: Thing,
    pub with: Thing,
}

impl TryFrom<surrealdb_obj_derive::SurrealValue> for Relation {}
```
> ref to crate schema

or

```rs
#[derive(SurrealDbObject, Debug, Clone)]
pub struct Mobile {
    pub id: u64,
}
```

* release
```
cargo install cargo-release

```