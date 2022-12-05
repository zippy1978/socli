use std::path::Path;

use crate::core::service::strategy::StrategyServiceImpl;

#[tokio::test]
async fn parse_player_slug() {

    let path = Path::new("dir/test.js");

    let strategy_service = StrategyServiceImpl::new("fake");
    assert_eq!(strategy_service.extract_strategy_name(&path), "test");
}