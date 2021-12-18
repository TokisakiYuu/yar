use indexed_db_futures::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::Object;
use js_sys::Reflect;

pub async fn db_test() {
    let mut db_req: OpenDbRequest = IdbDatabase::open_u32("my_db", 1).unwrap();
    db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
        if let None = evt.db().object_store_names().find(|n| n == "my_store") {
            let mut params = IdbObjectStoreParameters::new();
            params.auto_increment(true);
            params.key_path(Some(&IdbKeyPath::from("id")));
            let object_store = evt.db().create_object_store_with_params("my_store", &params).unwrap();
            let mut index_params = IdbIndexParameters::new();
            index_params.unique(false);
            object_store.create_index_with_params("name", &IdbKeyPath::from("name"), &index_params.clone()).unwrap();
            object_store.create_index_with_params("age", &IdbKeyPath::from("age"), &index_params.clone()).unwrap();
        }
        Ok(())
    }));

    let db: IdbDatabase = db_req.into_future().await.unwrap();

    // Insert/overwrite a record
    let tx: IdbTransaction = db
      .transaction_on_one_with_mode("my_store", IdbTransactionMode::Readwrite).unwrap();
    let store: IdbObjectStore = tx.object_store("my_store").unwrap();

    // let value_to_put: JsValue = JsValue::from("hello world");
    // store.put_key_val_owned("my_key", &value_to_put).unwrap();
    let object = Object::new();
    Reflect::set(&object, &JsValue::from("name"), &JsValue::from("张三")).unwrap();
    Reflect::set(&object, &JsValue::from("age"), &JsValue::from_f64(18 as f64)).unwrap();
    store.add_val(&object).unwrap();
    // IDBTransactions can have an Error or an Abort event; into_result() turns both into a
    // DOMException
    tx.await.into_result().unwrap();

    // // Delete a record
    // let tx = db.transaction_on_one_with_mode("my_store", IdbTransactionMode::Readwrite).unwrap();
    // let store = tx.object_store("my_store").unwrap();
    // store.delete_owned("my_key").unwrap();
    // tx.await.into_result().unwrap();

    // // Get a record
    // let tx = db.transaction_on_one("my_store").unwrap();
    // let store = tx.object_store("my_store").unwrap();

    // // TODO here got a None, i can not use unwrap skip this Option
    // let value = store.get_owned("my_key").unwrap().await.unwrap().unwrap();
    
    // println!("{}", value.as_string().unwrap());
}