use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHasher};
use log::info;
use sqlx::PgPool;

use crate::domain::account::Account;

pub struct AccountService {
    pub db_pool: PgPool,
}

impl AccountService {
    pub async fn register(&self, username: &str, password: &str) -> Result<Account, sqlx::Error> {
        info!("Registering new account: {}", username);
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| sqlx::Error::Protocol(e.to_string().into()))?
            .to_string();

        let record = sqlx::query!(
            r#"
            INSERT INTO accounts (username, password_hash)
            VALUES ($1, $2)
            RETURNING id, username, password_hash, created_at, updated_at
            "#,
            username,
            password_hash
        )
        .fetch_one(&self.db_pool)
        .await?;

        info!("Account registered: {:?}", record);
        Ok(Account {
            id: record.id,
            username: record.username,
            password_hash: record.password_hash,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<Account>, sqlx::Error> {
        info!("Finding account by username: {}", username);
        let record = sqlx::query!(
            r#"
            SELECT id, username, password_hash, created_at, updated_at
            FROM accounts
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.db_pool)
        .await?;

        info!("Account found: {:?}", record);
        Ok(record.map(|r| Account {
            id: r.id,
            username: r.username,
            password_hash: r.password_hash,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    pub async fn update_password_by_username(
        &self,
        username: &str,
        new_password: &str,
    ) -> Result<(), sqlx::Error> {
        info!("Updating password for username: {}", username);
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let new_hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| sqlx::Error::Protocol(e.to_string().into()))?
            .to_string();

        sqlx::query!(
            r#"
            UPDATE accounts
            SET password_hash = $1, updated_at = NOW()
            WHERE username = $2
            "#,
            new_hash,
            username
        )
        .execute(&self.db_pool)
        .await?;

        info!("Password updated for username: {}", username);
        Ok(())
    }

    pub async fn update_password(
        &self,
        account_id: i32,
        new_password: &str,
    ) -> Result<(), sqlx::Error> {
        info!("Updating password for account ID: {}", account_id);
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let new_hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| sqlx::Error::Protocol(e.to_string().into()))?
            .to_string();

        sqlx::query!(
            r#"
            UPDATE accounts
            SET password_hash = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            new_hash,
            account_id
        )
        .execute(&self.db_pool)
        .await?;

        info!("Password updated for account ID: {}", account_id);
        Ok(())
    }

    pub async fn update_account_info(
        &self,
        account_id: i32,
        new_username: Option<&str>,
    ) -> Result<Account, sqlx::Error> {
        info!("Updating account info for account ID: {}", account_id);
        let account = if let Some(username) = new_username {
            sqlx::query_as!(
                Account,
                r#"
                UPDATE accounts
                SET username = $1,
                    updated_at = NOW()
                WHERE id = $2
                RETURNING id, username, password_hash, created_at, updated_at
                "#,
                username,
                account_id,
            )
            .fetch_one(&self.db_pool)
            .await?
        } else {
            sqlx::query_as!(
                Account,
                r#"
                SELECT id, username, password_hash, created_at, updated_at
                FROM accounts
                WHERE id = $1
                "#,
                account_id
            )
            .fetch_one(&self.db_pool)
            .await?
        };

        info!("Account info updated: {:?}", account);
        Ok(account)
    }

    pub async fn get_accounts(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<Account>, i64), sqlx::Error> {
        info!(
            "Fetching accounts with offset: {}, limit: {}",
            offset, limit
        );
        let accounts = sqlx::query_as!(
            Account,
            r#"
            SELECT id, username, password_hash, created_at, updated_at
            FROM accounts
            ORDER BY id
            OFFSET $1 LIMIT $2
            "#,
            offset,
            limit
        )
        .fetch_all(&self.db_pool)
        .await?;

        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!: i64" FROM accounts
            "#,
        )
        .fetch_one(&self.db_pool)
        .await?;

        info!("Accounts fetched: {:?}, total: {}", accounts, total);
        Ok((accounts, total))
    }
}
