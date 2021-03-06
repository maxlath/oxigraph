use oxiri::{Iri, IriParseError};
use rio_api::model as rio;
use std::fmt;

/// An RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri)
///
/// The default string formatter is returning a N-Triples, Turtle and SPARQL compatible representation:
/// ```
/// use oxigraph::model::NamedNode;
///
/// assert_eq!(
///     "<http://example.com/foo>",
///     NamedNode::new("http://example.com/foo").unwrap().to_string()
/// )
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Hash)]
pub struct NamedNode {
    iri: String,
}

impl NamedNode {
    /// Builds and validate an RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri)
    pub fn new(iri: impl Into<String>) -> Result<Self, IriParseError> {
        Ok(Self::new_from_iri(Iri::parse(iri.into())?))
    }

    #[deprecated(note = "Use the `new` method")]
    pub fn parse(iri: impl Into<String>) -> Result<Self, IriParseError> {
        Self::new(iri)
    }

    pub(crate) fn new_from_iri(iri: Iri<String>) -> Self {
        Self::new_unchecked(iri.into_inner())
    }

    /// Builds an RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri) from a string.
    ///
    /// It is the caller's responsibility to ensure that `iri` is a valid IRI.
    ///
    /// Except if you really know what you do, you should use [`parse`](#method.parse).
    pub fn new_unchecked(iri: impl Into<String>) -> Self {
        Self { iri: iri.into() }
    }

    pub fn as_str(&self) -> &str {
        self.iri.as_str()
    }

    pub fn into_string(self) -> String {
        self.iri
    }
}

impl fmt::Display for NamedNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        rio::NamedNode::from(self).fmt(f)
    }
}

impl<'a> From<&'a NamedNode> for rio::NamedNode<'a> {
    fn from(node: &'a NamedNode) -> Self {
        rio::NamedNode { iri: node.as_str() }
    }
}

impl PartialEq<str> for NamedNode {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<NamedNode> for str {
    fn eq(&self, other: &NamedNode) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<&str> for NamedNode {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<NamedNode> for &str {
    fn eq(&self, other: &NamedNode) -> bool {
        *self == other
    }
}
