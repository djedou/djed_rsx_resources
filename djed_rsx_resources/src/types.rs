
use djed_rsx_shared::traits::{
    key_traits::{TFontKeysAPI, TImageKeysAPI},
    resource_traits::{TResourceGroup}
};

use files::types::SharedFiles;
use fonts::types::SharedFonts;
use images::types::SharedImages;

#[derive(Debug, PartialEq)]
pub struct ResourceGroup<ImageKeysAPI: TImageKeysAPI, FontKeysAPI: TFontKeysAPI> {
    files: SharedFiles,
    images: SharedImages<ImageKeysAPI>,
    fonts: SharedFonts<FontKeysAPI>
}

impl<ImageKeysAPI, FontKeysAPI> TResourceGroup for ResourceGroup<ImageKeysAPI, FontKeysAPI>
where
    ImageKeysAPI: TImageKeysAPI + 'static,
    FontKeysAPI: TFontKeysAPI + 'static
{
    type Files = SharedFiles;
    type Images = SharedImages<ImageKeysAPI>;
    type Fonts = SharedFonts<FontKeysAPI>;

    fn files(&self) -> Self::Files {
        Self::Files::clone(&self.files)
    }

    fn images(&self) -> Self::Images {
        Self::Images::clone(&self.images)
    }

    fn fonts(&self) -> Self::Fonts {
        Self::Fonts::clone(&self.fonts)
    }
}

impl<ImageKeysAPI, FontKeysAPI> ResourceGroup<ImageKeysAPI, FontKeysAPI>
where
    ImageKeysAPI: TImageKeysAPI,
    FontKeysAPI: TFontKeysAPI
{
    pub fn new<FileCache, ImagesCache, FontCache>(files: FileCache, images: ImagesCache, fonts: FontCache) -> Self
    where
        FileCache: Into<SharedFiles>,
        ImagesCache: Into<SharedImages<ImageKeysAPI>>,
        FontCache: Into<SharedFonts<FontKeysAPI>>
    {
        ResourceGroup {
            files: files.into(),
            images: images.into(),
            fonts: fonts.into()
        }
    }
}
