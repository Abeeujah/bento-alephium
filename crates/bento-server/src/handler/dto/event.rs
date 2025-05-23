use bento_types::EventModel;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::Pagination;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventDto {
    pub id: String,
    pub tx_id: String,
    pub contract_address: String,
    pub event_index: i32,
    pub fields: serde_json::Value,
}

impl From<EventModel> for EventDto {
    fn from(model: EventModel) -> Self {
        Self {
            id: model.id,
            tx_id: model.tx_id,
            contract_address: model.contract_address,
            event_index: model.event_index,
            fields: model.fields,
        }
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams, ToSchema, Serialize)]
#[into_params(style = Form, parameter_in = Query)]
pub struct EventsQuery {
    #[serde(flatten)]
    #[param(inline, example = json!({"offset": 0, "limit": 10}))]
    pub pagination: Pagination,
}

#[derive(Debug, Default, Deserialize, IntoParams, ToSchema, Serialize)]
#[into_params(style = Form, parameter_in = Query)]
pub struct EventByContractQuery {
    /// The contract ID to filter events by
    pub contract: String,

    // Include the pagination fields
    #[param(inline, example = json!({"offset": 0, "limit": 10}))]
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, IntoParams, ToSchema, Serialize, Deserialize)]
#[into_params(style = Form, parameter_in = Query)]
pub struct EventByTxIdQuery {
    /// The transaction ID to filter events by
    pub tx_id: String,

    // Include the pagination fields
    #[param(inline, example = json!({"offset": 0, "limit": 10}))]
    #[serde(flatten)]
    pub pagination: Pagination,
}
