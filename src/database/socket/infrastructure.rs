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
    fn create(sock: NewSocket, conn: &DBPooledConnection) -> Result<Socket, diesel::result::Error> {
        diesel::insert_into(socket::table)
            .values(&sock)
            .get_result(conn)
    }

    fn read(
        sockt: String,
        conn: &DBPooledConnection,
    ) -> Result<Vec<Socket>, diesel::result::Error> {
        socket.filter(socket_type.eq(&sockt)).load::<Socket>(conn)
    }

    fn update(conn: &DBPooledConnection, other: Socket) -> Result<Socket, diesel::result::Error> {
        diesel::update(
            socket
                .filter(motherboard_id.eq(&other.motherboard_id))
                .filter(cpu_id.eq(&other.cpu_id)),
        )
        .set(socket_type.eq(&other.socket_type))
        .get_result(conn)
    }

    fn delete(
        mbid: Option<String>,
        cpuid: Option<String>,
        conn: &DBPooledConnection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(
            socket
                .filter(motherboard_id.eq(mbid))
                .filter(cpu_id.eq(cpuid)),
        )
        .execute(conn)
    }
}
