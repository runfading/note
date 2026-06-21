use super::models::{
    ActiveModel as NotebookModel, CreateNotebook, Entity as NotebookEntity, NotebookInfo,
};
use crate::common::{ApiPageData, ApiPageQuery, ApiResponse, AppResult, DbPool, Empty};
use crate::notebooks::models;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};

pub async fn page_notebooks(
    pool: &DbPool,
    query: ApiPageQuery,
) -> AppResult<ApiPageData<NotebookInfo>> {
    let page_num = query.page_num.max(1);
    let page_size = query.page_size.max(1);
    let paginator = NotebookEntity::find()
        .order_by_desc(models::Column::UpdatedAt)
        .paginate(pool, page_size);
    let total = paginator.num_items().await?;
    let data = paginator.fetch_page(page_num - 1).await?;

    Ok(ApiResponse::ok(ApiPageData {
        list: data.into_iter().map(Into::into).collect(),
        page_num,
        page_size,
        total,
    }))
}

pub async fn create_notebook(pool: &DbPool, create: CreateNotebook) -> AppResult<i64> {
    let model = NotebookModel::builder().set_name(create.name);

    let model = model.insert(pool).await?;

    Ok(ApiResponse::ok(model.id))
}

pub async fn remove_notebook(pool: &DbPool, id: i64) -> AppResult<Empty> {
    NotebookEntity::delete_by_id(id).exec(pool).await?;
    Ok(ApiResponse::empty_ok())
}
