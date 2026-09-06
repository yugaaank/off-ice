//! Application actions.

use gpui::Action;
use serde::Deserialize;
use schemars::JsonSchema;

/// Action to create a new document.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, JsonSchema, Action)]
#[action(namespace = office)]
pub struct NewDocument;

/// Action to open an existing document.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, JsonSchema, Action)]
#[action(namespace = office)]
pub struct OpenDocument;

/// Action to save the current document.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, JsonSchema, Action)]
#[action(namespace = office)]
pub struct SaveDocument;

/// Action to save the current document as a new file.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, JsonSchema, Action)]
#[action(namespace = office)]
pub struct SaveDocumentAs;
