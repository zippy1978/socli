pub mod handler;

#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,      // Launch to initialize the application
    //Sleep(Duration), // Just take a little break
    // Refresh TODO
    LoadPlayerDetails(String),
    LoadPlayersStats(Vec<String>),
}