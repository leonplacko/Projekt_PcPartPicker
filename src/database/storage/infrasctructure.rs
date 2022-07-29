extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::storages;
use schema::storages::dsl::*;

impl CRUD for Storage {
    fn create(&self, conn: &DBPooledConnection) -> Result<Storage, diesel::result::Error> {
        let storg = NewStorage {
            name: self.name.clone(),
            manufacturer: self.manufacturer.clone(),
            capacity: self.capacity,
            speed: self.speed,
            type_: self.type_.clone(),
            price: self.price,
        };

        diesel::insert_into(storages::table)
            .values(&storg)
            .get_result(conn)
    }

    fn read_all(&self, conn: &DBPooledConnection) -> Result<Vec<Storage>, diesel::result::Error> {
        storages.load::<Storage>(conn)
    }

    fn update(
        &self,
        conn: &DBPooledConnection,
        other: Storage,
    ) -> Result<Storage, diesel::result::Error> {
        diesel::update(storages.filter(name.eq(&self.name)))
            .set((
                name.eq(other.name),
                manufacturer.eq(other.manufacturer),
                capacity.eq(other.capacity),
                speed.eq(other.speed),
                type_.eq(other.type_),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(storages.filter(name.eq(&self.name))).execute(conn)
    }
}
