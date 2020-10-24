use surf;
use url::Url;

pub struct Agent {
    client: surf::Client,
}

impl Agent {
    pub fn new(base_url: &str) -> Agent {
        let mut client = surf::client();
        client.set_base_url(Url::parse(base_url).unwrap());

        Agent { client: client }
    }

    pub async fn get<S>(&self, path: S) -> Result<surf::Response, surf::Error>
    where
        S: Into<String>,
    {
        self.client.get(path.into()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::agent::*;
    use mockito;

    #[async_std::test]
    async fn test_agent_get() -> surf::Result<()> {
        let base_url = &mockito::server_url();
        let path = "/hello";
        let header_name = "content-type";
        let header_value = "text/plain";
        let body = "hello, world!!";

        let _m = mockito::mock("GET", path)
            .with_status(surf::StatusCode::Ok as usize)
            .with_header(header_name, header_value)
            .with_body(body)
            .create();

        let agent = Agent::new(base_url);
        let mut response = agent.get(path).await?;
        assert_eq!(response.status(), surf::StatusCode::Ok);
        assert_eq!(response.header(header_name).unwrap(), header_value);
        assert_eq!(response.body_string().await?, body);

        Ok(())
    }
}
