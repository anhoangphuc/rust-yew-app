use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Feedback {
    pub id: uuid::Uuid,
    pub text: String,
    pub rating: u8,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Clone, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub feedbacks: Vec<Feedback>,
    pub loading: bool,
    pub alert_input: AlertInput,
}

pub fn set_feedback(feedback: Feedback, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.feedbacks.insert(0, feedback);
    })
}

pub fn delete_feedback(id: uuid::Uuid, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.feedbacks.retain(|feedback| feedback.id != id);
    })
}

pub fn set_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.loading = loading;
    })
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = true;
        store.alert_input.alert_message = message;
    })
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}
