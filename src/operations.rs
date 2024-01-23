use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    Channel, ExternalDocumentation, Message, OperationBinding, OperationTrait, ReferenceOr, Schema,
    Tag,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ActionType {
    #[serde(rename = "receive")]
    Receive,
    #[serde(rename = "send")]
    Send,
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::Receive
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OperationMessageType {
    Schema(Schema),
    Any(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OperationChannelType {
    Schema(Schema),
    Any(serde_json::Value),
}

/// TODO: remove these docs and add v3 ones
/// Describes a publish or a subscribe operation. This provides a place to document how
/// and why messages are sent and received.
///
/// For example, an operation might describe a chat application use case where a user sends
/// a text message to a group. A publish operation describes messages that are received by
/// the chat application, whereas a subscribe operation describes messages that are sent by
/// the chat application.
///
/// # Examples
/// ```json
/// {
///     "operationId": "registerUser",
///     "summary": "Action to sign a user up.",
///     "description": "A longer description",
///     "tags": [
///         { "name": "user" },
///         { "name": "signup" },
///         { "name": "register" }
///     ],
///     "message": {
///         "headers": {
///         "type": "object",
///         "properties": {
///             "applicationInstanceId": {
///             "description": "Unique identifier for a given instance of the publishing application",
///             "type": "string"
///             }
///         }
///         },
///         "payload": {
///         "type": "object",
///         "properties": {
///             "user": {
///             "$ref": "#/components/schemas/userCreate"
///             },
///             "signup": {
///             "$ref": "#/components/schemas/signup"
///             }
///         }
///         }
///     },
///     "bindings": {
///         "amqp": {
///         "ack": false
///         }
///     },
///     "traits": [
///         { "$ref": "#/components/operationTraits/kafka" }
///     ]
/// }
/// ```
///
/// ```yaml
/// operationId: registerUser
/// summary: Action to sign a user up.
/// description: A longer description
/// tags:
///   - name: user
///   - name: signup
///   - name: register
/// message:
///   headers:
///     type: object
///     properties:
///       applicationInstanceId:
///         description: Unique identifier for a given instance of the publishing application
///         type: string
///   payload:
///     type: object
///     properties:
///       user:
///         $ref: "#/components/schemas/userCreate"
///       signup:
///         $ref: "#/components/schemas/signup"
/// bindings:
///   amqp:
///     ack: false
/// traits:
///   - $ref: "#/components/operationTraits/kafka"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    /// **Required** The type of operation. Can only be `send` or `receive`.
    pub action: ActionType,
    /// The message that will be sent or received.
    /// TODO: more docs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<OperationChannelType>,

    /// TODO: more docs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<OperationMessageType>>,

    /// Unique string used to identify the operation.
    /// The id MUST be unique among all operations described in the API.
    /// The operationId value is **case-sensitive**.
    /// Tools and libraries MAY use the operationId to uniquely identify an
    /// operation, therefore, it is RECOMMENDED to follow common programming
    /// naming conventions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// A short summary of what the operation is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation.
    /// [CommonMark syntax](https://spec.commonmark.org/)
    /// can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// A map where the keys describe the name of the protocol and the
    /// values describe protocol-specific definitions for the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<ReferenceOr<OperationBinding>>,
    /// A list of traits to apply to the operation object.
    /// Traits MUST be merged into the operation object using the
    /// [JSON Merge Patch](https://tools.ietf.org/html/rfc7386)
    /// algorithm in the same order they are defined here.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub traits: Vec<ReferenceOr<OperationTrait>>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
