use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const DEFAULT_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "usr";
const DATABASE_PORT: u16 = 6767;
const DATABASE_NAME: &str = "test_db";

pub fn establish_connection_to_postgres(
    database_port: u16,
    database_name: &str,
) -> Result<PgConnection, diesel::ConnectionError> {
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{database_port}/{database_name}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = PgConnection::establish(&database_url) {
        eprintln!("Failed to establish connection: {:?}", e);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            eprintln!("Failed to establish connection after 10 attempts");
            std::process::exit(1);
        }
        number_of_attempts += 1;
    }

    PgConnection::establish(&database_url)
}

#[test]
fn connection_and_run_migrations() -> Result<(), diesel::ConnectionError> {
    let mut conn = establish_connection_to_postgres(DATABASE_PORT, DATABASE_NAME)?;
    conn.run_pending_migrations(DEFAULT_MIGRATIONS);
    Ok(())
}
