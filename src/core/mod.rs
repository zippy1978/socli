pub mod model;
pub mod repository;
pub mod service;

use dilib::{
    add_singleton_trait,
    global::{init_container, InitContainerError},
};
use quartermaster::{store::memory::InMemoryTaskStore, manager::TaskManager};

use crate::core::{
    repository::{player::{PlayerRepo, PlayerRepoImpl}, storage::{StorageRepo, StorageRepoImpl}, price::{PriceRepo, PriceRepoImpl}, stats::{StatsRepo, StatsRepoImpl}},
    service::{player::{PlayerService, PlayerServiceImpl}, price::{PriceService, PriceServiceImpl}, stats::{StatsService, StatsServiceImpl}, strategy::{StrategyService, StrategyServiceImpl}},
};

#[macro_export]
macro_rules! resolve_trait {
    ($trait_type:ident $(<$($generic:ident),+>)?) => {{
        let container = dilib::global::get_container().unwrap();
        let ret = container.get::<std::boxed::Box<(dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static)>>();
        ret.unwrap()
    }};

    ($trait_type:ident $(<$($generic:ident),+>)?, $name:literal) => {{
        let container = dilib::global::get_container().unwrap();
        let ret = container.get_with_name::<std::boxed::Box<(dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static)>>($name);
        ret.unwrap()
    }};

}

#[macro_export]
macro_rules! resolve {
    ($type:ident $(<$($generic:ident),+>)?) => {{
        let container = dilib::global::get_container().unwrap();
        let ret = container.get::<$type>();
        ret.unwrap()
    }};

    ($type:ident $(<$($generic:ident),+>)?, $name:literal) => {{
        let container = dilib::global::get_container().unwrap();
        let ret = container.get_with_name::<$type>($name);
        ret.unwrap()
    }};
}

pub type MainTaskManager = TaskManager<InMemoryTaskStore>;

/// Setup depedency injection.
/// Provided by https://github.com/Neo-Ciber94/dilib-rs#bind-trait-to-implementation.
pub async fn setup_container<'a>(strategies_dir: &str) -> Result<(), InitContainerError> {
    let init_result = init_container(|container| {

        // Task manager
        let task_manager = TaskManager::new(
            InMemoryTaskStore::new("task manager"),
            2,
        );
        container.add_singleton(task_manager).unwrap();

        // Repositories
        add_singleton_trait!(container, PlayerRepo => PlayerRepoImpl::new()).unwrap();
        add_singleton_trait!(container, PriceRepo => PriceRepoImpl::new()).unwrap();
        add_singleton_trait!(container, StorageRepo => StorageRepoImpl::new()).unwrap();
        add_singleton_trait!(container, StatsRepo => StatsRepoImpl::new()).unwrap();

        // Services
        add_singleton_trait!(container, PlayerService => PlayerServiceImpl{}).unwrap();
        add_singleton_trait!(container, PriceService => PriceServiceImpl{}).unwrap();
        add_singleton_trait!(container, StatsService => StatsServiceImpl{}).unwrap();
        add_singleton_trait!(container, StrategyService => StrategyServiceImpl::new(strategies_dir)).unwrap();
    });

    // Start task manager
    if init_result.is_ok() {
        resolve!(MainTaskManager)
            .start()
            .await;
    }

    // Result
    init_result
}
