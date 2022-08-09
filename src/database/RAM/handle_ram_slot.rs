//use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, dsl::count_star};

use crate::database::{
    ram_slot::{contract::CRUD, data::*},
    //schema::ram_slot::dsl::*
};

use super::contract::DBPooledConnection;

pub fn handle_ram_slot_insert(slotype: String, conn: &DBPooledConnection, id_: String) -> Result<(), diesel::result::Error>{
    let ramslot_data = RamSlot::read(slotype.clone(), conn);

    match ramslot_data{
        Ok(val) => {
            if val.len() == 0{
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
            }else{
                for data in val{
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
            }
        },

        Err(er) => Err(er)
    }
    
        
}


