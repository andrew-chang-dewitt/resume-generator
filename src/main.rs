use clap::Parser;
use sqlx::SqlitePool;

use std::env;

mod app;
mod model;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // get env vars
    dotenvy::dotenv().ok();

    // get cli args
    let args = app::Cli::parse();

    // create db connection
    let dburl = &env::var("DBURL")?;
    let pool = SqlitePool::connect(&dburl).await?;
    // make sure db is up to date
    sqlx::migrate!().run(&pool).await?;
    // // and create data model w/ db connection
    // let resume = model::Resume::new(pool);

    // setup stdio write stream
    let mut writer = std::io::stdout();

    // setup app
    let app = app::App::new(pool);
    app.run(args, &mut writer).await
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mockall::predicate::*;
//
//     // FIXME/TODO:
//     // As written, this test is only validating the code from the cli args to the model, but not
//     // that the model saves or returns the correct data to/from the db
//     // might need to go full integration test w/ a test db?
//     // that points to pulling the model setup code out of main--maybe all of db setup code--then
//     // passing a url connection string to a function that returns a db model object
//     // with that, we could use a prod db and a test db
//     #[tokio::test]
//     async fn test_mocked_add() {
//         let desc = String::from("my data");
//         let args = app::Args {
//             cmd: Some(app::Command::Add {
//                 description: desc.clone(),
//             }),
//         };
//
//         let mut mock_resume = model::MockResumeModel::new();
//         mock_resume
//             .expect_add_data()
//             .times(1)
//             .with(eq(desc))
//             .returning(|_| Ok(1));
//
//         let mut mock_writer = Vec::new();
//
//         app::handle_command(args, mock_resume, &mut mock_writer)
//             .await
//             .unwrap();
//
//         assert_eq!(
//             String::from_utf8_lossy(&mock_writer),
//             "Adding data w/ description \'my data\'\nAdded data with id 1\n"
//         );
//     }
// }
