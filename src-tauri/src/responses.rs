use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GetAppList_Response {
    pub applist: Apps_Array
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub(crate) struct Apps_Array {
    pub apps: Vec<App>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct App {
    pub appid: u32,
    pub name: String
}