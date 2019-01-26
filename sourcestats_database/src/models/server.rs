use diesel::prelude::*;
use diesel::result::Error;
use crate::schema::servers;
use crate::schema::server_keys;

#[derive(Queryable)]
pub struct Server {
    pub server_id: i64,
    pub server_name: String,
    pub server_address: String,
    pub server_website: Option<String>,
}

#[derive(Queryable)]
pub struct ServerKey {
    pub key_id: i64,
    pub key_data: Vec<u8>,
    pub server_id: i64
}

impl Server {
    pub fn get_by_id(id: i64, conn: &PgConnection) -> Result<Server, Error> {
        servers::table.filter(servers::server_id.eq(id)).get_result(conn)
    }

    pub fn get_server_key(&self, conn: &PgConnection) -> Result<ServerKey, Error> {
        server_keys::table.filter(server_keys::server_id.eq(self.server_id)).get_result(conn)
    }
}

impl ServerKey {
    pub fn get_by_id(id: i64, conn: &PgConnection) -> Result<ServerKey, Error> {
        server_keys::table.filter(server_keys::key_id.eq(id)).get_result(conn)
    }
}