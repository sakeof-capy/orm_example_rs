use sea_orm::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "capybaras")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,

    capybara_name: String,

    #[sea_orm(column_type = "Integer", nullable)]
    pub habitat_id: Option<i32>,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::habitat::Entity",
        from = "Column::HabitatId",
        to = "super::habitat::Column::Id"
    )]
    Habitat,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::habitat::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Habitat.def()
    }
}
