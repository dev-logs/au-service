use serde::{Deserialize, Serialize};
use surrealdb::{
    opt::{IntoResource, RecordId, Resource},
    sql::Id,
};

#[derive(Debug, Clone)]
pub struct OurId(pub String);

impl Serialize for OurId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for OurId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(RecordId::deserialize(deserializer)?.id.to_raw()))
    }
}

impl IntoResource<OurId> for OurId {
    fn into_resource(self) -> surrealdb::Result<surrealdb::opt::Resource> {
        Ok(Resource::RecordId(RecordId {
            tb: "".to_owned(),
            id: Id::String("".to_owned()),
        }))
    }
}
