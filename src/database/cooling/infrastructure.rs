extern crate diesel;

use crate::database::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;

use super::contract::*;
use super::data::*;
use diesel::QueryDsl;
use schema::cooling;
use schema::cooling::dsl::*;

impl CRUD for Cooling {
    fn create(&self, conn: &DBPooledConnection) -> Result<Cooling, diesel::result::Error> {
        let cool = NewCooling {
            name: self.name.clone(),
            manufacturer: self.manufacturer.clone(),
            ventilators: self.ventilators,
            cpu_ventilator: self.cpu_ventilator,
            price: self.price,
        };

        diesel::insert_into(cooling::table)
            .values(&cool)
            .get_result(conn)
    }

    fn read_all(&self, conn: &DBPooledConnection) -> Result<Vec<Cooling>, diesel::result::Error> {
        cooling.load::<Cooling>(conn)
    }

    fn update(
        &self,
        conn: &DBPooledConnection,
        other: Cooling,
    ) -> Result<Cooling, diesel::result::Error> {
        diesel::update(cooling.filter(name.eq(&self.name)))
            .set((
                name.eq(other.name),
                manufacturer.eq(other.manufacturer),
                ventilators.eq(other.ventilators),
                cpu_ventilator.eq(other.cpu_ventilator),
                price.eq(other.price),
            ))
            .get_result(conn)
    }

    fn delete(&self, conn: &DBPooledConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(cooling.filter(name.eq(&self.name))).execute(conn)
    }
}
