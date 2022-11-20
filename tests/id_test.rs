// use crate::test_base::*;
use surrealdb::{sql::Value, Datastore, Session};

use surrealdb_schema_derive::*;

#[derive(SurrealDbObject, Debug, Clone)]
pub struct Mobile {
    id: u64,
    // num: u64,  // success
}

#[tokio::test]
async fn test_id() {
    let (namespace, database, path) = ("test", "test", "memory");
    let datastore = Datastore::new(&path).await.unwrap();
    let session = Session::for_db(namespace.to_string(), database.to_string());

    let value = datastore
        .execute("insert into mobile {id:123}", &session, None, false)
        // .execute("insert into mobile {num:123}", &session, None, false)
        .await
        .unwrap()
        .remove(0)
        .result
        .unwrap();
    // dbg!(v);
    let mut vs = match value {
        Value::Array(x) => x.0,
        _ => panic!("error convert"),
    };
    let v = vs.remove(0);
    // dbg!(&v);
    let obj: Mobile = SurrealValue(v).try_into().unwrap();
    dbg!(&obj);
}
