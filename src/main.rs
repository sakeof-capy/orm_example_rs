mod capybara;
mod habitat;

use capybara::{ActiveModel as CapybaraModel, Entity as Capybara};
use habitat::{ActiveModel as HabitatModel, Entity as Habitat};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, EntityTrait};
use std::default::Default;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connection = Database::connect(env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let habitats = [HabitatModel {
        habitat_name: Set("Habitat1".to_owned()),
        ..Default::default()
    }];

    let insertion_futures = habitats
        .into_iter()
        .map(|habitat| habitat.insert(&connection))
        .collect::<Vec<_>>();

    let inserted_results = futures::future::join_all(insertion_futures).await;

    for habitat_insertion_result in inserted_results {
        match habitat_insertion_result {
            Ok(inserted_habitat) => {
                let capy1 = CapybaraModel {
                    capybara_name: Set(format!("Capybara{}_1", inserted_habitat.id)),
                    habitat_id: Set(Some(inserted_habitat.id)),
                    ..Default::default()
                };
                let capy2 = CapybaraModel {
                    capybara_name: Set(format!("Capybara{}_2", inserted_habitat.id)),
                    habitat_id: Set(Some(inserted_habitat.id)),
                    ..Default::default()
                };

                let capy1_insertion_result = capy1.insert(&connection).await;
                let capy2_insertion_result = capy2.insert(&connection).await;

                if let Err(error) = capy1_insertion_result {
                    eprintln!("Failed to insert capy1: {}", error);
                }

                if let Err(error) = capy2_insertion_result {
                    eprintln!("Failed to insert capy2: {}", error);
                }
            }
            Err(db_error) => {
                eprintln!("Unable to insert habitat: {}", db_error);
            }
        }
    }

    let all_habitats = Habitat::find().all(&connection).await.unwrap();
    println!("all habitats: {:?}", all_habitats);
    let all_capybaras = Capybara::find().all(&connection).await.unwrap();
    println!("all capybaras: {:?}", all_capybaras);
}
