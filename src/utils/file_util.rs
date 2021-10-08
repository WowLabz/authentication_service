use rocket::config::Config;

pub const STORAGE_DIRECTORY: &str = "./uploads";

pub struct FileUtil;

impl FileUtil {
    pub async fn get_basefile_path() -> String {
        let config = Config::figment();
        let address = config.extract_inner::<String>("address").unwrap();
        let port = config.extract_inner::<i64>("port").unwrap();
        format!("http://{}:{}/files", address, port)
    }
}
