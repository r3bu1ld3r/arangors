#![allow(unused_imports)]
#![allow(unused_parens)]

use log::trace;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};

use arangors::document::{
    DocumentInsertOptions, DocumentOverwriteMode, DocumentReadOptions, DocumentResponse,
    DocumentUpdateOptions,
};
use arangors::{ClientError, Connection, Document};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};

pub mod common;
#[cfg(not(feature = "arango3_7"))]
#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_post_create_document() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_create_document";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);

    let coll = database.create_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);

    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"Trying to make unit test for createDocument but there are many cases to handle"
    }));

    // First test is to create a simple document without options
    let create = coll.create_document(test_doc, None).await;
    assert_eq!(create.is_ok(), true, "succeed create a document");

    let result = create.unwrap();

    let header = result.header.unwrap();
    assert_eq!(header._id.is_empty(), false);
    assert_eq!(header._rev.is_empty(), false);
    assert_eq!(header._key.is_empty(), false);
    // Second test is to create a simple document with option to get the new document back
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "testDescription":"Test with new"
    }));

    let create = coll
        .create_document(
            test_doc,
            Some(DocumentInsertOptions {
                wait_for_sync: None,
                return_new: Some(true),
                return_old: None,
                silent: None,
                overwrite: None,
            }),
        )
        .await;
    assert_eq!(create.is_ok(), true, "succeed create a document");
    let result = create.unwrap();

    assert_eq!(result.new.is_some(), true);

    let doc = result.new.unwrap();

    assert_eq!(doc.document["testDescription"], "Test with new");

    let header = result.header.unwrap();
    assert_eq!(header._id.is_empty(), false);
    assert_eq!(header._rev.is_empty(), false);
    assert_eq!(header._key.is_empty(), false);

    let key = header._key;
    // Third test is to update a simple document with option return old
    // Should not return  anything according to doc if overWriteMode is not used for now
    // TODO update this test with overwriteMode later
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "_key" : key,
    "testDescription":"Test with old"
    }));
    let update = coll
        .create_document(
            test_doc,
            Some(DocumentInsertOptions {
                wait_for_sync: None,
                return_new: None,
                return_old: Some(true),
                silent: None,
                overwrite: Some(true),
            }),
        )
        .await;
    assert_eq!(update.is_ok(), true, "succeed update a document");
    let result = update.unwrap();

    assert_eq!(result.old.is_some(), true);

    let old_doc = result.old.unwrap();
    assert_eq!(old_doc.document["testDescription"], "Test with new");

    let header = result.header.unwrap();
    assert_eq!(header._id.is_empty(), false);
    assert_eq!(header._rev.is_empty(), false);
    assert_eq!(header._key.is_empty(), false);

    // Fourth testis about the silent option
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "_key" : key,
    "testDescription":"Test with silent"
    }));
    let update = coll
        .create_document(
            test_doc,
            Some(DocumentInsertOptions {
                wait_for_sync: None,
                return_new: None,
                return_old: None,
                silent: Some(true),
                overwrite: None,
            }),
        )
        .await;

    assert_eq!(update.is_ok(), true, "succeed create a document silently");

    let result = update.unwrap();

    assert_eq!(result.old.is_none(), true);
    assert_eq!(result.new.is_none(), true);
    assert_eq!(result.header.is_none(), true);
    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);
}

/// TODO need to use CI to validate this test
#[cfg(any(feature = "arango3_7"))]
#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_post_create_document_3_7() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_create_document_3_7";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);

    let coll = database.create_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);

    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"Trying to make unit test for createDocument but there are many cases to handle"
    }));

    // First test is to create a simple document without options
    let create = coll.create_document(test_doc, None).await;

    assert_eq!(create.is_ok(), true, "succeed create a document");

    let result = create.unwrap();
    let header = result.header.unwrap();
    assert_eq!(header._id.is_empty(), false);
    assert_eq!(header._rev.is_empty(), false);
    assert_eq!(header._key.is_empty(), false);
    // Second test is to create a simple document with option to get the new document back
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "testDescription":"Test with new"
    }));

    let create = coll
        .create_document(
            test_doc,
            Some(DocumentInsertOptions {
                wait_for_sync: None,
                return_new: Some(true),
                return_old: None,
                silent: None,
                overwrite_mode: None,
                overwrite: None,
            }),
        )
        .await;
    assert_eq!(create.is_ok(), true, "succeed create a document");
    let result = create.unwrap();

    assert_eq!(result.new.is_some(), true);

    let doc = result.new.unwrap();

    assert_eq!(doc.document["testDescription"], "Test with new");

    let header = result.header.unwrap();
    assert_eq!(header._id.is_empty(), false);
    assert_eq!(header._rev.is_empty(), false);
    assert_eq!(header._key.is_empty(), false);

    let key = header._key;
    // Third test is to update a simple document with option return old
    // Should not return  anything according to doc if overWriteMode is not used for now
    // TODO update this test with overwriteMode later
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "_key" : key,
    "testDescription":"Test with old"
    }));
    let update = coll
        .create_document(
            test_doc,
            Some(DocumentInsertOptions {
                wait_for_sync: None,
                return_new: None,
                return_old: Some(true),
                silent: None,
                overwrite_mode: None,
                overwrite: Some(true),
            }),
        )
        .await;

    let result = update.unwrap();
    assert_eq!(result.old.is_some(), true);

    let old_doc = result.old.unwrap();
    assert_eq!(old_doc.document["testDescription"], "Test with new");

    let header = result.header.unwrap();
    assert_eq!(header._id.is_empty(), false);
    assert_eq!(header._rev.is_empty(), false);
    assert_eq!(header._key.is_empty(), false);

    // Fourth testis about the silent option
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "_key" : key,
    "testDescription":"Test with silent"
    }));
    let update = coll
        .create_document(
            test_doc,
            Some(DocumentInsertOptions {
                wait_for_sync: None,
                return_new: None,
                return_old: None,
                silent: Some(true),
                overwrite_mode: None,
                overwrite: None,
            }),
        )
        .await;

    let result = update.unwrap();

    assert_eq!(result.old.is_none(), true);
    assert_eq!(result.new.is_none(), true);
    assert_eq!(result.header.is_none(), true);

    // Fifth test is about the overwrite _mode option ignore
    let test_doc: Document<Value> = Document::new(json!({ "no":2 ,
    "_key" : key,
    "testDescription":"Test with overwrite mode"
    }));
    let update = coll
        .create_document(
            test_doc,
            Some(DocumentInsertOptions {
                wait_for_sync: None,
                return_new: Some(true),
                return_old: None,
                silent: None,
                overwrite_mode: Some(DocumentOverwriteMode::Ignore),
                overwrite: None,
            }),
        )
        .await;

    let result = update.unwrap();

    assert_eq!(result.old.is_none(), true);
    assert_eq!(result.new.is_none(), true);
    assert_eq!(result.header.is_none(), true);

    // Sixth test is about the overwrite _mode option replace
    let test_doc: Document<Value> = Document::new(json!({ "no":3 ,
    "_key" : key,
    "testDescription":"Test with overwrite mode"
    }));
    let update = coll
        .create_document(
            test_doc,
            Some(DocumentInsertOptions {
                wait_for_sync: None,
                return_new: None,
                return_old: None,
                silent: None,
                overwrite_mode: Some(DocumentOverwriteMode::Replace),
                overwrite: None,
            }),
        )
        .await;

    let result = update.unwrap();
    assert_eq!(result.old.is_none(), true);
    assert_eq!(result.new.is_none(), false);

    let doc = result.new.unwrap();
    assert_eq!(doc.document["no"], 3);

    assert_eq!(result.header.is_none(), false);
    // Seventh test is about the overwrite _mode option update
    let test_doc: Document<Value> = Document::new(json!({ "no":4 ,
    "_key" : key,
    }));
    let update = coll
        .create_document(
            test_doc,
            Some(DocumentInsertOptions {
                wait_for_sync: None,
                return_new: None,
                return_old: None,
                silent: None,
                overwrite_mode: Some(DocumentOverwriteMode::Update),
                overwrite: None,
            }),
        )
        .await;

    let result = update.unwrap();
    assert_eq!(result.old.is_none(), true);
    assert_eq!(result.new.is_none(), false);

    let doc = result.new.unwrap();
    assert_eq!(doc.document["no"], 4);

    assert_eq!(result.header.is_none(), false);

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_get_read_document() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_read_document";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);

    let coll = database.create_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);

    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"read a document"
    }));

    // First test is to read a simple document without options
    let create = coll.create_document(test_doc, None).await;
    assert_eq!(create.is_ok(), true, "succeed create a document");

    let header = create.unwrap().header.unwrap();
    let _key = header._key;
    let _rev = header._rev;
    let read = coll.read_document(_key.as_str()).await;

    let result: Document<Value> = read.unwrap();

    assert_eq!(result.document["no"], 1);
    assert_eq!(result.document["testDescription"], "read a document");
    // Test if we get the right doc when it does match
    let read: Result<Document<Value>, ClientError> = coll
        .read_document_with_options(
            _key.as_str(),
            Some(DocumentReadOptions::IfMatch(_rev.clone())),
        )
        .await;
    assert_eq!(read.is_err(), false);
    // Test if we get the 412 code response when there is no match
    let read: Result<Document<Value>, ClientError> = coll
        .read_document_with_options(
            _key.as_str(),
            Some(DocumentReadOptions::IfMatch("_dsdsds_d".to_string())),
        )
        .await;
    // We should get a 412, for now for some reason the error is parsed as a document
    // todo fix how the reponse/error is built
    assert_eq!(read.is_err(), true);

    // todo need to test with with IfNoneMatch

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);
}

// #[maybe_async::test(
//     any(feature = "reqwest_blocking"),
//     async(any(feature = "reqwest_async"), tokio::test),
//     async(any(feature = "surf_async"), async_std::test)
// )]
// async fn test_get_read_document_header() {
//     test_setup();
//     let host = get_arangodb_host();
//     let user = get_normal_user();
//     let password = get_normal_password();
//
//     let collection_name = "test_collection_read_document_header";
//
//     let conn = Connection::establish_jwt(&host, &user, &password)
//         .await
//         .unwrap();
//     let mut database = conn.db("test_db").await.unwrap();
//
//     let coll = database.drop_collection(collection_name).await;
//     assert_eq!(coll.is_err(), true);
//
//     let coll = database.create_collection(collection_name).await;
//     assert_eq!(coll.is_err(), false);
//
//     let coll = database.collection(collection_name).await.unwrap();
//
//     let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
//     "testDescription":"read a document"
//     }));
//
//     // First test is to read a simple document without options
//     let create = coll.create_document(test_doc, None).await;
//     assert_eq!(create.is_ok(), true, "succeed create a document");
//
//     let _key = create.unwrap().header.unwrap()._key;
//     eprintln!("{:?}", _key);
//
//     let read = coll.read_document_header(_key.as_str()).await;
//     eprintln!("{:?}", read);
//     let result: DocumentHeaderOptions = read.unwrap();
//     eprintln!("{:?}", result);
//     assert_eq!(result._key, _key);
//     let coll = database.drop_collection(collection_name).await;
//     assert_eq!(coll.is_err(), false);
// }
#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_patch_update_document() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_update_document";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);

    let coll = database.create_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);

    let coll = database.collection(collection_name).await.unwrap();

    let test_doc: Document<Value> = Document::new(json!({ "no":1 ,
    "testDescription":"update document"
    }));

    // First test is to read a simple document without options
    let create = coll.create_document(test_doc, None).await;

    assert_eq!(create.is_ok(), true, "succeed create a document");

    let _key = create.unwrap().header.unwrap()._key;

    let update = coll
        .update_document(
            _key.as_str(),
            json!({ "no":2}),
            Some(DocumentUpdateOptions {
                keep_null: None,
                merge_objects: None,
                wait_for_sync: None,
                ignore_revs: None,
                return_new: Some(true),
                return_old: None,
                silent: None,
            }),
        )
        .await;

    let result: DocumentResponse<Value> = update.unwrap();
    assert_eq!(result.new.unwrap()["no"], 2);

    let _rev = result.header.unwrap()._rev;
    let update = coll
        .update_document(
            _key.as_str(),
            json!({ "no":3}),
            Some(DocumentUpdateOptions {
                keep_null: None,
                merge_objects: None,
                wait_for_sync: None,
                ignore_revs: None,
                return_new: None,
                return_old: None,
                silent: None,
            }),
        )
        .await;

    let result: DocumentResponse<Value> = update.unwrap();

    assert_eq!(result.header.unwrap()._rev != _rev, true);

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);
}
