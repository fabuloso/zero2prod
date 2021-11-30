use reqwest::Client;

use crate::domain::subscriber_email::SubscriberEmail;

#[derive(Clone)]
pub struct EmailClient {
    sender: SubscriberEmail,
    base_url: String,
    client: Client,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, base_url: String) -> EmailClient {
        EmailClient {
            sender,
            base_url,
            client: Client::new(),
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!();
    }
}

#[cfg(test)]
mod tests {

    use fake::{
        faker::{
            internet::en::SafeEmail,
            lorem::en::{Paragraph, Sentence},
        },
        Fake,
    };
    use wiremock::{matchers::any, Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();

        let recipient = SubscriberEmail::parse("gianni@gianni.it".to_string()).unwrap();
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;
        let email_client = EmailClient::new(sender, mock_server.uri());
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_client
            .send_email(recipient, &subject, &content, &content)
            .await;
    }
}
