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
    fn create(
        pu: NewPowerUnit,
        conn: &DBPooledConnection,
    ) -> Result<PowerUnit, diesel::result::Error> {
        diesel::insert_into(power_unit::table)
            .values(&pu)
            .get_result(conn)
    }

    fn read_all(conn: &DBPooledConnection) -> Result<Vec<PowerUnit>, diesel::result::Error> {
        power_unit.load::<PowerUnit>(conn)
    }

    fn update(
        conn: &DBPooledConnection,
        other: NewPowerUnit,
    ) -> Result<PowerUnit, diesel::result::Error> {
        diesel::update(power_unit.filter(name.eq(other.name)))
            .set((
                manufacturer.eq(other.manufacturer),
                power.eq(other.power),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(name_: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(power_unit.filter(name.eq(name_))).execute(conn)
    }
}
