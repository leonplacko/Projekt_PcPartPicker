extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::cooling;
use schema::cooling::dsl::*;

impl CRUD for Cooling {
    fn create(
        cool: NewCooling,
        conn: &DBPooledConnection,
    ) -> Result<Cooling, diesel::result::Error> {
        diesel::insert_into(cooling::table)
            .values(&cool)
            .get_result(conn)
    }

    fn read_all(conn: &DBPooledConnection) -> Result<Vec<Cooling>, diesel::result::Error> {
        cooling.load::<Cooling>(conn)
    }

    fn update(
        conn: &DBPooledConnection,
        other: NewCooling,
    ) -> Result<Cooling, diesel::result::Error> {
        diesel::update(cooling.filter(name.eq(other.name)))
            .set((
                manufacturer.eq(other.manufacturer),
                ventilators.eq(other.ventilators),
                cpu_ventilator.eq(other.cpu_ventilator),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(name_: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(cooling.filter(name.eq(name_))).execute(conn)
    }
}
