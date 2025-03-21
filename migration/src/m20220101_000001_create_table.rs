use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Capybara table
        manager
            .create_table(
                Table::create()
                    .table(Capybaras::Table)
                    .if_not_exists()
                    .col(pk_auto(Capybaras::Id))
                    .col(string(Capybaras::CapybaraName))
                    .col(integer(Capybaras::HabitatId))
                    .to_owned(),
            )
            .await?;

        // Create Habitat table
        manager
            .create_table(
                Table::create()
                    .table(Habitats::Table)
                    .if_not_exists()
                    .col(pk_auto(Habitats::Id))
                    .col(string(Habitats::HabitatName))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop Capybara table
        manager
            .drop_table(Table::drop().table(Capybaras::Table).to_owned())
            .await?;

        // Drop Habitat table
        manager
            .drop_table(Table::drop().table(Habitats::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Capybaras {
    Table,
    Id,
    CapybaraName,
    HabitatId,
}

#[derive(DeriveIden)]
enum Habitats {
    Table,
    Id,
    HabitatName,
}
