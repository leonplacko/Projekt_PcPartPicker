extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::cpu;
use schema::cpu::dsl::*;

use super::handle_socket::*;

impl CRUD for CPU {
    fn create(n_cpu: ExtendCPU, conn: &DBPooledConnection) -> Result<CPU, diesel::result::Error> {
        
        let sockt = n_cpu.socket.clone();

        let ins_cpu = NewCPU::from(n_cpu);

        let rez1 = diesel::insert_into(cpu::table)
            .values(&ins_cpu)
            .get_result::<CPU>(conn);

        match rez1{
            Err(er) => Err(er),
            Ok(value) => match handle_socket_insert(sockt, conn, value.id.clone()){
                Ok(_) => Ok(value),
                Err(er) => Err(er),
            } 
        }
    }

    fn read_all(conn: &DBPooledConnection) -> Result<Vec<CPU>, diesel::result::Error> {
        cpu.load::<CPU>(conn)
    }

    fn update(conn: &DBPooledConnection, other: NewCPU) -> Result<CPU, diesel::result::Error> {
        diesel::update(cpu.filter(name.eq(other.name)))
            .set((
                manufacturer.eq(other.manufacturer),
                speed.eq(other.speed),
                cores.eq(other.cores),
                cache.eq(other.cache),
                tdp.eq(other.tdp),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(cpuname: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(cpu.filter(name.eq(cpuname))).execute(conn)
    
    }
}
