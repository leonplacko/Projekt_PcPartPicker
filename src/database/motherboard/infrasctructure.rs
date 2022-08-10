extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::motherboards;
use schema::motherboards::dsl::*;

use super::{handle_build_size::*, handle_ram_slot::*, handle_socket::*, handle_storage_slot::*};

impl CRUD for Motherboard {
    fn create(
        exmb: ExtendMotherboard,
        conn: &DBPooledConnection,
    ) -> Result<Motherboard, diesel::result::Error> {
        let mb_data = vec![
            exmb.socket.clone(),
            exmb.ram_type.clone(),
            exmb.build_size.clone(),
            if exmb.nvme_slots > 0 {
                String::from("NVMe")
            } else {
                String::from("None")
            },
            if exmb.sata_slots > 0 {
                String::from("Sata")
            } else {
                String::from("None")
            },
        ];

        let mb = NewMotherboard::from(exmb);

        let res1 = diesel::insert_into(motherboards::table)
            .values(&mb)
            .get_result::<Motherboard>(conn);

        match res1 {
            Err(er) => Err(er),
            Ok(val) => match (
                handle_build_size_insert(mb_data[2].clone(), conn, val.id.clone()),
                handle_ram_slot_insert(mb_data[1].clone(), conn, val.id.clone()),
                handle_storage_slot_insert(
                    mb_data[4].clone(),
                    mb_data[3].clone(),
                    conn,
                    val.id.clone(),
                ),
                handle_socket_insert(mb_data[0].clone(), conn, val.id.clone()),
            ) {
                (Ok(_), Ok(_), Ok(_), Ok(_)) => Ok(val),
                (Err(er), _, _, _)
                | (Ok(_), Err(er), _, _)
                | (Ok(_), Ok(_), Err(er), _)
                | (Ok(_), Ok(_), Ok(_), Err(er)) => Err(er),
            },
        }
    }

    fn read(conn: &DBPooledConnection) -> Result<Vec<Motherboard>, diesel::result::Error> {
        motherboards.load::<Motherboard>(conn)
    }

    fn update(
        conn: &DBPooledConnection,
        other: NewMotherboard,
    ) -> Result<Motherboard, diesel::result::Error> {
        diesel::update(motherboards.filter(name.eq(other.name)))
            .set((
                manufacturer.eq(other.manufacturer),
                ram_slots.eq(other.ram_slots),
                sata_slots.eq(other.sata_slots),
                nvme_slots.eq(other.nvme_slots),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(name_: String, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(motherboards.filter(name.eq(name_))).execute(conn)
    }
}
