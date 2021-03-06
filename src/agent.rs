use surf;
use url::Url;

#[derive(Clone)]
pub struct Agent {
    client: surf::Client,
    user_agent: String,
}

impl Agent {
    pub fn new(base_url: impl Into<String>) -> Agent {
        let mut client = surf::client();
        client.set_base_url(Url::parse(&base_url.into()).unwrap());

        Agent {
            client: client,
            user_agent: String::from(""),
        }
    }

    pub fn set_user_agent(&mut self, user_agent: impl Into<String>) {
        self.user_agent = user_agent.into();
    }

    pub async fn get(&self, path: impl Into<String>) -> Result<surf::Response, surf::Error> {
        self.client
            .get(path.into())
            .header("User-Agent", self.user_agent.clone())
            .await
    }

    pub async fn post(
        &self,
        path: impl Into<String>,
        payload: impl Into<surf::Body>,
    ) -> Result<surf::Response, surf::Error> {
        self.client
            .post(path.into())
            .header("User-Agent", self.user_agent.clone())
            .body(payload.into())
            .await
    }

    pub async fn put(
        &self,
        path: impl Into<String>,
        payload: impl Into<surf::Body>,
    ) -> Result<surf::Response, surf::Error> {
        self.client
            .put(path.into())
            .header("User-Agent", self.user_agent.clone())
            .body(payload.into())
            .await
    }

    pub async fn patch(
        &self,
        path: impl Into<String>,
        payload: impl Into<surf::Body>,
    ) -> Result<surf::Response, surf::Error> {
        self.client
            .patch(path.into())
            .header("User-Agent", self.user_agent.clone())
            .body(payload.into())
            .await
    }

    pub async fn delete(
        &self,
        path: impl Into<String>,
        payload: impl Into<surf::Body>,
    ) -> Result<surf::Response, surf::Error> {
        self.client
            .delete(path.into())
            .header("User-Agent", self.user_agent.clone())
            .body(payload.into())
            .await
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
        let user_agent = "BENCH_RS";
        let path = "/hello";
        let header_name = "content-type";
        let header_value = "text/plain";
        let body = "hello, world!!";

        let _m = mockito::mock("GET", path)
            .match_header("User-Agent", user_agent)
            .with_status(surf::StatusCode::Ok as usize)
            .with_header(header_name, header_value)
            .with_body(body)
            .create();

        let mut agent = Agent::new(base_url);
        agent.set_user_agent(user_agent);
        let mut response = agent.get(path).await?;
        assert_eq!(response.status(), surf::StatusCode::Ok);
        assert_eq!(response.header(header_name).unwrap(), header_value);
        assert_eq!(response.body_string().await?, body);

        Ok(())
    }

    #[async_std::test]
    async fn test_agent_post() -> surf::Result<()> {
        let base_url = &mockito::server_url();
        let user_agent = "BENCH_RS";
        let path = "/hello";
        let payload = json!({ "hello": "world"});
        let header_name = "content-type";
        let header_value = "text/plain";
        let body = r#"{"hello": "world"}"#;

        let _m = mockito::mock("POST", path)
            .match_header("User-Agent", user_agent)
            .with_status(surf::StatusCode::Created as usize)
            .with_header(header_name, header_value)
            .with_body(body)
            .create();

        let mut agent = Agent::new(base_url);
        agent.set_user_agent(user_agent);
        let mut response = agent.post(path, payload).await?;
        assert_eq!(response.status(), surf::StatusCode::Created);
        assert_eq!(response.header(header_name).unwrap(), header_value);
        assert_eq!(response.body_string().await?, body);

        Ok(())
    }

    #[async_std::test]
    async fn test_agent_put() -> surf::Result<()> {
        let base_url = &mockito::server_url();
        let user_agent = "BENCH_RS";
        let path = "/hello";
        let payload = json!({ "hello": "world"});
        let header_name = "content-type";
        let header_value = "text/plain";
        let body = r#"{"hello": "world"}"#;

        let _m = mockito::mock("PUT", path)
            .match_header("User-Agent", user_agent)
            .with_status(surf::StatusCode::Created as usize)
            .with_header(header_name, header_value)
            .with_body(body)
            .create();

        let mut agent = Agent::new(base_url);
        agent.set_user_agent(user_agent);
        let mut response = agent.put(path, payload).await?;
        assert_eq!(response.status(), surf::StatusCode::Created);
        assert_eq!(response.header(header_name).unwrap(), header_value);
        assert_eq!(response.body_string().await?, body);

        Ok(())
    }

    #[async_std::test]
    async fn test_agent_patch() -> surf::Result<()> {
        let base_url = &mockito::server_url();
        let user_agent = "BENCH_RS";
        let path = "/hello";
        let payload = json!({ "hello": "world"});
        let header_name = "content-type";
        let header_value = "text/plain";
        let body = r#"{"hello": "world"}"#;

        let _m = mockito::mock("PATCH", path)
            .match_header("User-Agent", user_agent)
            .with_status(surf::StatusCode::Created as usize)
            .with_header(header_name, header_value)
            .with_body(body)
            .create();

        let mut agent = Agent::new(base_url);
        agent.set_user_agent(user_agent);
        let mut response = agent.patch(path, payload).await?;
        assert_eq!(response.status(), surf::StatusCode::Created);
        assert_eq!(response.header(header_name).unwrap(), header_value);
        assert_eq!(response.body_string().await?, body);

        Ok(())
    }

    #[async_std::test]
    async fn test_agent_delete() -> surf::Result<()> {
        let base_url = &mockito::server_url();
        let user_agent = "BENCH_RS";
        let path = "/hello";
        let payload = json!({ "hello": "world"});
        let header_name = "content-type";
        let header_value = "text/plain";
        let body = r#"{"hello": "world"}"#;

        let _m = mockito::mock("DELETE", path)
            .match_header("User-Agent", user_agent)
            .with_status(surf::StatusCode::Ok as usize)
            .with_header(header_name, header_value)
            .with_body(body)
            .create();

        let mut agent = Agent::new(base_url);
        agent.set_user_agent(user_agent);
        let mut response = agent.delete(path, payload).await?;
        assert_eq!(response.status(), surf::StatusCode::Ok);
        assert_eq!(response.header(header_name).unwrap(), header_value);
        assert_eq!(response.body_string().await?, body);

        Ok(())
    }
}
