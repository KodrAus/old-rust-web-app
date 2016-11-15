//! # Model
//! 
//! The domain for this application is an online retailer, with an ever-growing
//! product catalogue and their own on-site warehouse.
//! 
//! The [_bounded context_] of this application is the catalogue, that can
//! be updated and freely browsed.
//! The name of the game is discoverability from a business perspective,
//! and scalability from a technical perspective.
//! For storage, the products are persisted in [Elasticsearch](https://elastic.co) 
//! because it offers a simple CRUD document model with an advanced query DSL for search.
//! It's ok to use a database that favours availablity over consistency in the face
//! of a failure because the catalogue isn't storing _mission critical_ data.
//! If we were storing customer orders or stock keeping, we'd look at a more
//! approriate solution.
//! 
//! While a bounded context shouldn't really be a _technical_ boundary, one of the
//! perks of finding a good seam is that different aspects of a business domain
//! need different things, so the decisions made in one context don't need to bleed
//! into another one within the same domain.
//! 
//! For this application, we use a simple [CQRS]() approach, which is easy to design,
//! develop and scale.
//! If you explore the `worker` module you'll see some of the tradeoffs we need to make
//! in Rust around threading.
//! Knowing that we can't waste threads, we might want to lump a few commands on a single
//! worker to process.
//! This should be invisible to both the command being called, and the command caller.
//! Queues shouldn't be relied on as a consistency provider, because multiple consumers
//! can pull from the same queue.

/// Our product model.
pub mod product;