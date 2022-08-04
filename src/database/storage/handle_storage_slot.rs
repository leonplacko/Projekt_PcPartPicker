//use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, dsl::count_star};

use crate::database::{
    storage_slots::{contract::CRUD, data::*},
    //schema::storage_slots::dsl::*
};

use super::contract::DBPooledConnection;

pub fn handle_storage_slot_insert(storslot: String, conn: &DBPooledConnection, id_: String) -> Result<(), diesel::result::Error>{
    let buildsize_data = STRGslots::read(storslot.clone(), conn);

    if let Ok(val) = buildsize_data{
        
        for data in val{
            /*let storid = data.storage_id.clone();
            if storid == None{
                let delres = STRGslots::delete(data.motherboard_id.clone(), storid, conn);
                if let Err(er) = delres{
                    return Err(er)
                }
            }*/
            let build = NewSTRGslots{
                motherboard_id: data.motherboard_id.clone(),
                storage_id: Some(id_.clone()),
                slot: storslot.clone(),
            };
            let rez = STRGslots::create(build, conn);
            if let Err(er) = rez{
                return Err(er);
            }else{
                continue;
            }            
        }
        return Ok(())
    }else if let Err(diesel::result::Error::NotFound) = buildsize_data{
        let new_storslot = NewSTRGslots{
            motherboard_id: None,
            storage_id: Some(id_.clone()),
            slot: storslot.clone(),
        };
        let rez = STRGslots::create(new_storslot, conn);
        if let Err(er) = rez{
            return Err(er);
        }else{
            return Ok(())
        } 
    }else if let Err(er) = buildsize_data{
        return Err(er);
    }else{
        println!("Unindentified case");
        return Ok(())
    }
}
/*
pub fn handle_storage_slot_delete(conn: &DBPooledConnection, storid: Option<String>) -> Result<usize, diesel::result::Error>{
    let buildsize_data = storage_slots.filter(storage_id.eq(storid)).load::<STRGslots>(conn);
    if let Err(er) = buildsize_data{
        return Err(er)
    }else{
        let Ok(val) = buildsize_data;
        let succesfull_result = val.len();
        for data in val{
            let mbid = data.motherboard_id.clone();
            if mbid == None{
                let del_res = STRGslots::delete(mbid, storid, conn);
                if let Err(er) = del_res{
                    return Err(er)
                }
            }else {
                let count: Result<i64, diesel::result::Error> = storage_slots.filter(motherboard_id.eq(mbid)).select(count_star()).first(conn);
                if let Err(er) = count{
                    return Err(er)
                }else{
                    let Ok(val) = count;
                    if !(val > 1){
                        let new_storslot = NewSTRGslots{
                            motherboard_id: mbid,
                            storage_id: None,
                            slot: data.slot.clone(),
                        };
                        let ins_res = STRGslots::create(new_storslot, conn);
                        let del_res = STRGslots::delete(mbid, storid, conn);
                        if let Err(er) = ins_res{
                            return Err(er)
                        }
                        if let Err(er) = del_res{
                            return Err(er);
                        }
                    }else{
                        let del_res = STRGslots::delete(mbid, storid, conn);
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

