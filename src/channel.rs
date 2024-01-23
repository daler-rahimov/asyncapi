use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{ChannelBinding, Message, Parameter, ReferenceOr};

/// Describes the operations available on a single channel.
///
/// # Examples
/// ```json
/// {
///     "description": "This channel is used to exchange messages about users signing up",
///     "subscribe": {
///         "summary": "A user signed up.",
///         "message": {
///         "description": "A longer description of the message",
///         "payload": {
///             "type": "object",
///             "properties": {
///             "user": {
///                 "$ref": "#/components/schemas/user"
///             },
///             "signup": {
///                 "$ref": "#/components/schemas/signup"
///             }
///             }
///         }
///         }
///     },
///     "bindings": {
///         "amqp": {
///         "is": "queue",
///         "queue": {
///             "exclusive": true
///         }
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// description: This channel is used to exchange messages about users signing up
/// subscribe:
///   summary: A user signed up.
///   message:
///     description: A longer description of the message
///     payload:
///       type: object
///       properties:
///         user:
///           $ref: "#/components/schemas/user"
///         signup:
/// bindings:
///   amqp:
///     is: queue
///     queue:
///       exclusive: true
/// ```
///
/// Using `oneOf` to specify multiple messages per operation:
///
/// ```json
/// {
///     "subscribe": {
///         "message": {
///         "oneOf": [
///             { "$ref": "#/components/messages/signup" },
///             { "$ref": "#/components/messages/login" }
///         ]
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// subscribe:
///   message:
///     oneOf:
///       - $ref: '#/components/messages/signup'
///       - $ref: '#/components/messages/login'
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    /// TODO: add docs
    pub messages: IndexMap<String, Option<OperationMessageType>>,
    /// TODO: add docs
    /// TODO: check if this need to have it's own struct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// An optional description of this channel item.
    /// [CommonMark syntax](https://spec.commonmark.org/) can be used for rich
    /// text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The servers on which this channel is available, specified as an optional unordered
    /// list of names (string keys) of [Server Objects][crate::Server] defined in the
    /// [Servers Object][crate::Server] (a map). If `servers` is absent or empty then this
    /// channel must be available on all servers defined in the [Servers Object][crate::Server].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<String>,
    /// A map of the parameters included in the channel name. It SHOULD be present only
    /// when using channels with expressions (as defined by
    /// [RFC 6570 section 2.2](https://tools.ietf.org/html/rfc6570#section-2.2)).
    ///
    /// Describes a map of parameters included in a channel name.
    ///
    /// This map MUST contain all the parameters used in the parent channel name.
    ///
    /// # Examples
    ///
    /// ```json
    /// {
    ///     "user/{userId}/signup": {
    ///         "parameters": {
    ///             "userId": {
    ///                 "description": "Id of the user.",
    ///                 "schema": {
    ///                    "type": "string"
    ///                 }
    ///             }
    ///         },
    ///         "subscribe": {
    ///             "$ref": "#/components/messages/userSignedUp"
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ```yaml
    /// user/{userId}/signup:
    ///   parameters:
    ///     userId:
    ///       description: Id of the user.
    ///       schema:
    ///         type: string
    ///   subscribe:
    ///     $ref: "#/components/messages/userSignedUp"
    /// ```
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub parameters: IndexMap<String, ReferenceOr<Parameter>>,
    /// A map where the keys describe the name of the protocol and the values
    /// describe protocol-specific definitions for the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<ReferenceOr<ChannelBinding>>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
    /// Allows for an external definition of this channel item. The referenced structure
    /// MUST be in the format of a
    /// [Channel Item Object][crate::Channel].
    /// If there are conflicts between the referenced definition and this Channel Item's
    /// definition, the behavior is *undefined*.
    #[deprecated(note = "The $ref field in Channel Item Object is now deprecated
        from AsyncAPI 2.3.0. The current plan is that the $ref field will be
        removed from Channel Item Object in AsyncAPI 3.0, and replaced with
        Reference Object.")]
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OperationMessageType {
    Map(IndexMap<String, ReferenceOr<Message>>),
    Single(ReferenceOr<Message>),
}
