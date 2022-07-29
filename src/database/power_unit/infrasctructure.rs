extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::power_unit;
use schema::power_unit::dsl::*;

impl CRUD for PowerUnit {
    fn create(&self, conn: &DBPooledConnection) -> Result<PowerUnit, diesel::result::Error> {
        let pu = NewPowerUnit {
            name: self.name.clone(),
            manufacturer: self.manufacturer.clone(),
            power: self.power,
            price: self.price,
        };

        diesel::insert_into(power_unit::table)
            .values(&pu)
            .get_result(conn)
    }

    fn read_all(&self, conn: &DBPooledConnection) -> Result<Vec<PowerUnit>, diesel::result::Error> {
        power_unit.load::<PowerUnit>(conn)
    }

    fn update(
        &self,
        conn: &DBPooledConnection,
        other: PowerUnit,
    ) -> Result<PowerUnit, diesel::result::Error> {
        diesel::update(power_unit.filter(name.eq(&self.name)))
            .set((
                name.eq(other.name),
                manufacturer.eq(other.manufacturer),
                power.eq(other.power),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(power_unit.filter(name.eq(&self.name))).execute(conn)
    }
}
