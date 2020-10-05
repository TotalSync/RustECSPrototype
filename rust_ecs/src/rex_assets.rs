
use rltk::{rex::XpFile};

//rltk::embedded_resource!(SMALL_DUNGEON, "../resources/SmallDungeon_80x50.xp");
rltk::embedded_resource!(DEBUG_CARD, "../resources/Custom/Cards.xp");

pub struct RexAssets {
    //pub menu : XpFile,
    pub de_card :XpFile
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        //rltk::link_resource!(SMALL_DUNGEON, "../resources/SmallDungeon_80x50.xp");
        rltk::link_resource!(DEBUG_CARD, "../resources/Custom/Cards.xp");

        RexAssets{
            //menu : XpFile::from_resource("../../resources/SmallDungeon_80x50.xp").unwrap(),
            de_card : XpFile::from_resource("../resources/Custom/Cards.xp").unwrap()
        }
    }
}