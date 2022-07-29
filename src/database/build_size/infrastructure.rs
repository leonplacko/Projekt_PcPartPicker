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
    fn create(&self, conn: &DBPooledConnection) -> Result<BuildSize, diesel::result::Error> {
        let build = BuildSize {
            motherboard_id: self.motherboard_id.clone(),
            case_id: self.case_id.clone(),
            size: self.size.clone(),
        };

        diesel::insert_into(build_size::table)
            .values(&build)
            .get_result(conn)
    }

    fn read_all(&self, conn: &DBPooledConnection) -> Result<Vec<BuildSize>, diesel::result::Error> {
        build_size.load::<BuildSize>(conn)
    }

    fn update(
        &self,
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

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(
            build_size
                .filter(motherboard_id.eq(&self.motherboard_id))
                .filter(case_id.eq(&self.case_id)),
        )
        .execute(conn)
    }
}
