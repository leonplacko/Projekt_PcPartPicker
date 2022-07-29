extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::storage_slots;
use schema::storage_slots::dsl::*;

impl CRUD for STRGslots {
    fn create(&self, conn: &DBPooledConnection) -> Result<STRGslots, diesel::result::Error> {
        diesel::insert_into(storage_slots::table)
            .values(&*self)
            .get_result(conn)
    }

    fn read_all(&self, conn: &DBPooledConnection) -> Result<Vec<STRGslots>, diesel::result::Error> {
        storage_slots.load::<STRGslots>(conn)
    }

    fn update(
        &self,
        conn: &DBPooledConnection,
        other: STRGslots,
    ) -> Result<STRGslots, diesel::result::Error> {
        diesel::update(
            storage_slots
                .filter(motherboard_id.eq(other.motherboard_id))
                .filter(storage_id.eq(other.storage_id)),
        )
        .set(slot.eq(other.slot))
        .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(
            storage_slots
                .filter(motherboard_id.eq(&self.motherboard_id))
                .filter(storage_id.eq(&self.storage_id)),
        )
        .execute(conn)
    }
}
