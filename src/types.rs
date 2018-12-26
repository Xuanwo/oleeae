#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Package {
    pub id: i64,

    pub name: String,
    pub pkgver: String,
    pub pkgrel: i32,

    // TODO: Add depend and provide to support static check before makepkg.
    pub status: String,
    pub succeed_times: i32,
    pub failed_times: i32,

    pub builds: Vec<i64>,
    pub maintainers: Vec<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Build {
    pub id: i64,

    pub status: String,
    pub started_at: i64,
    pub finished_at: i64,

    pub package_id: i64,
    pub log_file: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Maintainer {
    pub id: i64,

    pub name: String,
    pub email: String,

    pub packages: Vec<i64>,
}
