use actix_identity::Identity;
use r2d2::{Pool, PooledConnection};
use rand::{thread_rng, seq::SliceRandom};
use diesel::{
    r2d2::ConnectionManager,
    PgConnection,
    query_dsl::{RunQueryDsl, QueryDsl},
    ExpressionMethods,
    dsl::*,
};

use actix_web::{web::{Path, Data, Json}, HttpResponse};

use super::build_models::*;

use crate::database::{
    GPU::data::*,
    power_unit::data::*,
    motherboard::data::*,
    cooling::data::*,
    schema::{
        cpu::{self, dsl::*},
        gpu::{self, dsl::*},
        ram::{self, dsl::*},
        storages::{self, dsl::*},
        power_unit::{self, dsl::*},
        motherboards::{self, dsl::*},
        cooling::{self, dsl::*},
        pc_case::{self, dsl::*},
        socket::dsl::*,
        build_size::dsl::*,
        storage_slots::dsl::*,
        ram_slot::dsl::*,
    }
};
use crate::response_error_handling::ErrorHandler;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn deploy_build(rams: Vec<String>, storagess: Vec<String>, coolings: Vec<String>, deploydata: Vec<String>, myprice: f32) -> HttpResponse{
        
    let genbuild = GeneratedBuild{
        motherboard_name: deploydata[0].clone(),
        cpu_name: deploydata[1].clone(),
        gpu_name: deploydata[2].clone(),
        storage_name: storagess,
        ram_name: rams,
        pu_name: deploydata[3].clone(),
        case_name: deploydata[4].clone(),
        cooling: coolings,
        price: myprice,
    };

    HttpResponse::Ok().content_type("application/json").json(genbuild)
}


fn get_cheap(pool: &DBPooledConnection) -> Result<f32, diesel::result::Error>{
    
    let min_gpu = gpu.select(min(gpu::price)).first::<Option<f32>>(pool);
    let min_cool = cooling.select(min(cooling::price)).first::<Option<f32>>(pool);
    let min_power = power_unit.select(min(power_unit::price)).first::<Option<f32>>(pool);

    match (min_gpu, min_cool, min_power){
        (Ok(Some(val1)), Ok(Some(val2)), Ok(Some(val3))) => Ok(val1 + val2 + val3),
        (Ok(None), Ok(_), Ok(_)) | (Ok(_) ,Ok(None), Ok(_)) | (Ok(_), Ok(_), Ok(None)) => Ok(0.0),
        (Err(er), _, _) | (Ok(_), Err(er), _) | (Ok(_), Ok(_), Err(er)) => Err(er),
    }

}



pub async fn gen_price(path: Path<String>, conn: Data<DBPool>, data: Json<UserRequest>, ident: Identity) -> HttpResponse{
    let pool = conn.get().expect("Error establishing pool");
    
    let username = path.into_inner();

    let myid = match ident.identity(){
        Some(val) => val,
        None => String::from("None"),
    };

    let iduser = myid.split(":").collect::<Vec<&str>>()[0];
        
    if iduser == "None" || iduser != username{
        return HttpResponse::Unauthorized().content_type("application/json").json("No user login or invalid user")
    }
    
    
    
    let (req_price, prio) = match data.0.priority{
        Some(val) => (data.0.price, val),
        None => (data.0.price, String::from("")),
    };

    //sql query sa group byom se može napravit

    let join_table_data: Result<Vec<JoinedData>, diesel::result::Error> = 
        motherboards.inner_join(socket.inner_join(cpu))
        .inner_join(build_size.inner_join(pc_case))
        .inner_join(ram_slot.inner_join(ram))
        .inner_join(storage_slots.inner_join(storages))
        .select((motherboards::all_columns, cpu::name, storages::name, ram::name, pc_case::name, 
            motherboards::price, cpu::price, storages::price, ram::price, pc_case::price))
        .load::<JoinedData>(&pool);

        
    
    match join_table_data{
        Err(er) => ErrorHandler::from(er).get_response(),
        Ok(vector) => {

            let mut priced_data: Vec<(Motherboard, String, String, String, String, f32)> = Vec::new();

            let cheap_p = get_cheap(&pool).unwrap();
        
            let priostatus = match prio.len(){
                0 => false,
                _ => true,
            };

            

            if !priostatus || prio == "low"{

                for val in vector{
                    let temp_price1 = val.cpu_price + val.case_price + val.mb_price + val.ram_price + val.stor_price;
                    if  temp_price1 + cheap_p < req_price{
                        priced_data.push((val.mb, val.cpu_name.clone(), val.ram_name.clone(), val.stor_name.clone(), val.case_name.clone(), temp_price1));
                    }else{
                        continue;
                    }
                }
            
            }else if prio == "mid"{
                
                for val in vector{
                    let temp_price1 = val.cpu_price + val.case_price + val.mb_price + (val.ram_price * 2.0) + val.stor_price;
                    if  temp_price1 + cheap_p < req_price{
                        priced_data.push((val.mb, val.cpu_name.clone(), val.ram_name.clone(), val.stor_name.clone(), val.case_name.clone(), temp_price1));
                    }else{
                        continue;
                    }
                }

            }else if prio == "high"{

                for val in vector{
                    let temp_price1 = val.cpu_price + val.case_price + val.mb_price + (val.ram_price * 4.0) + val.stor_price;
                    if  temp_price1 + cheap_p < req_price{
                        priced_data.push((val.mb, val.cpu_name.clone(), val.ram_name.clone(), val.stor_name.clone(), val.case_name.clone(), temp_price1));
                    }else{
                        continue;
                    }
                }
            }
        
            

            if priced_data.len() == 0{
                return HttpResponse::Ok().content_type("aplication/json").json("Unable to generate build with proper price")
            }

            priced_data.shuffle(&mut thread_rng());
            
            
            for data in priced_data{
                
                
                let mut cool_vent = cooling.filter(cooling::cpu_ventilator.eq(Some(true)))
                .filter(cooling::price.between(0.0, req_price - data.5))
                .load::<Cooling>(&pool).unwrap();
                cool_vent.shuffle(&mut thread_rng());

                let mut gpu_s = gpu.filter(gpu::price.between(0.0, req_price - data.5))
                .load::<GPU>(&pool).unwrap();
                gpu_s.shuffle(&mut thread_rng());

                let mut pu_s = power_unit.filter(power_unit::price.between(0.0, req_price - data.5))
                .load::<PowerUnit>(&pool).unwrap();
                pu_s.shuffle(&mut thread_rng());

                if cool_vent.len() == 0 || gpu_s.len() == 0 || pu_s.len() == 0{
                    continue;
                }
                
                let mut cool_water = cooling.filter(cooling::ventilators.is_null())
                .filter(cooling::price.between(0.0, req_price - data.5))
                .load::<Cooling>(&pool).unwrap();
                cool_water.shuffle(&mut thread_rng());

                let my_mb = data.0;

                for gp in &gpu_s{
                    for cool in &cool_vent{
                        for pu in &pu_s{
                            
                            let myprice = gp.price + cool.price + pu.price + data.5;

                            if myprice <= req_price{
                                
                                                                
                                let deploydata = vec![my_mb.name.clone(), data.1.clone(), gp.name.clone(), pu.name.clone(), data.4.clone()];

                                if prio == "low"{

                                    let rams = vec![data.2.clone()];
                                    let storagess = vec![data.3.clone()];
                                    let coolings = vec![cool.name.clone()];
                                    return deploy_build(rams, storagess, coolings, deploydata, myprice);

                                }else if prio == "mid"{

                                    let rams = vec![data.2.clone(), data.2.clone()];
                                    let storagess = vec![data.3.clone()];
                                    let coolings = vec![cool.name.clone()];
                                    return deploy_build(rams, storagess, coolings, deploydata, myprice);

                                }else if prio == "high"{

                                    let watercool_name = cool_water[0].name.clone();
                                    let rams = vec![data.2.clone(), data.2.clone(), data.2.clone(), data.2.clone()];
                                    let storagess = vec![data.3.clone(), data.3.clone()];
                                    let coolings = vec![watercool_name];
                                    return deploy_build(rams, storagess, coolings, deploydata, myprice);
                                }
                                
                                                                
                            }
                        }
                    }
                }

            }           
            
            return HttpResponse::Ok().content_type("application/json").json("Wasn't able to generate build")
        },
    }
       
    
}
/* 
pub async fn val_build(path: Path<String>, conn: Data<DBPool>, data: Json<GetBuild>) -> HttpResponse{
    
    let pool = conn.get().expect("Error connecting to pool");

    let join_table_data: Result<Vec<JoinedData>, diesel::result::Error> = 
        motherboards.inner_join(socket.inner_join(cpu))
        .inner_join(build_size.inner_join(pc_case))
        .inner_join(ram_slot.inner_join(ram))
        .inner_join(storage_slots.inner_join(storages))
        .select((motherboards::all_columns, cpu::name, storages::name, ram::name, pc_case::name, 
            motherboards::price, cpu::price, storages::price, ram::price, pc_case::price))
        .load::<(JoinedData)>(&pool);

    
    HttpResponse::Ok().content_type("application/json").json("")
}*/




