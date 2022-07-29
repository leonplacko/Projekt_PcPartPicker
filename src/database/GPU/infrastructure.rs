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
    fn create(&self, conn: &DBPooledConnection) -> Result<GPU, diesel::result::Error> {
        let gp = NewGPU {
            name: self.name.clone(),
            manufacturer: self.manufacturer.clone(),
            memory: self.memory,
            memory_type: self.memory_type.clone(),
            tdp: self.tdp,
            price: self.price,
        };

        diesel::insert_into(gpu::table).values(&gp).get_result(conn)
    }

    fn read(&self, conn: &DBPooledConnection) -> Result<Vec<GPU>, diesel::result::Error> {
        gpu.load::<GPU>(conn)
    }

    fn update(&self, conn: &DBPooledConnection, other: GPU) -> Result<GPU, diesel::result::Error> {
        diesel::update(gpu.filter(name.eq(&self.name)))
            .set((
                name.eq(other.name),
                manufacturer.eq(other.manufacturer),
                memory.eq(other.memory),
                memory_type.eq(other.memory_type),
                tdp.eq(self.tdp),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(gpu.filter(name.eq(&self.name))).execute(conn)
    }
}
