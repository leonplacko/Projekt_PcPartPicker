extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use super::handle_storage_slot::handle_storage_slot_insert;
use diesel::QueryDsl;
use schema::storages;
use schema::storages::dsl::*;

impl CRUD for Storage {
    fn create(
        exstor: ExtendStorage,
        conn: &DBPooledConnection,
    ) -> Result<Storage, diesel::result::Error> {
        let storslot = exstor.slot.clone();

        let stor = NewStorage::from(exstor);

        let res1 = diesel::insert_into(storages::table)
            .values(&stor)
            .get_result::<Storage>(conn);

        match res1 {
            Err(er) => Err(er),
            Ok(val) => match handle_storage_slot_insert(storslot, conn, val.id.clone()) {
                Err(er) => Err(er),
                Ok(_) => Ok(val),
            },
        }
    }

    fn read_all(conn: &DBPooledConnection) -> Result<Vec<Storage>, diesel::result::Error> {
        storages.load::<Storage>(conn)
    }

    fn update(
        conn: &DBPooledConnection,
        other: NewStorage,
    ) -> Result<Storage, diesel::result::Error> {
        diesel::update(storages.filter(name.eq(other.name)))
            .set((
                manufacturer.eq(other.manufacturer),
                capacity.eq(other.capacity),
                speed.eq(other.speed),
                type_.eq(other.type_),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(name_: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(storages.filter(name.eq(name_))).execute(conn)
    }
}
