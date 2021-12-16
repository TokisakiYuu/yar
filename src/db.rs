use indexed_db_futures::prelude::*;
use wasm_bindgen::JsValue;

pub async fn db_test() {
    let mut db_req: OpenDbRequest = IdbDatabase::open_u32("my_db", 1).unwrap();
    db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
        if let None = evt.db().object_store_names().find(|n| n == "my_store") {
            evt.db().create_object_store("my_store").unwrap();
        }
        Ok(())
    }));

    let db: IdbDatabase = db_req.into_future().await.unwrap();

    // Insert/overwrite a record
    let tx: IdbTransaction = db
      .transaction_on_one_with_mode("my_store", IdbTransactionMode::Readwrite).unwrap();
    let store: IdbObjectStore = tx.object_store("my_store").unwrap();

    let value_to_put: JsValue = JsValue::from("hello world");
    store.put_key_val_owned("my_key", &value_to_put).unwrap();

    // IDBTransactions can have an Error or an Abort event; into_result() turns both into a
    // DOMException
    tx.await.into_result().unwrap();

    // Delete a record
    let tx = db.transaction_on_one_with_mode("my_store", IdbTransactionMode::Readwrite).unwrap();
    let store = tx.object_store("my_store").unwrap();
    store.delete_owned("my_key").unwrap();
    tx.await.into_result().unwrap();

    // Get a record
    let tx = db.transaction_on_one("my_store").unwrap();
    let store = tx.object_store("my_store").unwrap();

    let value = store.get_owned("my_key").unwrap().await.unwrap().unwrap();
    
    println!("{}", value.as_string().unwrap());
}