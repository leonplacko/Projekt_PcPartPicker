extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::build_size;
use schema::build_size::dsl::*;

impl CRUD for BuildSize {
    fn create(
        build: NewBuildSize,
        conn: &DBPooledConnection,
    ) -> Result<BuildSize, diesel::result::Error> {
        diesel::insert_into(build_size::table)
            .values(&build)
            .get_result(conn)
    }

    fn read(
        size_: String,
        conn: &DBPooledConnection,
    ) -> Result<Vec<BuildSize>, diesel::result::Error> {
        build_size.filter(size.eq(size_)).load::<BuildSize>(conn)
    }

    fn update(
        conn: &DBPooledConnection,
        other: BuildSize,
    ) -> Result<BuildSize, diesel::result::Error> {
        diesel::update(
            build_size
                .filter(motherboard_id.eq(other.motherboard_id))
                .filter(case_id.eq(other.case_id)),
        )
        .set(size.eq(other.size))
        .get_result(conn)
    }

    fn delete(
        mbid: Option<String>,
        csid: Option<String>,
        conn: &DBPooledConnection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(
            build_size
                .filter(motherboard_id.eq(&mbid))
                .filter(case_id.eq(&csid)),
        )
        .execute(conn)
    }
}
