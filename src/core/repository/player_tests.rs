use crate::core::repository::player::PlayerRepoImpl;

#[tokio::test]
async fn parse_player_slug() {

    let player_repo = PlayerRepoImpl::new();
    assert_eq!(player_repo.parse_player_slug("kz-okpala-19990428-2022-rare-21").unwrap(), "kz-okpala-19990428");
    assert_eq!(player_repo.parse_player_slug("kevin-knox-ii-19990811-2022-limited-124").unwrap(), "kevin-knox-ii-19990811");
}