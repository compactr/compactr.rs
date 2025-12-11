//! Thread-safe schema registry for managing and resolving schema references.

use super::SchemaType;
use crate::error::{Result, SchemaError};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

/// A thread-safe registry for storing and resolving schemas.
///
/// The registry allows schemas to reference each other by name,
/// supporting the OpenAPI `$ref` pattern.
#[derive(Debug, Clone)]
pub struct SchemaRegistry {
    schemas: Arc<RwLock<HashMap<String, SchemaType>>>,
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaRegistry {
    /// Creates a new empty schema registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            schemas: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a schema with the given name.
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned (should not happen in normal usage).
    pub fn register(&self, name: impl Into<String>, schema: SchemaType) -> Result<()> {
        let name = name.into();
        let mut schemas = self.schemas.write().map_err(|_| {
            SchemaError::InvalidSchema("Failed to acquire write lock".to_owned())
        })?;
        schemas.insert(name, schema);
        Ok(())
    }

    /// Retrieves a schema by name.
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    pub fn get(&self, name: &str) -> Result<Option<SchemaType>> {
        let schemas = self.schemas.read().map_err(|_| {
            SchemaError::InvalidSchema("Failed to acquire read lock".to_owned())
        })?;
        Ok(schemas.get(name).cloned())
    }

    /// Resolves a schema reference, handling circular references.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The reference cannot be found
    /// - A circular reference is detected
    /// - The lock is poisoned
    pub fn resolve_ref(&self, reference: &str) -> Result<SchemaType> {
        let mut visited = HashSet::new();
        self.resolve_ref_internal(reference, &mut visited)
    }

    fn resolve_ref_internal(
        &self,
        reference: &str,
        visited: &mut HashSet<String>,
    ) -> Result<SchemaType> {
        // Parse reference (format: "#/ComponentName")
        let name = self.parse_reference(reference)?;

        // Check for circular references
        if !visited.insert(name.clone()) {
            return Err(SchemaError::CircularReference(name).into());
        }

        // Get the schema
        let schema = self
            .get(&name)?
            .ok_or_else(|| SchemaError::UnresolvedReference(name.clone()))?;

        // If it's another reference, resolve recursively
        if let SchemaType::Reference(ref inner_ref) = schema {
            self.resolve_ref_internal(inner_ref, visited)
        } else {
            Ok(schema)
        }
    }

    fn parse_reference(&self, reference: &str) -> Result<String> {
        // Support both "#/ComponentName" and "ComponentName" formats
        let name = if let Some(stripped) = reference.strip_prefix("#/") {
            stripped
        } else {
            reference
        };

        if name.is_empty() {
            return Err(SchemaError::InvalidReference(reference.to_owned()).into());
        }

        Ok(name.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_get() {
        let registry = SchemaRegistry::new();
        registry
            .register("User", SchemaType::string())
            .unwrap();

        let schema = registry.get("User").unwrap();
        assert_eq!(schema, Some(SchemaType::string()));
    }

    #[test]
    fn test_resolve_simple_reference() {
        let registry = SchemaRegistry::new();
        registry
            .register("User", SchemaType::string())
            .unwrap();

        let resolved = registry.resolve_ref("#/User").unwrap();
        assert_eq!(resolved, SchemaType::string());
    }

    #[test]
    fn test_circular_reference_detection() {
        let registry = SchemaRegistry::new();
        registry
            .register("A", SchemaType::reference("#/B"))
            .unwrap();
        registry
            .register("B", SchemaType::reference("#/A"))
            .unwrap();

        let result = registry.resolve_ref("#/A");
        assert!(matches!(
            result,
            Err(crate::error::Error::Schema(
                SchemaError::CircularReference(_)
            ))
        ));
    }

    #[test]
    fn test_unresolved_reference() {
        let registry = SchemaRegistry::new();
        let result = registry.resolve_ref("#/NonExistent");
        assert!(matches!(
            result,
            Err(crate::error::Error::Schema(
                SchemaError::UnresolvedReference(_)
            ))
        ));
    }
}
