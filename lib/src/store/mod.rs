//! RDF quads storage implementations.
//!
//! They encode a [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset)
//! and allow querying and updating them using SPARQL.

pub mod memory;
pub(crate) mod numeric_encoder;
#[cfg(feature = "rocksdb")]
pub mod rocksdb;
#[cfg(feature = "sled")]
pub mod sled;

use crate::sparql::GraphPattern;
pub use crate::store::memory::MemoryStore;
#[cfg(feature = "rocksdb")]
pub use crate::store::rocksdb::RocksDbStore;
#[cfg(feature = "sled")]
pub use crate::store::sled::SledStore;

use crate::model::*;
use crate::store::numeric_encoder::*;
use crate::{DatasetSyntax, Error, GraphSyntax, Result};
use rio_api::parser::{QuadsParser, TriplesParser};
use rio_turtle::{NQuadsParser, NTriplesParser, TriGParser, TurtleParser};
use rio_xml::RdfXmlParser;
use std::collections::HashMap;
use std::io::BufRead;
use std::iter::Iterator;

pub(crate) trait ReadableEncodedStore: StrLookup {
    fn encoded_quads_for_pattern<'a>(
        &'a self,
        subject: Option<EncodedTerm>,
        predicate: Option<EncodedTerm>,
        object: Option<EncodedTerm>,
        graph_name: Option<EncodedTerm>,
    ) -> Box<dyn Iterator<Item = Result<EncodedQuad>> + 'a>;
}

pub(crate) trait WritableEncodedStore: StrContainer {
    fn insert_encoded(&mut self, quad: &EncodedQuad) -> Result<()>;

    fn remove_encoded(&mut self, quad: &EncodedQuad) -> Result<()>;
}

fn load_graph<S: WritableEncodedStore>(
    store: &mut S,
    reader: impl BufRead,
    syntax: GraphSyntax,
    to_graph_name: &GraphName,
    base_iri: Option<&str>,
) -> Result<()> {
    let base_iri = base_iri.unwrap_or("");
    match syntax {
        GraphSyntax::NTriples => {
            load_from_triple_parser(store, NTriplesParser::new(reader)?, to_graph_name)
        }
        GraphSyntax::Turtle => {
            load_from_triple_parser(store, TurtleParser::new(reader, base_iri)?, to_graph_name)
        }
        GraphSyntax::RdfXml => {
            load_from_triple_parser(store, RdfXmlParser::new(reader, base_iri)?, to_graph_name)
        }
    }
}

fn load_from_triple_parser<S: WritableEncodedStore, P: TriplesParser>(
    store: &mut S,
    mut parser: P,
    to_graph_name: &GraphName,
) -> Result<()>
where
    Error: From<P::Error>,
{
    let mut bnode_map = HashMap::default();
    let to_graph_name = store.encode_graph_name(to_graph_name)?;
    parser.parse_all(&mut move |t| {
        let quad = store.encode_rio_triple_in_graph(t, to_graph_name, &mut bnode_map)?;
        store.insert_encoded(&quad)
    })
}

fn load_dataset<S: WritableEncodedStore>(
    store: &mut S,
    reader: impl BufRead,
    syntax: DatasetSyntax,
    base_iri: Option<&str>,
) -> Result<()> {
    let base_iri = base_iri.unwrap_or("");
    match syntax {
        DatasetSyntax::NQuads => load_from_quad_parser(store, NQuadsParser::new(reader)?),
        DatasetSyntax::TriG => load_from_quad_parser(store, TriGParser::new(reader, base_iri)?),
    }
}

fn load_from_quad_parser<S: WritableEncodedStore, P: QuadsParser>(
    store: &mut S,
    mut parser: P,
) -> Result<()>
where
    Error: From<P::Error>,
{
    let mut bnode_map = HashMap::default();
    parser.parse_all(&mut move |q| {
        let quad = store.encode_rio_quad(q, &mut bnode_map)?;
        store.insert_encoded(&quad)
    })
}
