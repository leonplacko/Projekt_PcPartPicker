//use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, dsl::count_star};

use crate::database::{
    build_size::{contract::CRUD, data::*},
    //schema::build_size::dsl::*
};

use super::contract::DBPooledConnection;

pub fn handle_build_size_insert(buildsize: String, conn: &DBPooledConnection, id_: String) -> Result<(), diesel::result::Error>{
    let buildsize_data = BuildSize::read(buildsize.clone(), conn);

    if let Ok(val) = buildsize_data{
        
        for data in val{
            let build = NewBuildSize{
                motherboard_id: Some(id_.clone()),
                case_id: data.case_id.clone(),
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
    }else if let Err(diesel::result::Error::NotFound) = buildsize_data{
        let build = NewBuildSize{
            motherboard_id: Some(id_.clone()),
            case_id: None,
            size: buildsize.clone(),
        };
        let rez = BuildSize::create(build, conn);
        if let Err(er) = rez{
            return Err(er);
        }else{
            return Ok(())
        } 
    }else if let Err(er) = buildsize_data{
        return Err(er);
    }else{
        println!("Unidentified case");
        return Ok(())
    }
}


