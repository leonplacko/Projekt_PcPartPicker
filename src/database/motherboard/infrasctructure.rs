extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::motherboards;
use schema::motherboards::dsl::*;

impl CRUD for Motherboard {
    fn create(&self, conn: &DBPooledConnection) -> Result<Motherboard, diesel::result::Error> {
        let mb = NewMotherboard {
            name: self.name.clone(),
            manufacturer: self.manufacturer.clone(),
            ram_slots: self.ram_slots,
            sata_slots: self.sata_slots,
            nvme_slots: self.nvme_slots,
            price: self.price,
        };

        diesel::insert_into(motherboards::table)
            .values(&mb)
            .get_result(conn)
    }

    fn read(&self, conn: &DBPooledConnection) -> Result<Vec<Motherboard>, diesel::result::Error> {
        motherboards.load::<Motherboard>(conn)
    }

    fn update(
        &self,
        conn: &DBPooledConnection,
        other: Motherboard,
    ) -> Result<Motherboard, diesel::result::Error> {
        diesel::update(motherboards.filter(name.eq(&self.name)))
            .set((
                name.eq(other.name),
                manufacturer.eq(other.manufacturer),
                ram_slots.eq(other.ram_slots),
                sata_slots.eq(other.sata_slots),
                nvme_slots.eq(other.nvme_slots),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(motherboards.filter(name.eq(&self.name))).execute(conn)
    }
}
