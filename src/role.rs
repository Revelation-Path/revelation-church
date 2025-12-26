// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "church_role", rename_all = "snake_case")
)]
pub enum ChurchRole {
    Guest,
    Member,
    Deacon,
    Elder,
    Pastor,
    Admin
}

impl ChurchRole {
    pub fn can_manage_content(&self) -> bool {
        matches!(self, Self::Pastor | Self::Admin)
    }

    pub fn can_manage_members(&self) -> bool {
        matches!(self, Self::Pastor | Self::Admin | Self::Elder)
    }
}
