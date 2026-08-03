//! Shared identity and structural vocabulary.
//!
//! These types preserve distinctions used across contracts. They carry no
//! authority to infer equivalence, resolve meaning, or execute runtime work.

use std::{fmt, str::FromStr};

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, de};
use uuid::Uuid;

/// Canonical identity of one corpus semantic object.
///
/// It is parsed as a UUID and serializes in canonical hyphenated form. It does
/// not by itself contain the object's topology, identifiers, or authored
/// structure, and it cannot stand in for any other semantic identity.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct SemanticObjectId(#[schemars(with = "String")] Uuid);

impl SemanticObjectId {
    /// Parses a UUID-bearing object identity.
    pub fn parse(value: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(value).map(Self)
    }

    /// Returns the canonical UUID value.
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for SemanticObjectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.hyphenated().fmt(formatter)
    }
}

impl FromStr for SemanticObjectId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for SemanticObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(de::Error::custom)
    }
}

macro_rules! opaque_identity {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        ///
        /// This is an opaque, non-empty identity. Its string has no implied UUID
        /// syntax and grants no authority beyond stable reference to its record.
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
        #[serde(transparent)]
        #[schemars(transparent)]
        pub struct $name(String);

        impl $name {
            /// Parses a non-empty opaque identity.
            pub fn parse(value: impl Into<String>) -> Result<Self, EmptyIdentityError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EmptyIdentityError);
                }
                Ok(Self(value))
            }

            /// Returns the authored opaque identity string.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = EmptyIdentityError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::parse(value).map_err(de::Error::custom)
            }
        }
    };
}

opaque_identity!(
    SemanticUnitId,
    "Canonical identity of one independently addressable authored semantic unit."
);
opaque_identity!(
    OccurrenceId,
    "Identity of one authored link or relation occurrence with provenance."
);
opaque_identity!(
    TemporalAnchorId,
    "Identity of one materially sourced temporal anchor record."
);
opaque_identity!(
    TransportSegmentId,
    "Identity of one technical segment subordinate to a canonical semantic unit."
);

/// Error returned when an opaque identity is empty or whitespace-only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyIdentityError;

impl fmt::Display for EmptyIdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("identity must not be empty")
    }
}

impl std::error::Error for EmptyIdentityError {}

/// Address of an authored structural region inside one semantic object.
///
/// The address preserves both the canonical parent object and a non-empty
/// authored structural address. It has no authority to select a unit, infer a
/// heading target, or treat the region as a separate epistemic object class.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SemanticRegionAddress {
    /// Canonical parent semantic object.
    pub object_id: SemanticObjectId,
    /// Non-empty authored heading, block, or structural address.
    pub authored_structural_address: String,
}

impl SemanticRegionAddress {
    /// Constructs an address after validating the authored structural part.
    pub fn parse(
        object_id: SemanticObjectId,
        authored_structural_address: impl Into<String>,
    ) -> Result<Self, EmptyIdentityError> {
        let authored_structural_address = authored_structural_address.into();
        if authored_structural_address.trim().is_empty() {
            return Err(EmptyIdentityError);
        }
        Ok(Self {
            object_id,
            authored_structural_address,
        })
    }
}

impl<'de> Deserialize<'de> for SemanticRegionAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawAddress {
            object_id: SemanticObjectId,
            authored_structural_address: String,
        }

        let raw = RawAddress::deserialize(deserializer)?;
        Self::parse(raw.object_id, raw.authored_structural_address).map_err(de::Error::custom)
    }
}

/// Stable categories of address represented by the projection.
///
/// These labels describe structural input and output kinds. They do not judge
/// semantic relevance or establish that a requested address exists.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AddressKind {
    /// Canonical semantic object identity.
    SemanticObject,
    /// Canonical semantic unit identity.
    SemanticUnit,
    /// Authored region address within an object.
    SemanticRegion,
    /// Identifier assignment or descriptor address.
    Identifier,
    /// Authored occurrence identity.
    Occurrence,
    /// Materially sourced temporal anchor.
    TemporalAnchor,
    /// Retrieval-surface address.
    RetrievalSurface,
}

/// Canonical or projected address used by plans and provenance records.
///
/// It can point only to represented address categories. It does not prove that
/// the target exists in a given snapshot; structural conformance owns that
/// later check.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum SemanticAddress {
    /// Canonical semantic object.
    Object(SemanticObjectId),
    /// Canonical semantic unit.
    Unit(SemanticUnitId),
    /// Authored semantic region.
    Region(SemanticRegionAddress),
    /// Identifier assignment or descriptor.
    Identifier(IdentifierAddress),
    /// Authored occurrence.
    Occurrence(OccurrenceId),
    /// Temporal anchor.
    TemporalAnchor(TemporalAnchorId),
    /// Retrieval surface.
    RetrievalSurface(RetrievalSurfaceAddress),
}

impl SemanticAddress {
    /// Returns the structural kind of this address.
    pub const fn kind(&self) -> AddressKind {
        match self {
            Self::Object(_) => AddressKind::SemanticObject,
            Self::Unit(_) => AddressKind::SemanticUnit,
            Self::Region(_) => AddressKind::SemanticRegion,
            Self::Identifier(_) => AddressKind::Identifier,
            Self::Occurrence(_) => AddressKind::Occurrence,
            Self::TemporalAnchor(_) => AddressKind::TemporalAnchor,
            Self::RetrievalSurface(_) => AddressKind::RetrievalSurface,
        }
    }
}

/// Address of an identifier descriptor or assignment.
///
/// It represents a named projected surface and optional represented value. It
/// cannot replace canonical object or unit identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IdentifierAddress {
    /// Identifier name represented by the projection.
    pub identifier_name: String,
    /// Optional exact represented value used to address one assignment.
    pub represented_value: Option<String>,
}

/// Address of a retrieval-surface descriptor.
///
/// It names a projected capability only. It does not execute the surface or
/// imply that the surface is available for a particular address.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RetrievalSurfaceAddress {
    /// Snapshot-local retrieval-surface identifier.
    pub surface_id: String,
}

/// Explicit direction for a represented structural connection.
///
/// Direction authorizes no traversal by itself; it is valid only when the
/// projection represents the requested incidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    /// Move from source incidence toward its represented target.
    Outgoing,
    /// Move from a target toward represented incoming incidence.
    Incoming,
}

/// Execution obligation attached to a plan operation or requested output.
///
/// It states whether failure affects support for the requested route. It does
/// not determine truth or semantic adequacy.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Requirement {
    /// Failure must be reported as affecting the requested route.
    Required,
    /// Failure is recorded but need not invalidate the primary route.
    Optional,
}

/// Retrieval-surface families admitted by the contracts.
///
/// A family describes a capability class only; the projection determines
/// whether a concrete surface exists and what it may inspect.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalSurfaceKind {
    /// Literal or exact matching.
    Exact,
    /// Lexical matching.
    Lexical,
    /// Vector-neighbour lookup.
    Vector,
    /// Graph or incidence navigation.
    Graph,
    /// Temporal lookup or ordering.
    Temporal,
}

/// Authored or materialized source span.
///
/// This is provenance for a represented record. It cannot reinterpret the
/// source or establish semantic relevance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SourceSpan {
    /// Source path or stable source identity.
    pub source: String,
    /// Inclusive starting byte offset when known.
    pub start_byte: Option<u64>,
    /// Exclusive ending byte offset when known.
    pub end_byte: Option<u64>,
}

/// Provenance of a projected or retrieved fact.
///
/// It records where a fact was materialized from. It may not upgrade source
/// authority or turn contextual participation into intrinsic typing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecordProvenance {
    /// Frontmatter field on a canonical source object.
    ObjectField {
        /// Canonical object that owns the field.
        object_id: SemanticObjectId,
        /// Authored field path.
        field_path: String,
    },
    /// Authored body span inside a canonical semantic unit.
    SemanticUnit {
        /// Canonical semantic unit containing the source.
        unit_id: SemanticUnitId,
        /// Optional exact source span.
        source_span: Option<SourceSpan>,
    },
    /// Deterministically materialized structural record.
    Materialization {
        /// Named materialization rule or contract source.
        rule: String,
        /// Source record addresses used by the rule.
        sources: Vec<SemanticAddress>,
    },
}
