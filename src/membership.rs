// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ChurchRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub id:        Uuid,
    pub user_id:   Uuid,
    pub church_id: Uuid,
    pub role:      ChurchRole,
    pub joined_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinChurch {
    pub church_id: Uuid,
    pub role:      ChurchRole
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemberRole {
    pub user_id: Uuid,
    pub role:    ChurchRole
}
