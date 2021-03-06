// Plural types
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[cfg(feature = "invariant")]
use crate::prelude::*;

/// Gets a locale-invariant default struct given a data key in this module's category.
#[cfg(feature = "invariant")]
pub(crate) fn get_invariant(data_key: &DataKey) -> Option<DataResponse<'static>> {
    use crate::invariant::make_inv_response;
    if data_key.category != DataCategory::Plurals {
        return None;
    }
    // TODO(#212): Match on TinyStr instead of &str
    match (data_key.sub_category.as_str(), data_key.version) {
        ("cardinal", 1) => make_inv_response::<PluralRuleStringsV1>(),
        ("ordinal", 1) => make_inv_response::<PluralRuleStringsV1>(),
        _ => None,
    }
}

/// Plural rule strings conforming to UTS 35 syntax. Includes separate fields for five of the six
/// standard plural forms. If none of the rules match, the "other" category is assumed.
///
/// More information: https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct PluralRuleStringsV1 {
    pub zero: Option<Cow<'static, str>>,
    pub one: Option<Cow<'static, str>>,
    pub two: Option<Cow<'static, str>>,
    pub few: Option<Cow<'static, str>>,
    pub many: Option<Cow<'static, str>>,
}

#[cfg(feature = "invariant")]
impl Default for PluralRuleStringsV1 {
    fn default() -> Self {
        Self {
            zero: None,
            one: None,
            two: None,
            few: None,
            many: None,
        }
    }
}
