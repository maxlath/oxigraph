//! Rudf is a work in progress graph database implementing the [SPARQL](https://www.w3.org/TR/sparql11-overview/) standard.
//!
//! Its goal is to provide a compliant, safe and fast graph database.
//!
//! It currently provides two `Repository` implementation providing [SPARQL 1.0 query](https://www.w3.org/TR/rdf-sparql-query/) capability:
//! * `MemoryRepository`: a simple in memory implementation.
//! * `RocksDbRepository`: a file system implementation based on the [RocksDB](https://rocksdb.org/) key-value store.
//!
//! Usage example with the `MemoryRepository`:
//!
//! ```
//! use rudf::model::*;
//! use rudf::{Repository, RepositoryConnection, MemoryRepository, Result};
//! use crate::rudf::sparql::PreparedQuery;
//! use rudf::sparql::algebra::QueryResult;
//!
//! let repository = MemoryRepository::default();
//! let connection = repository.connection().unwrap();
//!
//! // insertion
//! let ex = NamedNode::new("http://example.com");
//! let quad = Quad::new(ex.clone(), ex.clone(), ex.clone(), None);
//! connection.insert(&quad);
//!
//! // quad filter
//! let results: Result<Vec<Quad>> = connection.quads_for_pattern(None, None, None, None).collect();
//! assert_eq!(vec![quad], results.unwrap());
//!
//! // SPARQL query
//! let prepared_query = connection.prepare_query("SELECT ?s WHERE { ?s ?p ?o }".as_bytes()).unwrap();
//! let results = prepared_query.exec().unwrap();
//! if let QueryResult::Bindings(results) = results {
//!     assert_eq!(results.into_values_iter().next().unwrap().unwrap()[0], Some(ex.into()));
//! }
//! ```

pub mod model;
mod repository;
mod rio;
pub mod sparql;
pub(crate) mod store;

pub use failure::Error;
pub type Result<T> = ::std::result::Result<T, failure::Error>;
pub use crate::store::MemoryRepository;
#[cfg(feature = "rocksdb")]
pub use crate::store::RocksDbRepository;
pub use repository::Repository;
pub use repository::RepositoryConnection;
pub use rio::GraphSyntax;
