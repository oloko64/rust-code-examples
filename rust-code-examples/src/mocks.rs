use async_trait::async_trait;
use mockall::predicate::*;

#[cfg(test)]
use mockall::*;

// https://docs.rs/mockall/latest/mockall/macro.mock.html
// https://docs.rs/mockall_double/latest/mockall_double/attr.double.html
// https://docs.rs/mockito/latest/mockito/index.html

#[tokio::main]
async fn main() {
    let url = "https://docs.rs/mockall/latest/mockall/macro.mock.html";
    let val = get_user(&DatabaseTypeOne, "Hello world!");
    assert_eq!(val, "Hello world! Hello world!");

    let val = get_page_text(&Downloader, url).await.unwrap();
    assert!(val.contains("Macro"));
}

#[cfg_attr(test, automock)]
trait Database {
    fn return_user(&self, str: &str) -> String;
}

fn get_user<T: Database>(t: &T, str: &str) -> String {
    t.return_user(str)
}

struct DatabaseTypeOne;

impl Database for DatabaseTypeOne {
    fn return_user(&self, str: &str) -> String {
        format!("{str} Hello world!")
    }
}

// ------------------------------------------------------------------

#[cfg_attr(test, automock)]
#[async_trait]
trait DownloaderTrait {
    async fn page_text(&self, url: &str) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone)]
struct Downloader;

#[async_trait]
impl DownloaderTrait for Downloader {
    async fn page_text(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;
        if !response.status().is_success() {
            return Err(format!("Error downloading page {}", response.status()).into());
        }

        let body = response.text().await?;
        Ok(body)
    }
}

async fn get_page_text<T: DownloaderTrait>(
    t: &T,
    url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    t.page_text(url).await
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_user() {
        let mut database = MockDatabase::new();
        database
            .expect_return_user()
            .with(eq("Hello world!"))
            .times(1)
            .returning(|_| String::from("MOCKED"));

        let val = get_user(&database, "Hello world!");
        assert_eq!(val, "MOCKED");
    }

    #[tokio::test]
    async fn test_get_page_text_mocked() {
        let url = "https://crates.io/crates/mockall";
        let mut downloader = MockDownloaderTrait::new();
        downloader
            .expect_page_text()
            .times(1)
            .returning(|_| Ok(String::from("MOCKED")));

        let val = get_page_text(&downloader, url).await.unwrap();
        assert_eq!(val, "MOCKED");
    }

    #[tokio::test]
    async fn test_get_page_text_mocked_mockito() {
        let mut server = mockito::Server::new();
        // let host = server.host_with_port();
        let url = server.url();

        let mock_server = server
            .mock("GET", "/crates/mockall")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("MOCKED MOCKITO")
            .create();

        let val = get_page_text(&Downloader, &format!("{url}/crates/mockall"))
            .await
            .unwrap();
        assert_eq!(val, "MOCKED MOCKITO");

        mock_server.assert();
    }
}
