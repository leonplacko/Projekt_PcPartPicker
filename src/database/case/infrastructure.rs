extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::pc_case;
use schema::pc_case::dsl::*;

use super::handle_build_size::*;

impl CRUD for Case {
    fn create(case: ExtendCase, conn: &DBPooledConnection) -> Result<Case, diesel::result::Error> {
        let buildsize = case.build_size.clone();

        let newcase = NewCase::from(case);

        let rez1 = diesel::insert_into(pc_case::table)
            .values(&newcase)
            .get_result::<Case>(conn);

        match rez1 {
            Err(er) => Err(er),
            Ok(value) => match handle_build_size_insert(buildsize, conn, value.id.clone()) {
                Ok(_) => Ok(value),
                Err(er) => Err(er),
            },
        }
    }

    fn read_all(conn: &DBPooledConnection) -> Result<Vec<Case>, diesel::result::Error> {
        pc_case.load::<Case>(conn)
    }

    fn update(conn: &DBPooledConnection, other: NewCase) -> Result<Case, diesel::result::Error> {
        diesel::update(pc_case.filter(name.eq(other.name)))
            .set((
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
