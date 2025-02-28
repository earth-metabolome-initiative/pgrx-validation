use diesel::{connection::SimpleConnection, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::path::Path;
use std::process::Command;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};

const DEFAULT_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "usr";
const DATABASE_PORT: u16 = 6767;
const DATABASE_NAME: &str = "test_db";

/// Establish a connection to a postgres database.
pub fn establish_connection_to_postgres() -> Result<PgConnection, diesel::ConnectionError> {
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{DATABASE_PORT}/{DATABASE_NAME}",
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

/// Setup a docker container with a postgres database.
///
/// # Panics
///
/// * If the container cannot be started.
///
pub async fn setup_docker() -> ContainerAsync<GenericImage> {
    let container = GenericImage::new("postgres", "17")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", DATABASE_NAME)
        .with_mapped_port(DATABASE_PORT, 5432_u16.tcp())
        .with_copy_to(
            "/usr/share/postgresql/17/extension/pgrx_validation.control",
            Path::new(
                "./my_own_extension/usr/share/postgresql/17/extension/pgrx_validation.control",
            ),
        )
        .with_copy_to(
            "/usr/share/postgresql/17/extension/pgrx_validation--0.0.0.sql",
            Path::new(
                "./my_own_extension/usr/share/postgresql/17/extension/pgrx_validation--0.0.0.sql",
            ),
        )
        .with_copy_to(
            "/usr/lib/postgresql/17/lib/pgrx_validation.so",
            Path::new("./my_own_extension/usr/lib/postgresql/17/lib/pgrx_validation.so"),
        )
        .start()
        .await
        .expect("Failed to start container");

    container
}

#[tokio::test]
async fn connection_and_run_migrations() {
    let container = setup_docker().await;
    let mut conn = establish_connection_to_postgres().expect("Failed to establish connection");
    conn.run_pending_migrations(DEFAULT_MIGRATIONS)
        .expect("Failed to run migrations");
    conn.batch_execute("INSERT INTO price (value) VALUES (1);")
        .expect("Failed to insert value : 1.");
    conn.batch_execute("INSERT INTO price (value) VALUES (-1);")
        .expect_err("Insertion of value should have failed.");

    conn.batch_execute("INSERT INTO position (x,y) VALUES (3,2)")
        .expect("Failed to insert value");
    conn.batch_execute("INSERT INTO position (x,y) VALUES (2,3)")
        .expect_err("Insertion should have failed");

    container.stop().await.expect("Failed to stop container");
}
