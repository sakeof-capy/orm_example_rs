use sea_orm::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "habitats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_type = "Text", unique, nullable)]
    pub habitat_name: String,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::capybara::Entity")]
    Capybara,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::capybara::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Capybara.def() // Return the `has_many` relation definition
    }
}
