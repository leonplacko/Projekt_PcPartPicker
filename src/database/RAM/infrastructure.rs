extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::ram;
use schema::ram::dsl::*;

use super::handle_ram_slot::*;

impl CRUD for RAM {
    fn create(ram_: ExtendRAM, conn: &DBPooledConnection) -> Result<RAM, diesel::result::Error> {
        
        let ramslot = ram_.slot.clone();

        let n_ram = NewRAM::from(ram_);

        let rez1 = diesel::insert_into(ram::table)
            .values(&n_ram)
            .get_result::<RAM>(conn);
        
        match rez1{
            Err(er) => Err(er),
            Ok(val) => match handle_ram_slot_insert(ramslot, conn, val.id.clone()){
                Ok(_) => Ok(val),
                Err(er) => Err(er),
            }
        }

    }

    fn read_all(conn: &DBPooledConnection) -> Result<Vec<RAM>, diesel::result::Error> {
        ram.load::<RAM>(conn)
    }

    fn update(conn: &DBPooledConnection, other: NewRAM) -> Result<RAM, diesel::result::Error> {
        diesel::update(ram.filter(name.eq(other.name)))
            .set((
                manufacturer.eq(other.manufacturer),
                capacity.eq(other.capacity),
                speed.eq(other.speed),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(name_: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
                        
        diesel::delete(ram.filter(name.eq(&name_))).execute(conn)
        
    }
}
