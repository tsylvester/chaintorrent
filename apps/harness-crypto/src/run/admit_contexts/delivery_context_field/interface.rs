#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryContextField {
    EntitlementContract,
    SourceEntitlement,
    TargetEntitlement,
    OldInterval,
    NewInterval,
    Seller,
    Buyer,
}
