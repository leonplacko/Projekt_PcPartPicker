//use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, dsl::count_star};

use crate::database::storage_slots::{contract::CRUD, data::*};

use super::contract::DBPooledConnection;

pub fn handle_storage_slot_insert(
    storslot: String,
    conn: &DBPooledConnection,
    id_: String,
) -> Result<(), diesel::result::Error> {
    let storslot_data = STRGslots::read(storslot.clone(), conn);

    match storslot_data {
        Ok(val) => {
            let new_storslot = NewSTRGslots {
                motherboard_id: None,
                storage_id: Some(id_.clone()),
                slot: storslot.clone(),
            };
            let rez = STRGslots::create(new_storslot, conn);
            if let Err(er) = rez {
                return Err(er);
            } else {
                for data in val {
                    if data.storage_id != None {
                        continue;
                    } else {
                        let build = NewSTRGslots {
                            motherboard_id: data.motherboard_id.clone(),
                            storage_id: Some(id_.clone()),
                            slot: storslot.clone(),
                        };
                        let rez = STRGslots::create(build, conn);
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
