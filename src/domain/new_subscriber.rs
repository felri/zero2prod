use crate::domain::SubscriberName;
use crate::domain::SubscriberEmail;

#[derive(serde::Deserialize)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
