use std::env;

use rustyroad::database::*;
use rustyroad::generators::create_directory;
use rustyroad::writers::create_files;
use rustyroad::writers::new;
use rustyroad::Project;

/// Creates a new project
/// Takes an optional name <String> and db_type <String>
/// If no name is provided, it will default to "rustyroad"
/// If a name is provided, it will create a new directory with that name
/// and create a new project in that directory
/// If a directory with the same name already exists, it will return an error
/// and ask the user to choose a different name
/// If a db_type is provided, it will create a new database with that type
/// If no db_type is provided, it will default to "sqlite"
/// If a db_type is provided that is not supported, it will return an error
/// and ask the user to choose a different db_type
/// Allow unused variables because the db_type is not used yet
#[allow(unused_variables)]
pub async fn create_new_project(name: String, database_data: Database) -> Result<Project, Error> {
    // If name is provided, create a new directory with that name
    // If no name is provided, run the rest of the code in the function
    // write the database data to the rustyroad.toml file

    // Create new project with name
    let mut project = new(name);

    // Create the project directory
    create_directory(&project).unwrap_or_else(|why| {
        println!("Couldn't create directory: {:?}", why.kind());
    });

    // Create the files
    create_files(&project).unwrap_or_else(|why| {
        panic!("Couldn't create files: {:?}", why.kind());
    });

    // Write to rustyroad.toml file
    Project::write_to_rustyroad_toml(&project, &database_data)
        .expect("Failed to write to rustyroad.toml");

    // Write to the cargo.toml file
    rustyroad::writers::write_to_cargo_toml(&project, &database_data)
        .expect("Failed to write to cargo.toml");

    // Write to main.rs file
    rustyroad::writers::write_to_main_rs(&project).expect("Failed to write to main.rs");

    // Write to package.json file
    Project::write_to_package_json(&project).expect("Failed to write to package.json");

    // Write to README.md file
    Project::write_to_readme(&project).expect("Failed to write to README.md");

    // Write to index.js file
    Project::write_to_index_js(&project).unwrap_or_else(|why| {
        println!("Failed to write to index.js: {:?}", why.kind());
    });
    // Write to index.html.tera file
    rustyroad::writers::write_to_index_html(&project).unwrap_or_else(|why| {
        println!("Failed to write to index.html: {:?}", why.kind());
    });
    // Write to base.html.tera file
    rustyroad::writers::write_to_base_html(&project.base_html).unwrap_or_else(|why| {
        println!("Failed to write to base.html: {:?}", why.kind());
    });

    // Write to tailwind.css file
    Project::write_to_tailwind_css(&project).unwrap_or_else(|why| {
        println!("Failed to write to tailwind.css: {:?}", why.kind());
    });
    // need to create the function
    // Write to tailwind.config.js file
    Project::write_to_tailwind_config(&project).unwrap_or_else(|why| {
        println!("Failed to write to tailwind.config.js: {:?}", why.kind());
    });

    // Write to postcss.config.js file
    Project::write_to_postcss_config(&project).unwrap_or_else(|why| {
        println!("Failed to write to postcss.config.js: {:?}", why.kind());
    });

    // Write to index.html route
    rustyroad::writers::write_to_index_route(&project).unwrap_or_else(|why| {
        println!("Failed to write to index.html: {:?}", why.kind());
    });

    // Write to gitignore file
    Project::write_to_gitignore(&project).unwrap_or_else(|why| {
        println!("Failed to write to .gitignore: {:?}", why.kind());
    });

    rustyroad::writers::write_to_routes_mod(&project.routes_module, "index".to_string()).unwrap_or_else(|why| {
        println!("Failed to write to routes/mod: {:?}", why.kind());
    });
    // Write to Header
    rustyroad::writers::write_to_header(&project.header_section).unwrap_or_else(|why| {
        println!("Failed to write to header: {:?}", why.kind());
    });

    // write to navbar
    rustyroad::writers::write_to_navbar(&project).unwrap_or_else(|why| {
        println!("Failed to write to navbar: {:?}", why.kind());
    });

    // write to the dashboard page
    rustyroad::writers::write_to_dashboard(project.clone()).unwrap_or_else(|why| {
        println!("Failed to write to dashboard: {:?}", why.kind());
    });

    // write to the login page
    rustyroad::writers::write_to_login_page(project.clone()).unwrap_or_else(|why| {
        println!("Failed to write to login: {:?}", why.kind());
    });

    // We need to tell Diesel where to find our database. We do this by setting the DATABASE_URL environment variable.
    // We can do this by running the following command in the terminal:
    let temp_database = &database_data.clone();
    // Embed migrations from the "migrations" directory
    // Use the embed_migrations macro to embed migrations into the binary
    // Adjust the path to point to the location of your migration files

    match temp_database.database_type {
        DatabaseType::Sqlite => {
            // Create the database URL
            let database_url = project.config_dev_db.to_string();
            println!("database_url: {database_url}");

            // In SQLite, creating a connection to a non-existent database
            // automatically creates the database file, so we don't need to
            // explicitly create the database.

            // Generate the SQL content for the new project
            let sql_content = rustyroad::writers::load_sql_for_new_project(&project, database_data.clone()).await?;

            // Establish a connection to the new database
            let connection_result = SqliteConnectOptions::new()
                .filename(&database_url)
                .connect()
                .await;

            // Check if the connection was successful
            let mut connection = match connection_result {
                Ok(conn) => conn,
                Err(why) => {
                    panic!("Failed to establish connection: {why}");
                }
            };

            // Iterate through the vector of SQL commands and execute them one at a time
            for sql_command in sql_content {
                // Execute the SQL command
                sqlx::query(&sql_command)
                    .execute(&mut connection)
                    .await
                    .unwrap_or_else(|why| panic!("Failed to execute SQL command: {why}"));
            }

            rustyroad::writers::write_to_sqlite_user_models(&project).unwrap_or_else(|why| {
                println!("Failed to write to user models: {:?}", why.kind());
            });
        }

        DatabaseType::Postgres => {
            // Replace this line with the correct URL for the default "postgres" database
            let admin_database_url = format!(
                "postgres://{}:{}@{}:{}/postgres",
                database_data.username,
                database_data.password,
                database_data.host,
                database_data.port,
            );

            // Call the function with the admin_database_url
            rustyroad::writers::create_database_if_not_exists(&admin_database_url, database_data.clone())
                .await
                .unwrap_or_else(|why| {
                    panic!("Failed to create database: {why}");
                });

            // Create the database URL
            let database_url = format!(
                "postgres://{}:{}@{}:{}/{}",
                database_data.username,
                database_data.password,
                database_data.host,
                database_data.port,
                database_data.name
            );

            // Update the DATABASE_URL environment variable to point to the new 'test' database
            env::set_var(
                "DATABASE_URL",
                database_url.replace(&database_data.name, "test"),
            );

            project.config_dev_db = database_url.clone();

            println!("database_url: {database_url}");

            // Generate the SQL content for the new project
            let sql_content =
                rustyroad::writers::initial_sql_loader::load_sql_for_new_project(&project, database_data.clone())
                    .await?;

            // Establish a connection to the new database
            let connection_result = PgConnectOptions::new()
                .username(&database_data.username)
                .password(&database_data.password)
                .host(&database_data.host)
                .port(database_data.port.parse::<u16>().unwrap())
                .database(&database_data.name)
                .connect()
                .await;

            // Check if the connection was successful
            let mut connection = match connection_result {
                Ok(conn) => conn,
                Err(why) => {
                    panic!("Failed to establish connection: {why}");
                }
            };

            // Iterate through the vector of SQL commands and execute them one at a time
            for sql_command in sql_content {
                // Execute the SQL command
                sqlx::query(&sql_command)
                    .execute(&mut connection)
                    .await
                    .unwrap_or_else(|why| panic!("Failed to execute SQL command: {why}"));
            }

            /* Write to user models file */
            write_to_postgres_user_models(&project).unwrap_or_else(|why| {
                println!("Failed to write to user models: {why}");
            });
        }

        DatabaseType::Mysql => {
            // Create the database URL for the default "mysql" database
            let admin_database_url = format!(
                "mysql://{}:{}@{}:{}/mysql",
                database_data.username,
                database_data.password,
                database_data.host,
                database_data.port,
            );

            // Call the function with the admin_database_url
            create_database_if_not_exists(&admin_database_url, database_data.clone())
                .await
                .unwrap_or_else(|why| {
                    panic!("Failed to create database: {:?}", why);
                });

            // Create the database URL for the new database
            let database_url = format!(
                "mysql://{}:{}@{}:{}/{}",
                database_data.username,
                database_data.password,
                database_data.host,
                database_data.port,
                database_data.name
            );

            // Update the DATABASE_URL environment variable to point to the new 'test' database
            env::set_var(
                "DATABASE_URL",
                database_url.replace(&database_data.name, "test"),
            );

            project.config_dev_db = database_url.clone();

            println!("database_url: {database_url}");

            // Generate the SQL content for the new project
            let sql_content =
                initial_sql_loader::load_sql_for_new_project(&project, database_data.clone())
                    .await?;

            // Establish a connection to the new database
            let connection_result = MySqlConnectOptions::new()
                .username(&database_data.username)
                .password(&database_data.password)
                .host(&database_data.host)
                .port(database_data.port.parse::<u16>().unwrap())
                .database(&database_data.name)
                .connect()
                .await;

            // Check if the connection was successful
            let mut connection = match connection_result {
                Ok(conn) => conn,
                Err(why) => {
                    panic!("Failed to establish connection: {why}");
                }
            };

            // Iterate through the vector of SQL commands and execute them one at a time
            for sql_command in sql_content {
                println!("Executing SQL command: {sql_command}"); // Log the SQL command being executed
                                                                  // Execute the SQL command
                match sqlx::query(&sql_command).execute(&mut connection).await {
                    Ok(_) => {
                        println!("Successfully executed SQL command: {sql_command}");
                    }
                    Err(why) => {
                        println!("Failed to execute SQL command: {sql_command}, Error: {why}");
                        // Optionally, return an error instead of panicking
                        // return Err(why.into());
                    }
                }
            }

            write_to_mysql_user_models(&project).unwrap_or_else(|why| {
                println!("Failed to write to user models: {:?}", why.kind());
            });
        }

        DatabaseType::Mongo => {
            // Create the database
            let database_url = format!(
                "DATABASE_URL=mongodb://localhost:27017/{}",
                &database_data.clone().name
            );
            println!("database_url: {database_url}");
            let output = std::process::Command::new("diesel")
                .arg("setup")
                .env("DATABASE_URL", database_url)
                .output()
                .expect("Failed to execute process");
            println!("output: {:?}", output);
        }
    }

    println!("Project {} created!", &project.name);

    // Create the database
    Ok(project)
} // End of create_new_project function
