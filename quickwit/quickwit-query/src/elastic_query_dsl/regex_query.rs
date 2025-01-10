// Copyright (C) 2024 Quickwit, Inc.
//
// Quickwit is offered under the AGPL v3.0 and as commercial software.
// For commercial licensing, contact us at hello@quickwit.io.
//
// AGPL:
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use serde::Deserialize;

use crate::elastic_query_dsl::one_field_map::OneFieldMap;
use crate::elastic_query_dsl::ConvertibleToQueryAst;
use crate::query_ast::{QueryAst, RegexQuery as AstRegexQuery};

#[derive(Deserialize, Debug, Default, Eq, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct RegexQueryParams {
    value: String,
    // we could probably add case_insensitive
}

pub type RegexQuery = OneFieldMap<RegexQueryParams>;

impl ConvertibleToQueryAst for RegexQuery {
    fn convert_to_query_ast(self) -> anyhow::Result<QueryAst> {
        Ok(AstRegexQuery {
            field: self.field,
            regex: self.value.value,
        }
        .into())
    }
}
