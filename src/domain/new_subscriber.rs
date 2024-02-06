use super::SubscriberEmail;
use crate::domain::SubscriberName;

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
