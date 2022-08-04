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
            let build = NewRamSlot{
                motherboard_id: Some(id_.clone()),
                ram_id: data.ram_id.clone(),
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
            motherboard_id: Some(id_.clone()),
            ram_id: None,
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


