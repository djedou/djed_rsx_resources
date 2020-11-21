
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use types::GlyphsArray;

impl<GlyphInstance> Serialize for GlyphsArray<GlyphInstance>
where
    GlyphInstance: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_some(&self.0)
    }
}

impl<'de, GlyphInstance> Deserialize<'de> for GlyphsArray<GlyphInstance> {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        unimplemented!()
    }
}
