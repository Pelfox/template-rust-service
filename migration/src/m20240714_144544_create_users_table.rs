use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    Password,
    Firstname,
    Lastname,
    AccessToken
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Firstname).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Lastname).string().unique_key().not_null())
                    .col(ColumnDef::new(User::AccessToken).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("ids-users")
                    .table(User::Table)
                    .col(User::Id)
                    .to_owned()
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("emails-users")
                    .table(User::Table)
                    .col(User::Email)
                    .to_owned()
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("access_tokens-users")
                    .table(User::Table)
                    .col(User::AccessToken)
                    .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(Index::drop().name("ids-users").to_owned()).await?;
        manager.drop_index(Index::drop().name("emails-users").to_owned()).await?;
        manager.drop_index(Index::drop().name("access_tokens-users").to_owned()).await?;

        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        Ok(())
    }
}
