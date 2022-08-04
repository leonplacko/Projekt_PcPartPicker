//use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, dsl::count_star};

use crate::database::{
    storage_slots::{contract::CRUD, data::*},
    //schema::storage_slots::dsl::*
};

use super::contract::DBPooledConnection;

fn handle(storslot: String, conn: &DBPooledConnection, id_: String) -> Result<(), diesel::result::Error>{
    let buildsize_data = STRGslots::read(storslot.clone(), conn);

    if let Ok(val) = buildsize_data{
        
        for data in val{
            let build = NewSTRGslots{
                motherboard_id: Some(id_.clone()),
                storage_id: data.storage_id.clone(),
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
            motherboard_id: Some(id_.clone()),
            storage_id: None,
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

pub fn handle_storage_slot_insert(sataslot: String, nvmeslot: String, conn: &DBPooledConnection, id_: String) -> Result<(), diesel::result::Error>{
    
    if sataslot == "Sata" && nvmeslot == "NVMe"{
        let res1 = handle(sataslot, conn, id_.clone());
        let res2 = handle(nvmeslot, conn, id_.clone());

        return match(res1, res2){
            (Ok(_), Ok(_)) => Ok(()),
            (Err(er), _) | (Ok(_), Err(er)) => Err(er),
        }

    }else if sataslot == "Sata" && nvmeslot == "None"{
        return handle(sataslot, conn, id_.clone())
    }else if sataslot == "None" && nvmeslot == "NVMe"{
        return handle(nvmeslot, conn, id_.clone())
    }else{
        return Ok(())
    }

    
}


