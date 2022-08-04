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
            /*let csid = data.case_id.clone();
            if csid == None{
                let delres = BuildSize::delete(data.motherboard_id.clone(), csid, conn);
                if let Err(er) = delres{
                    return Err(er)
                }
            }*/
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
    }else if let Err(diesel::result::Error::NotFound) = buildsize_data{
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
    }else if let Err(er) = buildsize_data{
        return Err(er);
    }else{
        println!("Unidentified case");
        return Ok(())
    }
}

/*pub fn handle_build_size_delete(conn: &DBPooledConnection, csid: Option<String>) -> Result<usize, diesel::result::Error>{
    let buildsize_data = build_size.filter(case_id.eq(csid)).load::<BuildSize>(conn);
    if let Err(er) = buildsize_data{
        return Err(er)
    }else{
        let Ok(val) = buildsize_data;
        let succesfull_result = val.len();
        for data in val{
            let mbid = data.motherboard_id.clone();
            if mbid == None{
                let del_res = BuildSize::delete(mbid, csid, conn);
                if let Err(er) = del_res{
                    return Err(er)
                }
            }else {
                let count: Result<i64, diesel::result::Error> = build_size.filter(motherboard_id.eq(mbid)).select(count_star()).first(conn);
                if let Err(er) = count{
                    return Err(er)
                }else{
                    let Ok(val) = count;
                    if !(val > 1){
                        let build = NewBuildSize{
                            motherboard_id: mbid,
                            case_id: None,
                            size: data.size.clone(),
                        };
                        let ins_res = BuildSize::create(build, conn);
                        let del_res = BuildSize::delete(mbid, csid, conn);
                        if let Err(er) = ins_res{
                            return Err(er)
                        }
                        if let Err(er) = del_res{
                            return Err(er);
                        }
                    }else{
                        let del_res = BuildSize::delete(mbid, csid, conn);
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

