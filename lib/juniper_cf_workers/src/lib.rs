use juniper::http::GraphQLBatchRequest;
use juniper::http::GraphQLRequest;
use serde::Deserialize;
use serde_json::json;
use worker::{Method, Request, Response, Result};

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
struct GetGraphQLRequest {
    query: String,
    #[serde(rename = "operationName")]
    operation_name: Option<String>,
    variables: Option<String>,
}

impl<S> From<GetGraphQLRequest> for GraphQLRequest<S>
where
    S: juniper::ScalarValue,
{
    fn from(get_req: GetGraphQLRequest) -> Self {
        let GetGraphQLRequest {
            query,
            operation_name,
            variables,
        } = get_req;
        let variables = variables.map(|s| serde_json::from_str(&s).unwrap());
        Self::new(query, operation_name, variables)
    }
}

pub fn graphiql_source(
    graphql_endpoint_url: &str,
    subscriptions_endpoint_url: Option<&str>,
) -> Result<Response> {
    Response::from_html(juniper::http::graphiql::graphiql_source(
        graphql_endpoint_url,
        subscriptions_endpoint_url,
    ))
}

pub fn playground_source(
    graphql_endpoint_url: &str,
    subscriptions_endpoint_url: Option<&str>,
) -> Result<Response> {
    Response::from_html(juniper::http::playground::playground_source(
        graphql_endpoint_url,
        subscriptions_endpoint_url,
    ))
}

pub async fn graphql_handler<Query, Mutation, Subscription, CtxT, S>(
    schema: &juniper::RootNode<'static, Query, Mutation, Subscription, S>,
    context: &CtxT,
    req: &mut Request,
) -> Result<Response>
where
    Query: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Query::TypeInfo: Sync,
    Mutation: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Mutation::TypeInfo: Sync,
    Subscription: juniper::GraphQLSubscriptionType<S, Context = CtxT>,
    Subscription::TypeInfo: Sync,
    CtxT: Sync,
    S: juniper::ScalarValue + Send + Sync,
{
    match req.method() {
        Method::Get => get_graphql_handler(schema, context, req).await,
        Method::Post => post_graphql_handler(schema, context, req).await,
        _ => Response::error("Method not supported!", 503),
    }
}

pub async fn get_graphql_handler<Query, Mutation, Subscription, CtxT, S>(
    schema: &juniper::RootNode<'static, Query, Mutation, Subscription, S>,
    context: &CtxT,
    req: &mut Request,
) -> Result<Response>
where
    Query: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Query::TypeInfo: Sync,
    Mutation: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Mutation::TypeInfo: Sync,
    Subscription: juniper::GraphQLSubscriptionType<S, Context = CtxT>,
    Subscription::TypeInfo: Sync,
    CtxT: Sync,
    S: juniper::ScalarValue + Send + Sync,
{
    let url_query = req.url()?;
    let gql_query = url_query
        .query_pairs()
        .filter(|(key, _)| key == "query")
        .next();

    let gql_request = serde_json::from_str::<GraphQLBatchRequest<S>>(&gql_query.unwrap().1)?;
    let gql_batch_response = gql_request.execute(schema, context).await;
    let response = match gql_batch_response.is_ok() {
        true => Response::from_json(&json!(&gql_batch_response)),
        false => Response::error("Could not execute GET request/query", 500),
    };

    response
}

pub async fn post_graphql_handler<Query, Mutation, Subscription, CtxT, S>(
    schema: &juniper::RootNode<'static, Query, Mutation, Subscription, S>,
    context: &CtxT,
    req: &mut Request,
) -> Result<Response>
where
    Query: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Query::TypeInfo: Sync,
    Mutation: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Mutation::TypeInfo: Sync,
    Subscription: juniper::GraphQLSubscriptionType<S, Context = CtxT>,
    Subscription::TypeInfo: Sync,
    CtxT: Sync,
    S: juniper::ScalarValue + Send + Sync,
{
    let gql_request = serde_json::from_str::<GraphQLBatchRequest<S>>(&req.text().await?).unwrap();
    let gql_batch_response = gql_request.execute(schema, context).await;
    let response = match gql_batch_response.is_ok() {
        true => Response::from_json(&json!(&gql_batch_response)),
        false => Response::error("Could not execute POST request/query", 500),
    };

    response
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
