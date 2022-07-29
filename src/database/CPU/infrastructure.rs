extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::cpu;
use schema::cpu::dsl::*;

impl CRUD for CPU {
    fn create(&self, conn: &DBPooledConnection) -> Result<CPU, diesel::result::Error> {
        let n_cpu = NewCPU {
            name: self.name.clone(),
            manufacturer: self.manufacturer.clone(),
            cores: self.cores,
            cache: self.cache.clone(),
            speed: self.speed,
            tdp: self.tdp,
            price: self.price,
        };

        diesel::insert_into(cpu::table)
            .values(&n_cpu)
            .get_result(conn)
    }

    fn read(&self, conn: &DBPooledConnection) -> Result<Vec<CPU>, diesel::result::Error> {
        cpu.load::<CPU>(conn)
    }

    fn update(&self, conn: &DBPooledConnection, other: CPU) -> Result<CPU, diesel::result::Error> {
        diesel::update(cpu.filter(name.eq(&self.name)))
            .set((
                name.eq(other.name),
                manufacturer.eq(other.manufacturer),
                speed.eq(other.speed),
                cores.eq(other.cores),
                cache.eq(other.cache),
                tdp.eq(other.tdp),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(cpu.filter(name.eq(&self.name))).execute(conn)
    }
}
