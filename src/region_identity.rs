//! Deterministic canonical identity for authored heading regions.
//!
//! This module turns already-observed authored heading structure into region
//! addresses. It does not resolve authored targets or infer semantic meaning.

use std::collections::HashMap;

use crate::model::{SemanticObjectId, SemanticRegionAddress, SourceSpan};

/// One authored heading observation supplied to the region materializer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthoredRegionHeading {
    /// Authored heading depth. Any positive depth is structural; skipped levels
    /// remain faithful to the source hierarchy by using the nearest shallower
    /// preceding heading as the parent.
    pub level: u8,
    /// The observer/materializer's structural address for this heading.
    pub authored_structural_address: String,
    /// Exact source provenance. It is deliberately excluded from identity.
    pub source_span: Option<SourceSpan>,
}

/// A canonical region identity plus the provenance retained beside it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalRegionIdentity {
    /// Existing canonical object-plus-structural region address.
    pub address: SemanticRegionAddress,
    /// Authored heading path, retained separately from the serialized address.
    pub heading_path: Vec<String>,
    /// Exact authored heading-marker provenance, when available.
    pub source_span: Option<SourceSpan>,
}

#[derive(Clone, Debug)]
struct HeadingNode {
    parent_index: Option<usize>,
    authored_structural_address: String,
    source_span: Option<SourceSpan>,
}

/// Materializes one unique canonical region address per authored heading.
///
/// Parent relationships are established first from authored order and depth.
/// Equivalent siblings are counted only within that concrete parent
/// occurrence, then receive one-based authored-order ordinals when a collision
/// exists. The wire string is length-prefixed so legal heading content cannot
/// collide with another structural component. Source spans never participate.
pub fn canonical_region_identities(
    object_id: SemanticObjectId,
    headings: &[AuthoredRegionHeading],
) -> Result<Vec<CanonicalRegionIdentity>, crate::model::EmptyIdentityError> {
    let mut nodes = Vec::with_capacity(headings.len());
    let mut stack: Vec<(u8, usize)> = Vec::new();

    for heading in headings {
        while stack
            .last()
            .is_some_and(|(parent_level, _)| *parent_level >= heading.level)
        {
            stack.pop();
        }
        let parent_index = stack.last().map(|(_, index)| *index);
        let index = nodes.len();
        nodes.push(HeadingNode {
            parent_index,
            authored_structural_address: heading.authored_structural_address.clone(),
            source_span: heading.source_span.clone(),
        });
        stack.push((heading.level, index));
    }

    let mut sibling_counts: HashMap<(Option<usize>, String), usize> = HashMap::new();
    for node in &nodes {
        *sibling_counts
            .entry((node.parent_index, node.authored_structural_address.clone()))
            .or_default() += 1;
    }

    let mut sibling_ordinals: HashMap<(Option<usize>, String), usize> = HashMap::new();
    let mut canonical_components: Vec<Vec<CanonicalComponent>> = Vec::with_capacity(nodes.len());
    let mut identities = Vec::with_capacity(nodes.len());

    for (index, node) in nodes.iter().enumerate() {
        let sibling_key = (node.parent_index, node.authored_structural_address.clone());
        let sibling_count = sibling_counts[&sibling_key];
        let sibling_ordinal = sibling_ordinals.entry(sibling_key).or_default();
        if sibling_count > 1 {
            *sibling_ordinal += 1;
        }

        let mut components = node
            .parent_index
            .map(|parent| canonical_components[parent].clone())
            .unwrap_or_default();
        components.push(CanonicalComponent {
            authored_structural_address: node.authored_structural_address.clone(),
            collision_ordinal: (sibling_count > 1).then_some(*sibling_ordinal),
        });
        let authored_structural_address = encode_components(&components);
        let address = SemanticRegionAddress::parse(object_id.clone(), authored_structural_address)?;
        let heading_path = components
            .iter()
            .map(|component| component.authored_structural_address.clone())
            .collect();
        canonical_components.push(components);
        identities.push(CanonicalRegionIdentity {
            address,
            heading_path,
            source_span: node.source_span.clone(),
        });

        debug_assert_eq!(
            canonical_components[index].len(),
            identities[index].heading_path.len()
        );
    }

    Ok(identities)
}

#[derive(Clone, Debug)]
struct CanonicalComponent {
    authored_structural_address: String,
    collision_ordinal: Option<usize>,
}

/// Encodes the full hierarchy without relying on a delimiter that authored
/// heading content could contain. Byte lengths make the representation
/// injective and independent of machine paths, offsets, and runtime state.
fn encode_components(components: &[CanonicalComponent]) -> String {
    let mut encoded = String::from("region-v1:");
    for component in components {
        let byte_length = component.authored_structural_address.len();
        encoded.push_str(&byte_length.to_string());
        encoded.push(':');
        encoded.push_str(&component.authored_structural_address);
        encoded.push(':');
        encoded.push_str(
            &component
                .collision_ordinal
                .map_or_else(|| "0".to_owned(), |ordinal| ordinal.to_string()),
        );
        encoded.push(';');
    }
    encoded
}
