// SPDX-FileCopyrightText: 2019 Lemmy Developers
//
// SPDX-License-Identifier: AGPL-3.0-only

use actix_web::web::{Data, Json, Query};
use lemmy_api_common::{
  context::LemmyContext,
  person::{ListMedia, ListMediaResponse},
  utils::is_admin,
};
use lemmy_db_views::structs::{LocalImageView, LocalUserView};
use lemmy_utils::error::LemmyResult;

#[tracing::instrument(skip(context))]
pub async fn list_all_media(
  data: Query<ListMedia>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<ListMediaResponse>> {
  // Only let admins view all media
  is_admin(&local_user_view)?;

  let page = data.page;
  let limit = data.limit;
  let images = LocalImageView::get_all(&mut context.pool(), page, limit).await?;
  Ok(Json(ListMediaResponse { images }))
}
