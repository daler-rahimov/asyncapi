mod api;
mod channel;
pub mod channel_binding;
mod components;
mod correlation_id;
mod discriminator;
mod example;
mod external_documentation;
mod info;
mod message;
pub mod message_binding;
mod message_trait;
pub mod operation_binding;
mod operation_trait;
mod parameter;
mod reference;
pub mod schema;
mod security_scheme;
mod server;
pub mod server_binding;
mod tag;
mod variant_or;
mod vec_or_single;

pub use api::AsyncAPI;
pub use channel::{Channel, Operation};
pub use channel_binding::ChannelBinding;
pub use components::Components;
pub use correlation_id::CorrelationId;
pub use discriminator::Discriminator;
pub use example::Example;
pub use external_documentation::ExternalDocumentation;
pub use info::{Contact, Info, License};
pub use message::Message;
pub use message_binding::MessageBinding;
pub use message_trait::MessageTrait;
pub use operation_binding::OperationBinding;
pub use operation_trait::OperationTrait;
pub use parameter::Parameter;
pub use reference::ReferenceOr;
pub use schema::Schema;
pub use security_scheme::SecurityScheme;
pub use server::{SecurityRequirement, Server, ServerVariable};
pub use server_binding::ServerBinding;
pub use tag::Tag;
pub use variant_or::{VariantOrUnknown, VariantOrUnknownOrEmpty};

#[allow(clippy::trivially_copy_pass_by_ref)] // needs to match signature for use in serde attribute
#[inline]
pub(crate) const fn is_false(v: &bool) -> bool {
    !(*v)
}
