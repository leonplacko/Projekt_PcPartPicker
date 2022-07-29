extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::socket;
use schema::socket::dsl::*;

impl CRUD for Socket {
    fn create(&self, conn: &DBPooledConnection) -> Result<Socket, diesel::result::Error> {
        diesel::insert_into(socket::table)
            .values(&*self)
            .get_result(conn)
    }

    fn read_all(&self, conn: &DBPooledConnection) -> Result<Vec<Socket>, diesel::result::Error> {
        socket.load::<Socket>(conn)
    }

    fn update(&self, conn: &DBPooledConnection, other: Socket) -> Result<Socket, diesel::result::Error> {
        diesel::update(
            socket
                .filter(motherboard_id.eq(&self.motherboard_id))
                .filter(cpu_id.eq(&self.cpu_id)),
        )
        .set(socket_type.eq(&other.socket_type))
        .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(
            socket
                .filter(motherboard_id.eq(&self.motherboard_id))
                .filter(cpu_id.eq(&self.cpu_id)),
        )
        .execute(conn)
    }
}
