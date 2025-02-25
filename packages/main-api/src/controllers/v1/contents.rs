use by_axum::aide;
use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;
use validator::Validate;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct ContentPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct ContentController {
    repo: ContentRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ContentController {
    async fn create_bulk(
        &self,
        _auth: Option<Authorization>,
        _body: Vec<ContentCreateRequest>,
    ) -> Result<Json<Content>> {
        let mut tx = self.pool.begin().await?;
        let mut docs = vec![];

        for item in _body {
            item.validate()?;
            let ContentCreateRequest {
                creator_id,
                title,
                thumbnail_image,
                source,
                description,
            } = item;

            if let Some(doc) = self
                .repo
                .insert_with_tx(
                    &mut *tx,
                    title,
                    thumbnail_image,
                    source,
                    description,
                    creator_id,
                )
                .await?
            {
                docs.push(doc);
            }
        }

        tx.commit().await?;

        Ok(Json(
            docs.last().ok_or(Error::NoContentAfterInsert)?.clone(),
        ))
    }

    async fn mint(&self, _auth: Option<Authorization>, _id: i64) -> Result<Json<Content>> {
        todo!()
    }

    async fn query(
        &self,
        _auth: Option<Authorization>,
        _param: ContentQuery,
    ) -> Result<QueryResponse<ContentSummary>> {
        let mut total_count = 0;
        let items: Vec<ContentSummary> = ContentSummary::query_builder()
            .order_by_created_at_desc()
            .with_count()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.get("total_count");
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}

impl ContentController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Content::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_content).post(Self::act_content_by_id))
            .with_state(self.clone())
            .route("/", post(Self::act_content).get(Self::list_content))
            .with_state(self.clone()))
    }

    pub async fn act_content(
        State(ctrl): State<ContentController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<ContentAction>,
    ) -> Result<Json<Content>> {
        tracing::debug!("act_content {:?}", body);

        match body {
            ContentAction::CreateBulk(ContentCreateBulkRequest { items }) => {
                ctrl.create_bulk(auth, items).await
            }
            ContentAction::Create(item) => ctrl.create_bulk(auth, vec![item]).await,
        }
    }

    pub async fn act_content_by_id(
        State(ctrl): State<ContentController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(ContentPath { id }): Path<ContentPath>,
        Json(body): Json<ContentByIdAction>,
    ) -> Result<Json<Content>> {
        tracing::debug!("act_content_by_id {:?} {:?}", id, body);

        match body {
            ContentByIdAction::Mint(_) => ctrl.mint(auth, id).await,
        }
    }

    pub async fn get_content(
        State(ctrl): State<ContentController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(ContentPath { id }): Path<ContentPath>,
    ) -> Result<Json<Content>> {
        tracing::debug!("get_content {:?}", id);

        Ok(Json(
            Content::query_builder()
                .id_equals(id)
                .query()
                .map(Content::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn list_content(
        State(ctrl): State<ContentController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<ContentParam>,
    ) -> Result<Json<ContentGetResponse>> {
        tracing::debug!("list_content {:?}", q);

        match q {
            ContentParam::Query(param) => Ok(Json(ContentGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
        }
    }
}
