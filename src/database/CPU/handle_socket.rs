//use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, dsl::count_star};

use crate::database::socket::{contract::CRUD, data::*};

use super::contract::DBPooledConnection;

pub fn handle_socket_insert(
    sockt: String,
    conn: &DBPooledConnection,
    id_: String,
) -> Result<(), diesel::result::Error> {
    let socket_data = Socket::read(sockt.clone(), conn);

    match socket_data {
        Ok(val) => {
            let sckt = NewSocket {
                motherboard_id: None,
                cpu_id: Some(id_.clone()),
                socket_type: sockt.clone(),
            };
            let rez = Socket::create(sckt, conn);
            if let Err(er) = rez {
                return Err(er);
            } else {
                for data in val {
                    if data.cpu_id != None {
                        continue;
                    } else {
                        let build = NewSocket {
                            motherboard_id: data.motherboard_id.clone(),
                            cpu_id: Some(id_.clone()),
                            socket_type: sockt.clone(),
                        };
                        let rez = Socket::create(build, conn);
                        if let Err(er) = rez {
                            return Err(er);
                        } else {
                            continue;
                        }
                    }
                }
                return Ok(());
            }
        }

        Err(er) => Err(er),
    }
}
