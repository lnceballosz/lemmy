// SPDX-FileCopyrightText: 2019 Lemmy Developers
//
// SPDX-License-Identifier: AGPL-3.0-only

use actix_web::web::{Data, Json};
use lemmy_api_common::{context::LemmyContext, person::ListLoginsResponse};
use lemmy_db_schema::source::login_token::LoginToken;
use lemmy_db_views::structs::LocalUserView;
use lemmy_utils::error::LemmyResult;

pub async fn list_logins(
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<ListLoginsResponse>> {
  let logins = LoginToken::list(&mut context.pool(), local_user_view.local_user.id).await?;

  Ok(Json(ListLoginsResponse { logins }))
}
