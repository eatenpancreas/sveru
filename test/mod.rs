use sqlx::PgPool;

#[sqlx::test]
pub async fn test_err_conversion(db: PgPool) {
  sqlx::query_scalar("SELECT last_login FROM users WHERE username = $1")
    .fetch_one(db);
}