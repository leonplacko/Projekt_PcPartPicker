//use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, dsl::count_star};

use crate::database::{
    socket::{contract::CRUD, data::*},
    //schema::socket::dsl::*
};

use super::contract::DBPooledConnection;

pub fn handle_socket_insert(sockt: String, conn: &DBPooledConnection, id_: String) -> Result<(), diesel::result::Error>{
    let socket_data = Socket::read(sockt.clone(), conn);

    
    if let Ok(val) = socket_data{
        for data in val{
            /*let cpuid = data.cpu_id.clone();
            if cpuid == None{
                let delres = Socket::delete(data.motherboard_id.clone(), cpuid, conn);
                if let Err(er) = delres{
                    return Err(er)
                }
            }*/
            let build = NewSocket{
                motherboard_id: data.motherboard_id.clone(),
                cpu_id: Some(id_.clone()),
                socket_type: sockt.clone(),
            };
            let rez = Socket::create(build, conn);
            if let Err(er) = rez{
                return Err(er);
            }else{
                continue;
            }            
        }
        return Ok(())
    }else if let Err(diesel::result::Error::NotFound) = socket_data{
        let sckt = NewSocket{
            motherboard_id: None,
            cpu_id: Some(id_.clone()),
            socket_type: sockt.clone(),
        };
        println!("checkpoint 4");
        let rez = Socket::create(sckt, conn);
        if let Err(er) = rez{
            return Err(er);
        }else{
            return Ok(())
        } 
    }else if let Err(er) = socket_data{
        println!("checkpoint 5");
        return Err(er);
    }else{
        println!("Unindentified case");
        return Ok(())
    }
}
/*
pub fn handle_socket_delete(conn: &DBPooledConnection, cpuid: Option<String>) -> Result<usize, diesel::result::Error>{
    let socket_data = socket.filter(cpu_id.eq(cpuid)).load::<Socket>(conn);
    if let Err(er) = socket_data{
        return Err(er)
    }else{
        let Ok(val) = socket_data;
        let succesfull_result = val.len();
        for data in val{
            let mbid = data.motherboard_id.clone();
            if mbid == None{
                let del_res = Socket::delete(mbid, cpuid, conn);
                if let Err(er) = del_res{
                    return Err(er)
                }
            }else {
                let count: Result<i64, diesel::result::Error> = socket.filter(motherboard_id.eq(mbid)).select(count_star()).first(conn);
                if let Err(er) = count{
                    return Err(er)
                }else{
                    let Ok(val) = count;
                    if !(val > 1){
                        let sckt = NewSocket{
                            motherboard_id: mbid,
                            cpu_id: None,
                            socket_type: data.socket_type.clone(),
                        };
                        let ins_res = Socket::create(sckt, conn);
                        let del_res = Socket::delete(mbid, cpuid, conn);
                        if let Err(er) = ins_res{
                            return Err(er)
                        }
                        if let Err(er) = del_res{
                            return Err(er);
                        }
                    }else{
                        let del_res = Socket::delete(mbid, cpuid, conn);
                        if let Err(er) = del_res{
                            return Err(er)
                        }
                    }
                }
            }
        }
        return Ok(succesfull_result)
    }
}*/

