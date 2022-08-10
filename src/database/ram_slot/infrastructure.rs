extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::ram_slot;
use schema::ram_slot::dsl::*;

impl CRUD for RamSlot {
    fn create(
        ramslot: NewRamSlot,
        conn: &DBPooledConnection,
    ) -> Result<RamSlot, diesel::result::Error> {
        diesel::insert_into(ram_slot::table)
            .values(&ramslot)
            .get_result(conn)
    }

    fn read(
        slotype: String,
        conn: &DBPooledConnection,
    ) -> Result<Vec<RamSlot>, diesel::result::Error> {
        ram_slot.filter(type_.eq(slotype)).load::<RamSlot>(conn)
    }

    fn update(conn: &DBPooledConnection, other: RamSlot) -> Result<RamSlot, diesel::result::Error> {
        diesel::update(
            ram_slot
                .filter(motherboard_id.eq(&other.motherboard_id))
                .filter(ram_id.eq(&other.ram_id)),
        )
        .set(type_.eq(other.type_))
        .get_result(conn)
    }

    fn delete(
        mbid: Option<String>,
        ramid: Option<String>,
        conn: &DBPooledConnection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(
            ram_slot
                .filter(motherboard_id.eq(&mbid))
                .filter(ram_id.eq(&ramid)),
        )
        .execute(conn)
    }
}
