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
    fn create(&self, conn: &DBPooledConnection) -> Result<RamSlot, diesel::result::Error> {
        diesel::insert_into(ram_slot::table)
            .values(&*self)
            .get_result(conn)
    }

    fn read_all(&self, conn: &DBPooledConnection) -> Result<Vec<RamSlot>, diesel::result::Error> {
        ram_slot.load::<RamSlot>(conn)
    }

    fn update(
        &self,
        conn: &DBPooledConnection,
        other: RamSlot,
    ) -> Result<RamSlot, diesel::result::Error> {
        diesel::update(
            ram_slot
                .filter(motherboard_id.eq(&self.motherboard_id))
                .filter(ram_id.eq(&self.ram_id)),
        )
        .set(type_.eq(other.type_))
        .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(
            ram_slot
                .filter(motherboard_id.eq(&self.motherboard_id))
                .filter(ram_id.eq(&self.ram_id)),
        )
        .execute(conn)
    }
}
