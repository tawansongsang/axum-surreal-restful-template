use serde::de::DeserializeOwned;
use surrealdb::sql;

use crate::{
    ctx::Ctx,
    model::{Error, ModelManager, Result},
};

use super::{Users, UsersCreated, UsersForCreate, UsersRecord};

pub struct UsersBmc;

impl UsersBmc {
    pub async fn get<'de, E>(_ctx: &Ctx, mm: &ModelManager, id: sql::Uuid) -> Result<Option<E>>
    where
        E: DeserializeOwned,
    {
        let db = mm.db();
        let sql = "SELECT * FROM users:$id LIMIT 1;";
        let mut result = db.query(sql).bind(("id", id.to_string())).await?;

        let users: Option<E> = result.take(0)?;

        Ok(users)
    }

    pub async fn first_by_username<'de, E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: DeserializeOwned,
    {
        let db = mm.db();
        let sql = "SELECT * FROM users WHERE username = $username LIMIT 1;";
        let mut result = db
            .query(sql)
            .bind(("username", username.to_string()))
            .await?;

        let users_for_auth: Option<E> = result.take(0)?;

        Ok(users_for_auth)
    }

    pub async fn first_by_id<'de, E>(_ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<Option<E>>
    where
        E: DeserializeOwned,
    {
        let db = mm.db();
        let users_for_auth = db.select(("users", id)).await?;

        Ok(users_for_auth)
    }

    pub async fn update_pwd(
        ctx: &Ctx,
        mm: &ModelManager,
        id: sql::Uuid,
        password: &str,
    ) -> Result<()> {
        let db = mm.db();
        let sql =
            "UPDATE ONLY users:&id SET password = &password update_by = users:&update_by update_on = time::now();";
        let mut result = db
            .query(sql)
            .bind(("id", id))
            .bind(("password", password))
            .bind(("update_by", ctx.user_id()))
            .await?;

        let _users: Option<Users> = result.take(0)?;

        Ok(())
    }

    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        users_for_create: UsersForCreate,
    ) -> Result<UsersRecord> {
        // Verify Username in DB
        let users =
            UsersBmc::first_by_username::<UsersRecord>(&ctx, mm, &users_for_create.username)
                .await?;
        if let Some(_) = users {
            return Err(Error::UsernameAlreadyExists);
        }
        let validate_username = UsersBmc::validate_username(mm, &users_for_create.username).await?;
        if !validate_username {
            return Err(Error::UsernameNotValidFormat);
        }

        let db = mm.db();

        let user_id_create = ctx.user_id_thing();

        let users_created = UsersCreated {
            username: &users_for_create.username,
            email: &users_for_create.username,
            title: users_for_create.title,
            first_name: users_for_create.first_name,
            middle_name: users_for_create.middle_name,
            last_name: users_for_create.last_name,
            password: users_for_create.password,
            create_by: &user_id_create,
            update_by: &user_id_create,
        };

        let mut created: Vec<UsersRecord> = db.create("users").content(users_created).await?;

        let users = created.pop().ok_or(Error::DataNotFound)?;

        Ok(users)
    }

    pub async fn validate_password(mm: &ModelManager, hash: &str, password: &str) -> Result<bool> {
        let db = mm.db();

        let sql = "RETURN crypto::argon2::compare($hash, $pass)";

        let mut result = db
            .query(sql)
            .bind(("hash", hash))
            .bind(("pass", password))
            .await?;

        result
            .take::<Option<bool>>(0)?
            .ok_or(Error::CannotComparePasswordFromDB)
    }

    pub async fn validate_username(mm: &ModelManager, username: &str) -> Result<bool> {
        let db = mm.db();

        let sql = "RETURN string::is::email($username);";

        let mut result = db.query(sql).bind(("username", username)).await?;

        result
            .take::<Option<bool>>(0)?
            .ok_or(Error::CannotValidateUsernameFromDB)
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>; // For tests.
    use crate::model::{self, users::UsersForAuth};

    use super::*;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        // -- Setup & Fixtures
        let mm = model::ModelManager::new().await?;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        // -- Exec
        let user = UsersBmc::first_by_username::<UsersForAuth>(&ctx, &mm, fx_username)
            .await?
            .ok_or("Should have user 'demo1'")?;

        // -- Check
        assert_eq!(user.username, fx_username);

        Ok(())
    }
}
// endregion: --- Tests
