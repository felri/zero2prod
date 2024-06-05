use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;

#[derive(serde::Deserialize)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
