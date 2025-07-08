/// Git仓库信息
#[derive(Debug, Clone)]
pub struct GitInfo {
    pub repo_url: String,
    pub branch: String,
    pub tag: Option<String>,
    pub commit_hash: String,
    pub commit_short: String,
    pub clone_depth: Option<u32>,
}
