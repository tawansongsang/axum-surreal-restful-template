use lib_auth::pwd::{self, ContentToHash};
use serde::de::DeserializeOwned;
use surrealdb::sql::{self, Datetime, Thing};
use tracing::debug;
use uuid::Uuid;

use crate::{
    ctx::Ctx,
    model::{
        users::{UsersForAuth, UsersForDelete},
        Error, ModelManager, Result,
    },
};

use super::{
    UsersCreated, UsersDeleted, UsersForCreate, UsersForUpdate, UsersForUpdateByAdmin, UsersRecord,
    UsersUpdated, UsersUpdatedByAdmin,
};

pub struct UsersBmc;

impl UsersBmc {
    pub async fn get<'de, E>(_ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<Option<E>>
    where
        E: DeserializeOwned,
    {
        let db = mm.db();
        let sql = format!("SELECT * FROM ONLY type::thing('users', $id) WHERE deleted_on IS NONE;");
        let mut result = db.query(sql).bind(("id", id)).await?;
        let user = result.take(0)?;

        Ok(user)
    }

    pub async fn list<'de, E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        limit: Option<u32>,
        offset: Option<u32>,
        order: Option<bool>,
    ) -> Result<Vec<E>>
    where
        E: DeserializeOwned,
    {
        let db = mm.db();
        let order = match order {
            Some(true) => "DESC",
            Some(false) => "ASC",
            None => "DESC",
        };

        let sql =
            format!("SELECT * FROM users WHERE deleted_on IS NONE ORDER BY create_on {order} LIMIT $limit START $offset;");
        let mut result = db
            .query(sql)
            .bind(("limit", limit.unwrap_or(50)))
            .bind(("offset", offset.unwrap_or(0)))
            .await?;

        let users: Vec<E> = result.take(0)?;

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
        let sql = "SELECT * FROM users WHERE username = $username AND deleted_on IS NONE LIMIT 1;";
        let mut result = db
            .query(sql)
            .bind(("username", username.to_string()))
            .await?;

        let users_for_auth: Option<E> = result.take(0)?;

        Ok(users_for_auth)
    }

    pub async fn delete(
        _ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
        user_for_delete: UsersForDelete,
    ) -> Result<()> {
        let db = mm.db();

        let user_deleted = UsersDeleted {
            deleted_by: user_for_delete.deleted_by,
            deleted_on: Datetime::default(),
        };

        let _user_record: Result<UsersRecord> = db
            .update(("users", user_id))
            .merge(user_deleted)
            .await?
            .ok_or(Error::DataNotFoundForDelete);

        Ok(())
    }

    pub async fn update<'de, E>(
        ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
        user_for_update: UsersForUpdate,
    ) -> Result<E>
    where
        E: DeserializeOwned,
    {
        let db = mm.db();
        let user_id_update = ctx.user_id_thing().ok_or(Error::CannotGetUserIdFromCtx)?;

        let users_updated = UsersUpdated {
            email: user_for_update.email,
            title: user_for_update.title,
            firstname: user_for_update.firstname,
            middlename: user_for_update.middlename,
            lastname: user_for_update.lastname,
            image: user_for_update.image,
            update_by: user_id_update,
            update_on: Datetime::default(),
        };
        let user_record: Result<E> = db
            .update(("users", user_id))
            .merge(users_updated)
            .await?
            .ok_or(Error::DataNotFoundForUpdate);

        user_record
    }

    pub async fn update_by_admin<'de, E>(
        ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
        user_for_update_by_admin: UsersForUpdateByAdmin,
    ) -> Result<E>
    where
        E: DeserializeOwned,
    {
        let db = mm.db();
        let user_id_update = ctx.user_id_thing().ok_or(Error::CannotGetUserIdFromCtx)?;

        let users_updated_by_admin = UsersUpdatedByAdmin {
            username: user_for_update_by_admin.username,
            email: user_for_update_by_admin.email,
            title: user_for_update_by_admin.title,
            firstname: user_for_update_by_admin.firstname,
            middlename: user_for_update_by_admin.middlename,
            lastname: user_for_update_by_admin.lastname,
            image: user_for_update_by_admin.image,
            role: user_for_update_by_admin.role,
            update_by: user_id_update,
            update_on: Datetime::default(),
        };
        let user_record: Result<E> = db
            .update(("users", user_id))
            .merge(users_updated_by_admin)
            .await?
            .ok_or(Error::DataNotFoundForUpdate);

        user_record
    }

    pub async fn update_pwd(
        ctx: &Ctx,
        mm: &ModelManager,
        id: &str,
        password: String,
        password_salt: Uuid,
    ) -> Result<()> {
        let db = mm.db();
        let user_id = ctx.user_id().ok_or(Error::CannotGetUserIdFromCtx)?;

        // -- Hashing Password
        let to_hash = ContentToHash::new(password, password_salt);
        let password_hash = pwd::hash_pwd(to_hash).await?;

        let sql =
            "UPDATE type::thing('users',$id) SET password = $password_hash, update_by = type::thing('users', $update_by), update_on = time::now();";
        let mut result = db
            .query(sql)
            .bind(("id", id))
            .bind(("password_hash", password_hash))
            .bind(("update_by", user_id))
            .await?;

        let _users_record = result
            .take::<Option<UsersRecord>>(0)?
            .ok_or(Error::DataNotFoundForUpdate)?;

        Ok(())
    }

    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        users_for_create: UsersForCreate,
        is_email_verified: bool,
    ) -> Result<UsersRecord> {
        // Verify Username in DB
        let users =
            UsersBmc::first_by_username::<UsersRecord>(ctx, mm, &users_for_create.username).await?;
        if let Some(_) = users {
            return Err(Error::UsernameAlreadyExists);
        }
        let validate_username = UsersBmc::validate_username(mm, &users_for_create.username).await?;
        if !validate_username {
            return Err(Error::UsernameNotValidFormat);
        }

        let email_verified = match is_email_verified {
            true => Some(Datetime::default()),
            false => None,
        };

        let user_id_create = ctx.user_id_thing();
        let user_record =
            UsersBmc::inner_create(user_id_create, mm, users_for_create, email_verified).await?;

        Ok(user_record)
    }

    pub async fn validate_password(
        password: String,
        password_salt: Uuid,
        hash: String,
    ) -> Result<()> {
        debug!("{} - hash", hash);
        debug!("{} - pass", password);

        let to_hash = ContentToHash::new(password, password_salt);

        let _scheme_status = pwd::validate_pwd(to_hash, hash).await?;

        // -- Update password scheme if needed
        // if let SchemeStatus::Outdated = scheme_status {
        //     debug!("pwd encrypt scheme outdated, upgrading.");
        //     UserBmc::update_pwd(&root_ctx, &mm, user.id, &pwd_clear).await?;
        // }
        Ok(())
    }

    pub async fn is_admin(ctx: &Ctx, mm: &ModelManager) -> Result<bool> {
        let user_id = ctx.user_id().ok_or(Error::UserIdNotFound)?;
        let user_for_auth = UsersBmc::get::<UsersForAuth>(ctx, mm, user_id)
            .await?
            .ok_or(Error::UserIdNotFound)?;
        match user_for_auth.role.as_str() {
            "ADMIN" => Ok(true),
            _ => Ok(false),
        }
    }

    async fn validate_username(mm: &ModelManager, username: &str) -> Result<bool> {
        let db = mm.db();

        let sql = "RETURN string::is::email($username);";

        let mut result = db.query(sql).bind(("username", username)).await?;

        result
            .take::<Option<bool>>(0)?
            .ok_or(Error::CannotValidateUsernameFromDB)
    }

    async fn inner_create(
        user_id_create: Option<Thing>,
        mm: &ModelManager,
        users_for_create: UsersForCreate,
        email_verified: Option<Datetime>,
    ) -> Result<UsersRecord> {
        let db = mm.db();

        let password_salt = sql::Uuid::new_v4();

        // -- Hashing Password
        let to_hash = ContentToHash::new(users_for_create.password, Uuid::from(password_salt));
        let password_hash = pwd::hash_pwd(to_hash).await?;

        let users_created = UsersCreated {
            username: &users_for_create.username,
            email: &users_for_create.username,
            email_verified,
            title: users_for_create.title,
            firstname: users_for_create.firstname,
            middlename: users_for_create.middlename,
            lastname: users_for_create.lastname,
            password: password_hash,
            password_salt,
            create_by: &user_id_create,
            update_by: &user_id_create,
        };

        let mut created: Vec<UsersRecord> = db.create("users").content(users_created).await?;

        let users = created.pop().ok_or(Error::DataNotFound)?;

        Ok(users)
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
