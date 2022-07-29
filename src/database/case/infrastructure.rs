extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::pc_case;
use schema::pc_case::dsl::*;

impl CRUD for Case {
    fn create(case: NewCase, conn: &DBPooledConnection) -> Result<Case, diesel::result::Error> {
        
        diesel::insert_into(pc_case::table)
            .values(&case)
            .get_result(conn)
    }

    fn read_all(conn: &DBPooledConnection) -> Result<Vec<Case>, diesel::result::Error> {
        pc_case.load::<Case>(conn)
    }

    fn update(conn: &DBPooledConnection, other: Case) -> Result<Case, diesel::result::Error> {
        diesel::update(pc_case.filter(name.eq(other.name.clone())))
            .set((
                name.eq(other.name),
                manufacturer.eq(other.manufacturer),
                dimensions.eq(other.dimensions),
                ventilators.eq(other.ventilators),
                available_vent.eq(other.available_vent),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(_name: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(pc_case.filter(name.eq(&_name))).execute(conn)
    }
}
