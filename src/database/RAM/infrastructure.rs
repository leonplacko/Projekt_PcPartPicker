extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::ram;
use schema::ram::dsl::*;

impl CRUD for RAM {
    fn create(&self, conn: &DBPooledConnection) -> Result<RAM, diesel::result::Error> {
        let n_ram = NewRAM {
            name: self.name.clone(),
            manufacturer: self.manufacturer.clone(),
            speed: self.speed,
            capacity: self.capacity,
            price: self.price,
        };

        diesel::insert_into(ram::table)
            .values(&n_ram)
            .get_result(conn)
    }

    fn read_all(&self, conn: &DBPooledConnection) -> Result<Vec<RAM>, diesel::result::Error> {
        ram.load::<RAM>(conn)
    }

    fn update(&self, conn: &DBPooledConnection, other: RAM) -> Result<RAM, diesel::result::Error> {
        diesel::update(ram.filter(name.eq(&self.name)))
            .set((
                name.eq(other.name),
                manufacturer.eq(other.manufacturer),
                capacity.eq(other.capacity),
                speed.eq(other.speed),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(ram.filter(name.eq(&self.name))).execute(conn)
    }
}
