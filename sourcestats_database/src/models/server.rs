use diesel::prelude::*;
use diesel::result::Error;
use crate::schema::servers::dsl::*;
use crate::pool::Pool;

use std::ops::Deref;

#[derive(Queryable)]
pub struct Server {
    pub server_id: i32,
    pub server_name: String,
    pub server_address: String,
    pub server_website: Option<String>,
}

impl Server {
    pub fn get_by_id(id: i32, pool: &Pool) -> Result<Server, Error> {
        servers.filter(server_id.eq(id)).get_result::<Server>(pool.get_connection().deref())
    }
}