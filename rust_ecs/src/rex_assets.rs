use rltk::{rex::XpFile};
use std::fs::*;
use std::io::prelude::*;
use super::{CardID};

//rltk::embedded_resource!(SMALL_DUNGEON, "../resources/SmallDungeon_80x50.xp");
rltk::embedded_resource!(DEBUG_CARD, "../resources/Debug/Cards.xp");
rltk::embedded_resource!(CUR_CARDS, "../resources/cards/001.xp");

pub struct RexAssets {
    pub cur_card : XpFile,
    pub cur_id : CardID,
    pub de_card : XpFile
}


impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        //rltk::link_resource!(SMALL_DUNGEON, "../resources/SmallDungeon_80x50.xp");
        rltk::link_resource!(DEBUG_CARD, "../resources/Debug/Cards.xp");
        rltk::link_resource!(CUR_CARDS, "../resources/cards/001.xp");

        RexAssets{
            de_card : XpFile::from_resource("../resources/Debug/Cards.xp").unwrap(),
            cur_card : XpFile::from_resource("../resources/cards/001.xp").unwrap(),
            cur_id : CardID{id: 1},
        }
    }
    

    pub fn load_new_card(&mut self, id: &CardID) {
        let mut path = *b"resources/cards/000.xp";
        path[18] = 48 + (id.id % 10) as u8;
        path[17] = 48 + (id.id % 100 / 10) as u8;
        path[16] = 48 + (id.id / 100) as u8;
        let true_path = std::str::from_utf8(&path[..]).unwrap();
        println!("{:?}", true_path);
        let mut card_file = File::open(&true_path).unwrap();
        self.cur_card = XpFile::read(&mut card_file).unwrap();
        self.cur_id = *id;
    }
}