/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Order : An order for a pets from the pet store

use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "petId", skip_serializing_if = "Option::is_none")]
    pub pet_id: Option<i64>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(rename = "shipDate", skip_serializing_if = "Option::is_none")]
    pub ship_date: Option<String>,
    /// Order Status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

impl Order {
    /// An order for a pets from the pet store
    pub fn new() -> Order {
        Order {
            id: None,
            pet_id: None,
            quantity: None,
            ship_date: None,
            status: None,
            complete: None,
        }
    }
}

/// Order Status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "placed")]
    Placed,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "delivered")]
    Delivered,
}

