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

    pub async fn post<S, T>(&self, path: S, payload: T) -> Result<surf::Response, surf::Error>
    where
        S: Into<String>,
        T: Into<surf::Body>,
    {
        self.client.post(path.into()).body(payload.into()).await
    }

    pub async fn put<S, T>(&self, path: S, payload: T) -> Result<surf::Response, surf::Error>
    where
        S: Into<String>,
        T: Into<surf::Body>,
    {
        self.client.put(path.into()).body(payload.into()).await
    }

    pub async fn patch<S, T>(&self, path: S, payload: T) -> Result<surf::Response, surf::Error>
    where
        S: Into<String>,
        T: Into<surf::Body>,
    {
        self.client.patch(path.into()).body(payload.into()).await
    }

    pub async fn delete<S, T>(&self, path: S, payload: T) -> Result<surf::Response, surf::Error>
    where
        S: Into<String>,
        T: Into<surf::Body>,
    {
        self.client.delete(path.into()).body(payload.into()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::agent::*;
    use mockito;
    use serde_json::json;

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

    #[async_std::test]
    async fn test_agent_post() -> surf::Result<()> {
        let base_url = &mockito::server_url();
        let path = "/hello";
        let payload = json!({ "hello": "world"});
        let header_name = "content-type";
        let header_value = "text/plain";
        let body = r#"{"hello": "world"}"#;

        let _m = mockito::mock("POST", path)
            .with_status(surf::StatusCode::Created as usize)
            .with_header(header_name, header_value)
            .with_body(body)
            .create();

        let agent = Agent::new(base_url);
        let mut response = agent.post(path, payload).await?;
        assert_eq!(response.status(), surf::StatusCode::Created);
        assert_eq!(response.header(header_name).unwrap(), header_value);
        assert_eq!(response.body_string().await?, body);

        Ok(())
    }

    #[async_std::test]
    async fn test_agent_put() -> surf::Result<()> {
        let base_url = &mockito::server_url();
        let path = "/hello";
        let payload = json!({ "hello": "world"});
        let header_name = "content-type";
        let header_value = "text/plain";
        let body = r#"{"hello": "world"}"#;

        let _m = mockito::mock("PUT", path)
            .with_status(surf::StatusCode::Created as usize)
            .with_header(header_name, header_value)
            .with_body(body)
            .create();

        let agent = Agent::new(base_url);
        let mut response = agent.put(path, payload).await?;
        assert_eq!(response.status(), surf::StatusCode::Created);
        assert_eq!(response.header(header_name).unwrap(), header_value);
        assert_eq!(response.body_string().await?, body);

        Ok(())
    }

    #[async_std::test]
    async fn test_agent_patch() -> surf::Result<()> {
        let base_url = &mockito::server_url();
        let path = "/hello";
        let payload = json!({ "hello": "world"});
        let header_name = "content-type";
        let header_value = "text/plain";
        let body = r#"{"hello": "world"}"#;

        let _m = mockito::mock("PATCH", path)
            .with_status(surf::StatusCode::Created as usize)
            .with_header(header_name, header_value)
            .with_body(body)
            .create();

        let agent = Agent::new(base_url);
        let mut response = agent.patch(path, payload).await?;
        assert_eq!(response.status(), surf::StatusCode::Created);
        assert_eq!(response.header(header_name).unwrap(), header_value);
        assert_eq!(response.body_string().await?, body);

        Ok(())
    }

    #[async_std::test]
    async fn test_agent_delete() -> surf::Result<()> {
        let base_url = &mockito::server_url();
        let path = "/hello";
        let payload = json!({ "hello": "world"});
        let header_name = "content-type";
        let header_value = "text/plain";
        let body = r#"{"hello": "world"}"#;

        let _m = mockito::mock("DELETE", path)
            .with_status(surf::StatusCode::Ok as usize)
            .with_header(header_name, header_value)
            .with_body(body)
            .create();

        let agent = Agent::new(base_url);
        let mut response = agent.delete(path, payload).await?;
        assert_eq!(response.status(), surf::StatusCode::Ok);
        assert_eq!(response.header(header_name).unwrap(), header_value);
        assert_eq!(response.body_string().await?, body);

        Ok(())
    }
}
