use std::{ops::Deref, sync::Arc};

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;
use diesel::PgConnection;

use crate::footballer::Footballer;
use crate::footballer::NewFootballer;

pub struct FootballerRepository {
    m_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl FootballerRepository {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> FootballerRepository {
        FootballerRepository { m_pool: pool }
    }

    /// Save a footballer in the database
    pub fn create(&self, v_footballer: NewFootballer) -> Result<Footballer, Error> {
        use crate::schema::footballer::dsl::*;

        match diesel::insert_into(footballer)
            .values(&v_footballer)
            .get_result::<Footballer>(self.m_pool.deref().get().unwrap().deref())
        {
            Ok(saved_footballer) => Ok(saved_footballer),
            Err(e) => Err(e.into()),
        }
    }

    /// Find a footballer by its position
    pub fn find_by_position(&self, footballer_position: &str) -> Result<Vec<Footballer>, Error> {
        use crate::schema::footballer::dsl::*;

        let footballers = footballer
            .filter(position.eq(footballer_position))
            .load::<Footballer>(self.m_pool.deref().get().unwrap().deref())?;

        Ok(footballers)
    }

    /// Find a footballer by its id
    pub fn find_by_id(&self, footballer_id: i64) -> Result<Footballer, Error> {
        use crate::schema::footballer::dsl::*;

        Ok(footballer
            .filter(id.eq(footballer_id))
            .first::<Footballer>(self.m_pool.deref().get().unwrap().deref())?)
    }

    /// Find all footballers
    pub fn find_all(&self) -> Result<Vec<Footballer>, Error> {
        use crate::schema::footballer::dsl::*;

        Ok(footballer.load::<Footballer>(self.m_pool.deref().get().unwrap().deref())?)
    }

    /// Delete a footballer by its id
    pub fn delete_by_id(&self, footballer_id: i64) -> Result<bool, Error> {
        use crate::schema::footballer::dsl::*;

        let count = diesel::delete(footballer.filter(id.eq(footballer_id)))
            .execute(self.m_pool.deref().get().unwrap().deref())?;

        Ok(count > 0)
    }
}
