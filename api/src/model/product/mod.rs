//! # Product model
//! 
//! Rust has a strong type system and some nice ownership and immutability
//! properties that we can leverage when designing our model.
//! The purpose of a domain model is to enforce the invariants for some entity.
//! An example of an invariant is that _the title must not be blank_.
//! Rust has no `null`, so we have to opt-in to nullability with the [`Option`]()
//! type.
//! Rust by itself isn't going to reject `""` so we need to build that invariant
//! ourselves.
//! 
//! In this simple domain, the `Product` is our one and only entity.
//! That makes it really easy to model.
//! 
//! When other entities come along, perhaps `Categories` or `Author`s,
//! then we need to be able to model those and their invariants, and the
//! invaraints that now need to hold between related entities.
//! A tempting solution is to just lump them all onto the one structure,
//! afterall _a product has a category and an author_ is easy to model
//! this way.
//! We can then label our `Product` as an [_aggregate_]() of a number of
//! entities.
//! The problem with this approach is that we can end up with _One Aggregate
//! to Rule Them All_, that is difficult to slice and refactor.
//! We're better off keeping the `Product` simple, and modelling these new
//! entities independently, ensuring invariants hold when their members can
//! be changed.
//! 
//! So if we were going to expand this domain with more entities,
//! that's when we'd consider a move away from Elasticsearch as the primary
//! datastore and look at something more relational, like [Postgres]().
//! We certainly wouldn't throw that work away though, Elasticsearch is still
//! a very valuable query engine.
//! So we compose our domain model into a simpler _query model_, maybe by
//! denormalising products, categories and authors into a single structure
//! that's easy to query and display.
//! This is where the value of CQRS becomes apparent.

/// Queries that return slices of the domain.
pub mod queries;
/// Commands that update domain.
pub mod commands;

/// A product id.
/// 
/// A `ProductId` must be a non-empty string with a maximum length of 32 chars.
pub struct ProductId(String);

impl AsRef<str> for ProductId {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

/// A price in dollars.
/// 
/// A `Price` is a non-zero number with 2 decimal point precision.
/// 
/// > Note we've opted for the name `Price` over something more general
/// like `Money`.
/// This is because the only concept of money in our product catalogue is
/// the price, so that's what we model.
pub struct Price(f32);

/// A product.
pub struct Product {
	id: ProductId,
	title: String,
	details: String,
	price: Price
}