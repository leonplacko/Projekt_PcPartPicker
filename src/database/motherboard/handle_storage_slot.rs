//use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, dsl::count_star};

use crate::database::storage_slots::{contract::CRUD, data::*};

use super::contract::DBPooledConnection;

fn handle(
    storslot: String,
    conn: &DBPooledConnection,
    id_: String,
) -> Result<(), diesel::result::Error> {
    let storslot_data = STRGslots::read(storslot.clone(), conn);

    match storslot_data {
        Ok(val) => {
            let new_storslot = NewSTRGslots {
                motherboard_id: Some(id_.clone()),
                storage_id: None,
                slot: storslot.clone(),
            };
            let rez = STRGslots::create(new_storslot, conn);
            if let Err(er) = rez {
                return Err(er);
            } else {
                for data in val {
                    if data.motherboard_id != None {
                        continue;
                    } else {
                        let build = NewSTRGslots {
                            motherboard_id: Some(id_.clone()),
                            storage_id: data.storage_id.clone(),
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

pub fn handle_storage_slot_insert(
    sataslot: String,
    nvmeslot: String,
    conn: &DBPooledConnection,
    id_: String,
) -> Result<(), diesel::result::Error> {
    if sataslot == "Sata" && nvmeslot == "NVMe" {
        println!("slucaj 1");
        let res1 = handle(sataslot, conn, id_.clone());
        let res2 = handle(nvmeslot, conn, id_.clone());

        return match (res1, res2) {
            (Ok(_), Ok(_)) => Ok(()),
            (Err(er), _) | (Ok(_), Err(er)) => Err(er),
        };
    } else if sataslot == "Sata" && nvmeslot == "None" {
        println!("slucaj 2");
        return handle(sataslot, conn, id_.clone());
    } else if sataslot == "None" && nvmeslot == "NVMe" {
        println!("slucaj 3");
        return handle(nvmeslot, conn, id_.clone());
    } else {
        println!("slucaj 4");
        return Ok(());
    }
}
