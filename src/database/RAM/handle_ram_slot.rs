//use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, dsl::count_star};

use crate::database::{
    ram_slot::{contract::CRUD, data::*},
    //schema::ram_slot::dsl::*
};

use super::contract::DBPooledConnection;

pub fn handle_ram_slot_insert(slotype: String, conn: &DBPooledConnection, id_: String) -> Result<(), diesel::result::Error>{
    let buildsize_data = RamSlot::read(slotype.clone(), conn);

    if let Ok(val) = buildsize_data{
        
        for data in val{
            /*let ramid = data.ram_id.clone();
            if ramid == None{
                let delres = RamSlot::delete(data.motherboard_id.clone(), ramid, conn);
                if let Err(er) = delres{
                    return Err(er)
                }
            }*/
            let build = NewRamSlot{
                motherboard_id: data.motherboard_id.clone(),
                ram_id: Some(id_.clone()),
                type_: slotype.clone(),
            };
            let rez = RamSlot::create(build, conn);
            if let Err(er) = rez{
                return Err(er);
            }else{
                continue;
            }            
        }
        return Ok(())
    }else if let Err(diesel::result::Error::NotFound) = buildsize_data{
        let ramslot = NewRamSlot{
            motherboard_id: None,
            ram_id: Some(id_.clone()),
            type_: slotype.clone(),
        };
        let rez = RamSlot::create(ramslot, conn);
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
pub fn handle_ram_slot_delete(conn: &DBPooledConnection, ramid: Option<String>) -> Result<usize, diesel::result::Error>{
    let buildsize_data = ram_slot.filter(ram_id.eq(ramid)).load::<RamSlot>(conn);
    if let Err(er) = buildsize_data{
        return Err(er)
    }else{
        let Ok(val) = buildsize_data;
        let succesfull_result = val.len();
        for data in val{
            let mbid = data.motherboard_id.clone();
            if mbid == None{
                let del_res = RamSlot::delete(mbid, ramid, conn);
                if let Err(er) = del_res{
                    return Err(er)
                }
            }else {
                let count: Result<i64, diesel::result::Error> = ram_slot.filter(motherboard_id.eq(mbid)).select(count_star()).first(conn);
                if let Err(er) = count{
                    return Err(er)
                }else{
                    let Ok(val) = count;
                    if !(val > 1){
                        let ramslot = NewRamSlot{
                            motherboard_id: mbid,
                            ram_id: None,
                            type_: data.type_.clone(),
                        };
                        let ins_res = RamSlot::create(ramslot, conn);
                        let del_res = RamSlot::delete(mbid, ramid, conn);
                        if let Err(er) = ins_res{
                            return Err(er)
                        }
                        if let Err(er) = del_res{
                            return Err(er);
                        }
                    }else{
                        let del_res = RamSlot::delete(mbid, ramid, conn);
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

