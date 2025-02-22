use flowy_derive::{ProtoBuf, ProtoBuf_Enum};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter, EnumString};

pub const DEFAULT_ROW_HEIGHT: i32 = 36;
pub const DEFAULT_FIELD_WIDTH: i32 = 150;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ProtoBuf)]
pub struct GridMeta {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2)]
    pub fields: Vec<FieldMeta>,

    #[pb(index = 3)]
    pub block_metas: Vec<GridBlockMeta>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, ProtoBuf)]
pub struct GridBlockMeta {
    #[pb(index = 1)]
    pub block_id: String,

    #[pb(index = 2)]
    pub start_row_index: i32,

    #[pb(index = 3)]
    pub row_count: i32,
}

impl GridBlockMeta {
    pub fn len(&self) -> i32 {
        self.row_count
    }

    pub fn is_empty(&self) -> bool {
        self.row_count == 0
    }
}

impl GridBlockMeta {
    pub fn new() -> Self {
        GridBlockMeta {
            block_id: uuid::Uuid::new_v4().to_string(),
            ..Default::default()
        }
    }
}

pub struct GridBlockMetaChangeset {
    pub block_id: String,
    pub start_row_index: Option<i32>,
    pub row_count: Option<i32>,
}

impl GridBlockMetaChangeset {
    pub fn from_row_count(block_id: &str, row_count: i32) -> Self {
        Self {
            block_id: block_id.to_string(),
            start_row_index: None,
            row_count: Some(row_count),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ProtoBuf)]
pub struct GridBlockMetaSerde {
    #[pb(index = 1)]
    pub block_id: String,

    #[pb(index = 2)]
    pub row_metas: Vec<RowMeta>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ProtoBuf, PartialEq, Eq)]
pub struct FieldMeta {
    #[pb(index = 1)]
    pub id: String,

    #[pb(index = 2)]
    pub name: String,

    #[pb(index = 3)]
    pub desc: String,

    #[pb(index = 4)]
    pub field_type: FieldType,

    #[pb(index = 5)]
    pub frozen: bool,

    #[pb(index = 6)]
    pub visibility: bool,

    #[pb(index = 7)]
    pub width: i32,

    #[pb(index = 8)]
    pub type_options: String,
}

impl FieldMeta {
    pub fn new(name: &str, desc: &str, field_type: FieldType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            desc: desc.to_string(),
            field_type,
            frozen: false,
            visibility: true,
            width: DEFAULT_FIELD_WIDTH,
            type_options: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct FieldChangeset {
    #[pb(index = 1)]
    pub field_id: String,

    #[pb(index = 2, one_of)]
    pub name: Option<String>,

    #[pb(index = 3, one_of)]
    pub desc: Option<String>,

    #[pb(index = 4, one_of)]
    pub field_type: Option<FieldType>,

    #[pb(index = 5, one_of)]
    pub frozen: Option<bool>,

    #[pb(index = 6, one_of)]
    pub visibility: Option<bool>,

    #[pb(index = 7, one_of)]
    pub width: Option<i32>,

    #[pb(index = 8, one_of)]
    pub type_options: Option<String>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, ProtoBuf_Enum, EnumCountMacro, EnumString, EnumIter, Display, Serialize, Deserialize,
)]
pub enum FieldType {
    RichText = 0,
    Number = 1,
    DateTime = 2,
    SingleSelect = 3,
    MultiSelect = 4,
    Checkbox = 5,
}

impl std::default::Default for FieldType {
    fn default() -> Self {
        FieldType::RichText
    }
}

impl AsRef<FieldType> for FieldType {
    fn as_ref(&self) -> &FieldType {
        self
    }
}

impl From<&FieldType> for FieldType {
    fn from(field_type: &FieldType) -> Self {
        field_type.clone()
    }
}

impl FieldType {
    pub fn type_id(&self) -> String {
        let ty = self.clone();
        format!("{}", ty as u8)
    }

    pub fn from_type_id(type_id: &str) -> Result<FieldType, String> {
        match type_id {
            "0" => Ok(FieldType::RichText),
            "1" => Ok(FieldType::Number),
            "2" => Ok(FieldType::DateTime),
            "3" => Ok(FieldType::SingleSelect),
            "4" => Ok(FieldType::MultiSelect),
            "5" => Ok(FieldType::Checkbox),
            _ => Err(format!("Invalid type_id: {}", type_id)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ProtoBuf)]
pub struct AnyData {
    #[pb(index = 1)]
    pub type_id: String,

    #[pb(index = 2)]
    pub value: Vec<u8>,
}

impl AnyData {
    pub fn from_str<F: Into<FieldType>>(field_type: F, s: &str) -> AnyData {
        Self::from_bytes(field_type, s.as_bytes().to_vec())
    }

    pub fn from_bytes<T: AsRef<[u8]>, F: Into<FieldType>>(field_type: F, bytes: T) -> AnyData {
        AnyData {
            type_id: field_type.into().type_id(),
            value: bytes.as_ref().to_vec(),
        }
    }
}

impl ToString for AnyData {
    fn to_string(&self) -> String {
        match String::from_utf8(self.value.clone()) {
            Ok(s) => s,
            Err(_) => "".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, ProtoBuf)]
pub struct RowMeta {
    #[pb(index = 1)]
    pub id: String,

    #[pb(index = 2)]
    pub block_id: String,

    #[pb(index = 3)]
    pub cell_by_field_id: HashMap<String, CellMeta>,

    #[pb(index = 4)]
    pub height: i32,

    #[pb(index = 5)]
    pub visibility: bool,
}

impl RowMeta {
    pub fn new(block_id: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            block_id: block_id.to_owned(),
            cell_by_field_id: Default::default(),
            height: DEFAULT_ROW_HEIGHT,
            visibility: true,
        }
    }
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct RowMetaChangeset {
    #[pb(index = 1)]
    pub row_id: String,

    #[pb(index = 2, one_of)]
    pub height: Option<i32>,

    #[pb(index = 3, one_of)]
    pub visibility: Option<bool>,

    #[pb(index = 4)]
    pub cell_by_field_id: HashMap<String, CellMeta>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ProtoBuf)]
pub struct CellMeta {
    #[pb(index = 1)]
    pub field_id: String,

    #[pb(index = 2)]
    pub data: String,
}

impl CellMeta {
    pub fn new(field_id: &str, data: String) -> Self {
        Self {
            field_id: field_id.to_string(),
            data,
        }
    }
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct CellMetaChangeset {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2)]
    pub row_id: String,

    #[pb(index = 3)]
    pub field_id: String,

    #[pb(index = 4, one_of)]
    pub data: Option<String>,
}

impl std::convert::From<CellMetaChangeset> for RowMetaChangeset {
    fn from(changeset: CellMetaChangeset) -> Self {
        let mut cell_by_field_id = HashMap::with_capacity(1);
        if let Some(data) = changeset.data {
            let field_id = changeset.field_id;
            let cell_meta = CellMeta {
                field_id: field_id.clone(),
                data,
            };
            cell_by_field_id.insert(field_id, cell_meta);
        }

        RowMetaChangeset {
            row_id: changeset.row_id,
            height: None,
            visibility: None,
            cell_by_field_id,
        }
    }
}

#[derive(Clone, ProtoBuf)]
pub struct BuildGridContext {
    #[pb(index = 1)]
    pub field_metas: Vec<FieldMeta>,

    #[pb(index = 2)]
    pub block_metas: GridBlockMeta,

    #[pb(index = 3)]
    pub block_meta_data: GridBlockMetaSerde,
}

impl std::default::Default for BuildGridContext {
    fn default() -> Self {
        let grid_block = GridBlockMeta::new();
        let grid_block_meta_data = GridBlockMetaSerde {
            block_id: grid_block.block_id.clone(),
            row_metas: vec![],
        };

        Self {
            field_metas: vec![],
            block_metas: grid_block,
            block_meta_data: grid_block_meta_data,
        }
    }
}
