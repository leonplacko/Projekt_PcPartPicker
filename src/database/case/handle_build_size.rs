use crate::database::{
    build_size::{contract::CRUD, data::*},
};

use super::contract::DBPooledConnection;

pub fn handle_build_size_insert(buildsize: String, conn: &DBPooledConnection, id_: String) -> Result<(), diesel::result::Error>{
    let buildsize_data = BuildSize::read(buildsize.clone(), conn);

    match buildsize_data{
        Ok(val) => {
            if val.len() == 0{
                let build = NewBuildSize{
                    motherboard_id: None,
                    case_id: Some(id_.clone()),
                    size: buildsize.clone(),
                };
                let rez = BuildSize::create(build, conn);
                if let Err(er) = rez{
                    return Err(er);
                }else{
                    return Ok(())
                }
            }else{
                for data in val{
                    let build = NewBuildSize{
                        motherboard_id: data.motherboard_id.clone(),
                        case_id: Some(id_.clone()),
                        size: buildsize.clone(),
                    };
                    let rez = BuildSize::create(build, conn);
                    if let Err(er) = rez{
                        return Err(er);
                    }else{
                        continue;
                    }            
                }
                return Ok(())
            }
        }
        
        Err(er) => Err(er),
    }
}

