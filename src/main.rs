mod domain {
    pub mod model {
        pub mod character;
    }
    pub mod repository {
        pub mod character;
    }
}
mod infrastructure {
    pub mod repository {
        pub mod character;
    }
}
mod usecase {
    pub mod character;
}
mod presentation {
    pub mod http {
        pub mod handler {
            pub mod character;
        }
    }
    pub mod app;
} // Automatically exported by saba.

use sea_orm::Database;
use std::env;
use migration::{Migrator, MigratorTrait};
use presentation::app;
use presentation::http::handler::character::{
    create_character,
    get_characters,
    update_character,
    delete_character,
};
use axum::{
    Router,
    routing::{
        get,
        post,
    }
};


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL という環境変数は設定されていません。");

    let conn = Database::connect(db_url)
        .await
        .expect("データベースの接続に失敗しました。");
    Migrator::up(&conn, None).await.unwrap();

    let state = app::State{ conn };

    let app = Router::new()
        .route("/characters", get(get_characters))
        .route(
            "/character",
            post(create_character)
            .put(update_character)
            .delete(delete_character),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:4000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
