// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use mongodb::{
    self as mongo,
    Database,
    bson::{doc, Bson},
};
use reusable_fmt::fmt;
use crate::{*, consts};

type DbResult<T> = Result<T, mongo::error::Error>;

fn notes_collection(db: &Database) -> mongo::Collection {
    db.collection(consts::db::COLLECTION_NOTES)
}

fn stickers_collection(db: &Database) -> mongo::Collection {
    db.collection(consts::db::COLLECTION_STICKERS)
}

pub async fn add_note(db: &Database, name: &str, msg_id: i32) -> DbResult<mongo::results::UpdateResult> {
    let notes = notes_collection(db);
    notes
        .update_one(
            doc! { "_id": name },
            doc! { "message_id": msg_id },
            mongo::options::UpdateOptions::builder()
                .upsert(true)
                .build(),
        )
        .await
}

pub async fn find_note(db: &Database, name: &str) -> DbResult<Option<i32>> {
    let notes = notes_collection(db);
    let note = notes.find_one(doc! { "_id": name }, None).await?;
    Ok(note.map(|v| v.get("message_id").and_then(Bson::as_i32).unwrap()))
}

pub async fn remove_note(db: &Database, name: &str) -> DbResult<mongo::results::DeleteResult> {
    let notes = notes_collection(db);
    notes
        .delete_one(doc! { "_id": &name }, None)
        .await
}

pub async fn note_list(db: &Database) -> DbResult<impl Iterator<Item = String>> {
    let notes = notes_collection(db);
    let keys = notes.distinct("_id", None, None).await?;
    Ok(keys.into_iter().map(|b| Bson::as_str(&b).unwrap().to_owned()))
}

pub async fn sticker_pack_name(db: &Database, id: i32, pack_user: &str) -> DbResult<String> {
    let stick_coll = stickers_collection(db);
    let pack = stick_coll.find_one_and_update(
        doc!{"pack_name": doc!{"$exists": true}},
        doc!{"$setOnInsert": doc!{"pack_name": fmt!(PACK_NAME_FMT, id = id, user = pack_user, time = time::epoch_ms())}}, // TODO: We don't want this to be eager
        mongo::options::FindOneAndUpdateOptions::builder()
            .upsert(true)
            .return_document(mongo::options::ReturnDocument::After)
            .build()
    ).await?.unwrap();
    let pack = pack.get("pack_name").and_then(Bson::as_str).unwrap();

    Ok(pack.to_owned())
}
