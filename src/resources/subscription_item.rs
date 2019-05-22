// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{SubscriptionId, SubscriptionItemId};
use crate::params::{Expand, List, Metadata, Object, Timestamp};
use crate::resources::{Plan, SubscriptionItemBillingThresholds, TaxRate};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionItem".
///
/// For more details see [https://stripe.com/docs/api/subscription_items/object](https://stripe.com/docs/api/subscription_items/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionItem {
    /// Unique identifier for the object.
    pub id: SubscriptionItemId,

    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    deleted: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,

    /// The [quantity](https://stripe.com/docs/subscriptions/quantities) of the plan to which the customer should be subscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The `subscription` this `subscription_item` belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    /// The tax rates which apply to this `subscription_item`.
    ///
    /// When set, the `default_tax_rates` on the subscription do not apply to this `subscription_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
}

impl SubscriptionItem {
    /// Returns a list of your subscription items for a given subscription.
    pub fn list(
        client: &Client,
        params: SubscriptionItemListParams<'_>,
    ) -> Response<List<SubscriptionItem>> {
        client.get_query("/subscription_items", &params)
    }
}

impl Object for SubscriptionItem {
    type Id = SubscriptionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription_item"
    }
}

/// The parameters for `SubscriptionItem::list`.
#[derive(Clone, Debug, Serialize)]
pub struct SubscriptionItemListParams<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a SubscriptionItemId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a SubscriptionItemId>,

    /// The ID of the subscription whose items will be retrieved.
    subscription: SubscriptionId,
}

impl<'a> SubscriptionItemListParams<'a> {
    pub fn new(subscription: SubscriptionId) -> Self {
        SubscriptionItemListParams {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            subscription,
        }
    }
}