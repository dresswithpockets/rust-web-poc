use diesel::{Connection, ConnectionResult, PgConnection};

pub fn establish_connection(url: &str) -> ConnectionResult<PgConnection> {
    PgConnection::establish(url)
}