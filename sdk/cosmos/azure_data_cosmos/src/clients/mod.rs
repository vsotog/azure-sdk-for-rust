// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Clients used to communicate with Azure Cosmos DB

mod container_client;
mod cosmos_client;
mod database_client;

pub use container_client::ContainerClient;
pub use cosmos_client::CosmosClient;
pub use database_client::DatabaseClient;
