// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

pub struct StoreItemBuilder;

#[derive(PartialEq, Debug)]
pub struct StoreItem<'a>(
    pub StoreItemPart<'a>,
    pub Option<StoreItemPart<'a>>,
    pub Option<StoreItemPart<'a>>,
);

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct StoreItemPart<'a>(&'a str);

// TODO: Change variant names
#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, Debug)]
pub enum StoreItemError {
    InvalidCollection,
    InvalidBucket,
    InvalidObject,
}

const STORE_ITEM_PART_LEN_MIN: usize = 0;
const STORE_ITEM_PART_LEN_MAX: usize = 128;

impl<'a> StoreItemPart<'a> {
    pub fn from_str(part: &'a str) -> Result<Self, ()> {
        let len = part.len();

        if len > STORE_ITEM_PART_LEN_MIN
            && len <= STORE_ITEM_PART_LEN_MAX
            && part.chars().all(|character| character.is_ascii())
        {
            Ok(StoreItemPart(part))
        } else {
            Err(())
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl<'a> From<StoreItemPart<'a>> for &'a str {
    fn from(part: StoreItemPart<'a>) -> Self {
        part.as_str()
    }
}

impl StoreItemBuilder {
    pub fn from_depth_1(collection: &str) -> Result<StoreItem, StoreItemError> {
        // Validate & box collection
        if let Ok(collection_item) = StoreItemPart::from_str(collection) {
            Ok(StoreItem(collection_item, None, None))
        } else {
            Err(StoreItemError::InvalidCollection)
        }
    }

    pub fn from_depth_2<'a>(
        collection: &'a str,
        bucket: &'a str,
    ) -> Result<StoreItem<'a>, StoreItemError> {
        // Validate & box collection + bucket
        match (
            StoreItemPart::from_str(collection),
            StoreItemPart::from_str(bucket),
        ) {
            (Ok(collection_item), Ok(bucket_item)) => {
                Ok(StoreItem(collection_item, Some(bucket_item), None))
            }
            (Err(_), _) => Err(StoreItemError::InvalidCollection),
            (_, Err(_)) => Err(StoreItemError::InvalidBucket),
        }
    }

    pub fn from_depth_3<'a>(
        collection: &'a str,
        bucket: &'a str,
        object: &'a str,
    ) -> Result<StoreItem<'a>, StoreItemError> {
        // Validate & box collection + bucket + object
        match (
            StoreItemPart::from_str(collection),
            StoreItemPart::from_str(bucket),
            StoreItemPart::from_str(object),
        ) {
            (Ok(collection_item), Ok(bucket_item), Ok(object_item)) => Ok(StoreItem(
                collection_item,
                Some(bucket_item),
                Some(object_item),
            )),
            (Err(_), _, _) => Err(StoreItemError::InvalidCollection),
            (_, Err(_), _) => Err(StoreItemError::InvalidBucket),
            (_, _, Err(_)) => Err(StoreItemError::InvalidObject),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_store_item_depth_1() {
        assert_eq!(
            StoreItemBuilder::from_depth_1("c:test:1"),
            Ok(StoreItem(StoreItemPart("c:test:1"), None, None))
        );
        assert_eq!(
            StoreItemBuilder::from_depth_1(""),
            Err(StoreItemError::InvalidCollection)
        );
    }

    #[test]
    fn it_builds_store_item_depth_2() {
        assert_eq!(
            StoreItemBuilder::from_depth_2("c:test:2", "b:test:2"),
            Ok(StoreItem(
                StoreItemPart("c:test:2"),
                Some(StoreItemPart("b:test:2")),
                None
            ))
        );
        assert_eq!(
            StoreItemBuilder::from_depth_2("", "b:test:2"),
            Err(StoreItemError::InvalidCollection)
        );
        assert_eq!(
            StoreItemBuilder::from_depth_2("c:test:2", ""),
            Err(StoreItemError::InvalidBucket)
        );
    }

    #[test]
    fn it_builds_store_item_depth_3() {
        assert_eq!(
            StoreItemBuilder::from_depth_3("c:test:3", "b:test:3", "o:test:3"),
            Ok(StoreItem(
                StoreItemPart("c:test:3"),
                Some(StoreItemPart("b:test:3")),
                Some(StoreItemPart("o:test:3"))
            ))
        );
        assert_eq!(
            StoreItemBuilder::from_depth_3("", "b:test:3", "o:test:3"),
            Err(StoreItemError::InvalidCollection)
        );
        assert_eq!(
            StoreItemBuilder::from_depth_3("c:test:3", "", "o:test:3"),
            Err(StoreItemError::InvalidBucket)
        );
        assert_eq!(
            StoreItemBuilder::from_depth_3("c:test:3", "b:test:3", ""),
            Err(StoreItemError::InvalidObject)
        );
    }
}
