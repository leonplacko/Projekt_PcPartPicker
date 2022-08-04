extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::gpu;
use schema::gpu::dsl::*;

impl CRUD for GPU {
    fn create(gp: NewGPU, conn: &DBPooledConnection) -> Result<GPU, diesel::result::Error> {
        
        diesel::insert_into(gpu::table).values(&gp).get_result(conn)
    }

    fn read_all(conn: &DBPooledConnection) -> Result<Vec<GPU>, diesel::result::Error> {
        gpu.load::<GPU>(conn)
    }

    fn update(conn: &DBPooledConnection, other: NewGPU) -> Result<GPU, diesel::result::Error> {
        diesel::update(gpu.filter(name.eq(other.name)))
            .set((
                manufacturer.eq(other.manufacturer),
                memory.eq(other.memory),
                memory_type.eq(other.memory_type),
                tdp.eq(other.tdp),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(name_: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(gpu.filter(name.eq(name_))).execute(conn)
    }
}
