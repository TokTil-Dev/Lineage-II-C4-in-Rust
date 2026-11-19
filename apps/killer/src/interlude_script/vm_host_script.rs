use l2_uscript::vm::Value;

use super::host_impl::{HostEvent, ScriptHost, ScriptRequest};
use super::vm_host::vector;
use super::{parse_int, parse_string};

pub const CONSOLE_SOUNDS: [&str; 19] = [
    "InterfaceSound.click_01",
    "itemsound.click_2",
    "itemsound.click_failed",
    "itemsound.pickup",
    "itemsound.trash_basket",
    "itemsound.window_open",
    "itemsound.window_close",
    "itemsound.quest_tutorial",
    "itemSound.itemdrop_etc_paper",
    "itemSound.cooltime_end",
    "itemSound.petition",
    "InterfaceSound.charstat_open_01",
    "InterfaceSound.charstat_close_01",
    "InterfaceSound.inventory_open_01",
    "InterfaceSound.inventory_close_01",
    "InterfaceSound.map_open_01",
    "InterfaceSound.map_close_01",
    "InterfaceSound.system_open_01",
    "InterfaceSound.system_close_01",
];

pub const EVENT_SET_CHAT_STRING: i32 = 0x230;

fn counted_pairs(param: &str, count_name: &str, first: &str, second: &str) -> Vec<(i32, i32)> {
    let count = parse_int(param, count_name).unwrap_or(0).max(0);
    (0..count)
        .map(|i| (parse_int(param, &format!("{first}{i}")).unwrap_or(0), parse_int(param, &format!("{second}{i}")).unwrap_or(0)))
        .collect()
}

fn counted_triples(param: &str, count_name: &str, first: &str, second: &str, third: &str) -> Vec<(i32, i32, i32)> {
    let count = parse_int(param, count_name).unwrap_or(0).max(0);
    (0..count)
        .map(|i| {
            (
                parse_int(param, &format!("{first}{i}")).unwrap_or(0),
                parse_int(param, &format!("{second}{i}")).unwrap_or(0),
                parse_int(param, &format!("{third}{i}")).unwrap_or(0),
            )
        })
        .collect()
}

impl ScriptHost {
    fn request(&mut self, request: ScriptRequest) -> Option<Value> {
        self.queue.push(HostEvent::Request(request));
        Some(Value::None)
    }

    pub fn script_native(&mut self, class: &str, function: &str, args: &mut [Value]) -> Option<Value> {
        let text = |index: usize| args.get(index).map_or_else(String::new, Value::as_str);
        let int = |index: usize| args.get(index).map_or(0, Value::as_int);
        let flag = |index: usize| args.get(index).is_some_and(Value::as_bool);
        let loc = |index: usize| args.get(index).map(vector).unwrap_or_default();
        match (class, function) {
            ("UIScript" | "AudioAPI", "PlaySound") => {
                let name = text(0);
                if !name.is_empty() && !name.eq_ignore_ascii_case("None") {
                    self.queue.push(HostEvent::Sound(name));
                }
                Some(Value::None)
            }
            ("UIScript", "PlayConsoleSound") => {
                if let Some(name) = usize::try_from(int(0)).ok().and_then(|index| CONSOLE_SOUNDS.get(index)) {
                    self.queue.push(HostEvent::Sound((*name).to_string()));
                }
                Some(Value::None)
            }
            ("UIScript", "SetChatMessage") => {
                let mut param = String::new();
                super::param_add(&mut param, "String", &text(0));
                self.queue.push(HostEvent::ExecuteEvent(EVENT_SET_CHAT_STRING, param));
                Some(Value::None)
            }
            ("UIScript", "ProcessChatMessage") => self.request(ScriptRequest::ChatLine { message: text(0), kind: int(1) }),
            ("UIScript", "ExecuteCommand" | "ExecuteCommandFromAction") => self.request(ScriptRequest::Command { command: text(0) }),
            ("UIScript", "Debug") => {
                log::debug!("script {}: {}", self.current, text(0));
                Some(Value::None)
            }
            ("UIScript", "SetOptionBool") => {
                let (section, name) = (text(0), text(1));
                self.option_writes.push((section, name, flag(2).to_string()));
                Some(Value::None)
            }
            ("UIScript", "SetOptionInt") => {
                let (section, name) = (text(0), text(1));
                self.option_writes.push((section, name, int(2).to_string()));
                Some(Value::None)
            }
            ("UIScript", "SetOptionString") => {
                let (section, name, value) = (text(0), text(1), text(2));
                self.option_writes.push((section, name, value));
                Some(Value::None)
            }
            ("UIScript", "SetINIBool") => {
                let key = (text(3), text(0), text(1));
                self.ini_values.insert(key, if flag(2) { "True".into() } else { "False".into() });
                Some(Value::None)
            }
            ("UIScript", "SetINIInt") => {
                let key = (text(3), text(0), text(1));
                self.ini_values.insert(key, int(2).to_string());
                Some(Value::None)
            }
            ("UIScript", "SetINIFloat") => {
                let key = (text(3), text(0), text(1));
                self.ini_values.insert(key, l2_uscript::vm::format_float(args.get(2).map_or(0.0, Value::as_float)));
                Some(Value::None)
            }
            ("UIScript", "SetINIString") => {
                let key = (text(3), text(0), text(1));
                self.ini_values.insert(key, text(2));
                Some(Value::None)
            }
            ("WindowHandle", "SetTimer") => {
                let window = text(0);
                self.queue.push(HostEvent::SetTimer(window, int(1), int(2)));
                Some(Value::None)
            }
            ("WindowHandle", "KillTimer") => {
                let window = text(0);
                self.queue.push(HostEvent::KillTimer(window, int(1)));
                Some(Value::None)
            }
            ("UIScript", "RequestSelfTarget") => self.request(ScriptRequest::SelfTarget),
            ("UIScript", "RequestTargetCancel") => self.request(ScriptRequest::TargetCancel),
            ("UIScript", "RequestTargetUser") => self.request(ScriptRequest::TargetUser { server_id: int(0) }),
            ("UIScript", "RequestAttack") => self.request(ScriptRequest::Attack { server_id: int(0), loc: loc(1) }),
            ("UIScript", "RequestAction") => self.request(ScriptRequest::Action { server_id: int(0), loc: loc(1) }),
            ("UIScript", "RequestAssist") => self.request(ScriptRequest::Assist { server_id: int(0), loc: loc(1) }),
            ("UIScript", "RequestUseItem") => self.request(ScriptRequest::UseItem { server_id: int(0) }),
            ("UIScript", "RequestDestroyItem") => self.request(ScriptRequest::DestroyItem { server_id: int(0), num: int(1) }),
            ("UIScript", "RequestDropItem") => self.request(ScriptRequest::DropItem { server_id: int(0), num: int(1), loc: loc(2) }),
            ("UIScript", "RequestUnequipItem") => self.request(ScriptRequest::UnequipItem { server_id: int(0), slot_bit_type: int(1) }),
            ("UIScript", "RequestCrystallizeItem") => self.request(ScriptRequest::CrystallizeItem { server_id: int(0), num: int(1) }),
            ("UIScript", "RequestItemList") => self.request(ScriptRequest::ItemList),
            ("UIScript", "RequestLoadAllItem") => self.request(ScriptRequest::LoadAllItem),
            ("UIScript", "RequestSkillList") => self.request(ScriptRequest::SkillList),
            ("QuestAPI", "RequestQuestList") => self.request(ScriptRequest::QuestList),
            ("QuestAPI", "RequestDestroyQuest") => self.request(ScriptRequest::DestroyQuest { quest_id: int(0) }),
            ("UIScript", "RequestRestartPoint") => self.request(ScriptRequest::RestartPoint { kind: int(0) }),
            ("UIScript", "RequestExit") => self.request(ScriptRequest::Exit),
            ("UIScript", "RequestStartTrade") => self.request(ScriptRequest::StartTrade { target_id: int(0) }),
            ("UIScript", "RequestAddTradeItem") => self.request(ScriptRequest::AddTradeItem { server_id: int(0), num: int(1) }),
            ("UIScript", "RequestTradeDone") => self.request(ScriptRequest::TradeDone { done: flag(0) }),
            ("UIScript", "AnswerTradeRequest") => self.request(ScriptRequest::AnswerTrade { ok: flag(0) }),
            ("UIScript", "RequestSellItem") => {
                let param = text(0);
                let merchant = parse_int(&param, "merchant").unwrap_or(0);
                let items = counted_triples(&param, "num", "serverID", "classID", "count");
                self.request(ScriptRequest::SellItems { merchant, items })
            }
            ("UIScript", "RequestBuyItem") => {
                let param = text(0);
                let merchant = parse_int(&param, "merchant").unwrap_or(0);
                let items = counted_pairs(&param, "num", "classID", "count");
                self.request(ScriptRequest::BuyItems { merchant, items })
            }
            ("UIScript", "RequestBuySeed") => {
                let param = text(0);
                let merchant = parse_int(&param, "merchant").unwrap_or(0);
                let items = counted_pairs(&param, "num", "classID", "count");
                self.request(ScriptRequest::BuySeed { merchant, items })
            }
            ("UIScript", "RequestProcureCrop") => {
                let param = text(0);
                let merchant = parse_int(&param, "merchant").unwrap_or(0);
                let items = counted_triples(&param, "num", "serverID", "classID", "count");
                self.request(ScriptRequest::ProcureCrop { merchant, items })
            }
            ("UIScript", "RequestPreviewItem") => {
                let param = text(0);
                let (merchant, npc) = (parse_int(&param, "merchant").unwrap_or(0), parse_int(&param, "npc").unwrap_or(0));
                let count = parse_int(&param, "num").unwrap_or(0).max(0);
                let items = (0..count).map(|i| parse_int(&param, &format!("classID{i}")).unwrap_or(0)).collect();
                self.request(ScriptRequest::PreviewItem { merchant, npc, items })
            }
            ("UIScript", "RequestWarehouseDeposit") => {
                let param = text(0);
                self.request(ScriptRequest::WarehouseDeposit { items: counted_pairs(&param, "num", "dbID", "count") })
            }
            ("UIScript", "RequestWarehouseWithdraw") => {
                let param = text(0);
                self.request(ScriptRequest::WarehouseWithdraw { items: counted_pairs(&param, "num", "dbID", "count") })
            }
            ("UIScript", "RequestPackageSend") => {
                let param = text(0);
                let target_id = parse_int(&param, "targetID").unwrap_or(0);
                self.request(ScriptRequest::PackageSend { target_id, items: counted_pairs(&param, "num", "dbID", "count") })
            }
            ("UIScript", "RequestPackageSendableItemList") => self.request(ScriptRequest::PackageSendableItemList { target_id: int(0) }),
            ("UIScript", "RequestMultiSellChoose") => {
                let param = text(0);
                self.request(ScriptRequest::MultiSellChoose {
                    shop_id: parse_int(&param, "ShopID").unwrap_or(0),
                    item_id: parse_int(&param, "ItemID").unwrap_or(0),
                    item_count: parse_int(&param, "ItemCount").unwrap_or(0),
                    refinery_op1: parse_int(&param, "RefineryOp1").unwrap_or(0),
                })
            }
            ("UIScript", "RequestQuitPrivateShop") => self.request(ScriptRequest::QuitPrivateShop { kind: text(0) }),
            ("UIScript", "RequestInviteParty") => self.request(ScriptRequest::InviteParty { name: text(0) }),
            ("UIScript", "RequestAcquireSkillInfo") => self.request(ScriptRequest::AcquireSkillInfo { id: int(0), level: int(1), kind: int(2) }),
            ("UIScript", "RequestAcquireSkill") => self.request(ScriptRequest::AcquireSkill { id: int(0), level: int(1), kind: int(2) }),
            ("UIScript", "RequestExEnchantSkillInfo") => self.request(ScriptRequest::ExEnchantSkillInfo { id: int(0), level: int(1) }),
            ("UIScript", "RequestExEnchantSkill") => self.request(ScriptRequest::ExEnchantSkill { id: int(0), level: int(1) }),
            ("EnchantAPI", "RequestEnchantItem") => self.request(ScriptRequest::EnchantItem { server_id: int(0) }),
            ("UIScript", "RequestBypassToServer") => self.request(ScriptRequest::Bypass { command: text(0) }),
            ("UIScript", "RequestLinkHtml") => self.request(ScriptRequest::LinkHtml { link: text(0) }),
            ("UIScript", "RequestBBSBoard") => self.request(ScriptRequest::BbsBoard),
            ("UIScript", "RequestOpenMinimap") => self.request(ScriptRequest::OpenMinimap),
            ("UIScript", "RequestHennaItemList") => self.request(ScriptRequest::HennaItemList),
            ("UIScript", "RequestHennaItemInfo") => self.request(ScriptRequest::HennaItemInfo { henna_id: int(0) }),
            ("UIScript", "RequestHennaEquip") => self.request(ScriptRequest::HennaEquip { henna_id: int(0) }),
            ("UIScript", "RequestHennaUnEquipList") => self.request(ScriptRequest::HennaUnequipList),
            ("UIScript", "RequestHennaUnEquipInfo") => self.request(ScriptRequest::HennaUnequipInfo { henna_id: int(0) }),
            ("UIScript", "RequestHennaUnEquip") => self.request(ScriptRequest::HennaUnequip { henna_id: int(0) }),
            ("UIScript", "RequestRaidRecord") => self.request(ScriptRequest::RaidRecord),
            ("UIScript", "RequestObserverModeEnd") => self.request(ScriptRequest::ObserverModeEnd),
            ("UIScript", "RequestFishRanking") => self.request(ScriptRequest::FishRanking),
            ("PetAPI", "RequestPetUseItem") => self.request(ScriptRequest::PetUseItem { server_id: int(0) }),
            ("PetAPI", "RequestGiveItemToPet") => self.request(ScriptRequest::GiveItemToPet { server_id: int(0), num: int(1) }),
            ("PetAPI", "RequestGetItemFromPet") => self.request(ScriptRequest::GetItemFromPet { server_id: int(0), num: int(1), is_equip_item: flag(2) }),
            ("PetAPI", "RequestPetInventoryItemList") => self.request(ScriptRequest::PetInventoryItemList),
            ("UIScript", "RequestChangePetName") => self.request(ScriptRequest::ChangePetName { name: text(0) }),
            ("MacroAPI", "RequestMacroList") => self.request(ScriptRequest::MacroList),
            ("MacroAPI", "RequestMakeMacro") => {
                self.queue.push(HostEvent::Request(ScriptRequest::MakeMacro {
                    macro_id: int(0),
                    name: text(1),
                    icon_name: text(2),
                    icon_num: int(3),
                    description: text(4),
                }));
                Some(Value::Bool(true))
            }
            ("MacroAPI", "RequestDeleteMacro") => self.request(ScriptRequest::DeleteMacro { macro_id: int(0) }),
            ("MacroAPI", "RequestUseMacro") => self.request(ScriptRequest::UseMacro { macro_id: int(0) }),
            ("PetitionAPI", "RequestPetition") => self.request(ScriptRequest::Petition { message: text(0), kind: int(1) }),
            ("PetitionAPI", "RequestPetitionCancel") => self.request(ScriptRequest::PetitionCancel),
            ("PetitionAPI", "RequestPetitionFeedBack") => self.request(ScriptRequest::PetitionFeedBack { rate: int(0), message: text(1) }),
            ("GMAPI", "RequestGMCommand") => self.request(ScriptRequest::GmCommand { param: text(0) }),
            ("GMAPI", "RequestSnoopEnd") => self.request(ScriptRequest::SnoopEnd { snoop_id: int(0) }),
            ("OlympiadAPI", "RequestOlympiadMatchList") => self.request(ScriptRequest::OlympiadMatchList),
            ("OlympiadAPI", "RequestOlympiadObserverEnd") => self.request(ScriptRequest::OlympiadObserverEnd),
            ("HeroTowerAPI", "RequestWriteHeroWords") => self.request(ScriptRequest::WriteHeroWords { words: text(0) }),
            ("MiniMapAPI", "RequestCursedWeaponList") => self.request(ScriptRequest::CursedWeaponList),
            ("MiniMapAPI", "RequestCursedWeaponLocation") => self.request(ScriptRequest::CursedWeaponLocation),
            ("PartyMatchAPI", "RequestOpenPartyMatch") => self.request(ScriptRequest::OpenPartyMatch),
            ("PartyMatchAPI", "RequestPartyRoomList") => {
                self.request(ScriptRequest::PartyRoomList { page: int(0), location_filter: int(1), level_filter: int(2) })
            }
            ("PartyMatchAPI", "RequestPartyMatchWaitList") => {
                self.request(ScriptRequest::PartyMatchWaitList { page: int(0), min_level: int(1), max_level: int(2), room_type: int(3) })
            }
            ("PartyMatchAPI", "RequestJoinPartyRoom") => self.request(ScriptRequest::JoinPartyRoom { room_number: int(0) }),
            ("PartyMatchAPI", "RequestJoinPartyRoomAuto") => {
                self.request(ScriptRequest::JoinPartyRoomAuto { page: int(0), location_filter: int(1), level_filter: int(2) })
            }
            ("PartyMatchAPI", "RequestManagePartyRoom") => self.request(ScriptRequest::ManagePartyRoom {
                room_number: int(0),
                max_members: int(1),
                min_level: int(2),
                max_level: int(3),
                title: text(4),
            }),
            ("PartyMatchAPI", "RequestDismissPartyRoom") => self.request(ScriptRequest::DismissPartyRoom { room_number: int(0) }),
            ("PartyMatchAPI", "RequestWithdrawPartyRoom") => self.request(ScriptRequest::WithdrawPartyRoom { room_number: int(0) }),
            ("PartyMatchAPI", "RequestBanFromPartyRoom") => self.request(ScriptRequest::BanFromPartyRoom { member_id: int(0) }),
            ("PartyMatchAPI", "RequestAskJoinPartyRoom") => self.request(ScriptRequest::AskJoinPartyRoom { name: text(0) }),
            ("PartyMatchAPI", "RequestExitPartyMatchingWaitingRoom") => self.request(ScriptRequest::ExitPartyMatchingWaitingRoom),
            ("CommandChannelAPI", "RequestCommandChannelInfo") => self.request(ScriptRequest::CommandChannelInfo),
            ("CommandChannelAPI", "RequestCommandChannelWithdraw") => self.request(ScriptRequest::CommandChannelWithdraw),
            ("CommandChannelAPI", "RequestCommandChannelBanParty") => {
                self.request(ScriptRequest::CommandChannelBanParty { party_master_name: text(0) })
            }
            ("CommandChannelAPI", "RequestCommandChannelPartyMembersInfo") => {
                self.request(ScriptRequest::CommandChannelPartyMembersInfo { master_id: int(0) })
            }
            ("RecipeAPI", "RequestRecipeBookOpen") => self.request(ScriptRequest::RecipeBookOpen { kind: int(0) }),
            ("RecipeAPI", "RequestRecipeItemDelete") => self.request(ScriptRequest::RecipeItemDelete { server_id: int(0) }),
            ("RecipeAPI", "RequestRecipeItemMakeInfo") => self.request(ScriptRequest::RecipeItemMakeInfo { server_id: int(0) }),
            ("RecipeAPI", "RequestRecipeItemMakeSelf") => self.request(ScriptRequest::RecipeItemMakeSelf { recipe_id: int(0) }),
            ("RecipeAPI", "RequestRecipeShopMakeInfo") => self.request(ScriptRequest::RecipeShopMakeInfo { server_id: int(0), recipe_id: int(1) }),
            ("RecipeAPI", "RequestRecipeShopMakeDo") => {
                self.request(ScriptRequest::RecipeShopMakeDo { merchant_id: int(0), recipe_id: int(1), adena: int(2) })
            }
            ("RecipeAPI", "RequestRecipeShopSellList") => self.request(ScriptRequest::RecipeShopSellList { server_id: int(0) }),
            ("RecipeAPI", "RequestRecipeShopMessageSet") => self.request(ScriptRequest::RecipeShopMessageSet { message: text(0) }),
            ("RecipeAPI", "RequestRecipeShopListSet") => {
                let param = text(0);
                self.request(ScriptRequest::RecipeShopListSet { items: counted_pairs(&param, "num", "ServerID_", "Price_") })
            }
            ("RecipeAPI", "RequestRecipeShopManageQuit") => self.request(ScriptRequest::RecipeShopManageQuit),
            ("RefineryAPI", "RequestRefine") => self.request(ScriptRequest::Refine {
                target_item: int(0),
                refiner_item: int(1),
                gem_stone: int(2),
                gem_stone_count: int(3),
            }),
            ("RefineryAPI", "RequestRefineCancel") => self.request(ScriptRequest::RefineCancel { cancel_item: int(0) }),
            ("SSQAPI", "RequestSSQStatus") => self.request(ScriptRequest::SsqStatus { status_id: int(0) }),
            ("SiegeAPI", "RequestCastleSiegeAttackerList") => self.request(ScriptRequest::CastleSiegeAttackerList { castle_id: int(0) }),
            ("SiegeAPI", "RequestCastleSiegeDefenderList") => self.request(ScriptRequest::CastleSiegeDefenderList { castle_id: int(0) }),
            ("SiegeAPI", "RequestJoinCastleSiege") => {
                self.request(ScriptRequest::JoinCastleSiege { castle_id: int(0), is_attacker: int(1), is_register: int(2) })
            }
            ("SiegeAPI", "RequestConfirmCastleSiegeWaitingList") => {
                self.request(ScriptRequest::ConfirmCastleSiegeWaitingList { castle_id: int(0), clan_id: int(1), is_register: int(2) })
            }
            ("SiegeAPI", "RequestSetCastleSiegeTime") => self.request(ScriptRequest::SetCastleSiegeTime { castle_id: int(0), time_id: int(1) }),
            ("UIScript", "RequestDuelStart") => self.request(ScriptRequest::DuelStart { target_name: text(0), duel_type: int(1) }),
            ("UIScript", "RequestDuelAnswerStart") => {
                self.request(ScriptRequest::DuelAnswerStart { duel_type: int(0), option: int(1), answer: int(2) })
            }
            ("UIScript", "RequestDuelSurrender") => self.request(ScriptRequest::DuelSurrender),
            ("UIScript", "RequestClanMemberInfo") => self.request(ScriptRequest::ClanMemberInfo { kind: int(0), name: text(1) }),
            ("UIScript", "RequestClanGradeList") => self.request(ScriptRequest::ClanGradeList),
            ("UIScript", "RequestClanChangeGrade") => self.request(ScriptRequest::ClanChangeGrade { name: text(0), grade: int(1) }),
            ("UIScript", "RequestClanAssignPupil") => self.request(ScriptRequest::ClanAssignPupil { master: text(0), pupil: text(1) }),
            ("UIScript", "RequestClanDeletePupil") => self.request(ScriptRequest::ClanDeletePupil { master: text(0), pupil: text(1) }),
            ("UIScript", "RequestClanLeave") => self.request(ScriptRequest::ClanLeave { clan_name: text(0), clan_type: int(1) }),
            ("UIScript", "RequestClanExpelMember") => self.request(ScriptRequest::ClanExpelMember { clan_type: int(0), name: text(1) }),
            ("UIScript", "RequestClanAskJoin") => self.request(ScriptRequest::ClanAskJoin { id: int(0), clan_type: int(1) }),
            ("UIScript", "RequestClanDeclareWar") => self.request(ScriptRequest::ClanDeclareWar),
            ("UIScript", "RequestClanWithdrawWar") => self.request(ScriptRequest::ClanWithdrawWar),
            ("UIScript", "RequestClanWarList") => self.request(ScriptRequest::ClanWarList { page: int(0), state: int(1) }),
            ("UIScript", "RequestClanReorganizeMember") => self.request(ScriptRequest::ClanReorganizeMember {
                kind: int(0),
                member_name: text(1),
                clan_type: int(2),
                target_member_name: text(3),
            }),
            ("UIScript", "RequestClanChangeNickName") => self.request(ScriptRequest::ClanChangeNickName { name: text(0), nick_name: text(1) }),
            ("UIScript", "RequestClanRegisterCrest") => self.request(ScriptRequest::ClanRegisterCrest),
            ("UIScript", "RequestClanUnregisterCrest") => self.request(ScriptRequest::ClanUnregisterCrest),
            ("UIScript", "RequestClanRegisterEmblem") => self.request(ScriptRequest::ClanRegisterEmblem),
            ("UIScript", "RequestClanUnregisterEmblem") => self.request(ScriptRequest::ClanUnregisterEmblem),
            ("UIScript", "RequestSetSeed") => {
                let param = text(0);
                let manor_id = parse_int(&param, "ManorID").unwrap_or(0);
                self.request(ScriptRequest::SetSeed { manor_id, seeds: counted_pairs(&param, "SeedCnt", "SeedID", "TomorrowSalesVolume") })
            }
            ("UIScript", "RequestSetCrop") => {
                let param = text(0);
                let manor_id = parse_int(&param, "ManorID").unwrap_or(0);
                self.request(ScriptRequest::SetCrop { manor_id, crops: counted_pairs(&param, "CropCnt", "CropID", "BuyCnt") })
            }
            ("UIScript", "RequestProcureCropList") => {
                let param = text(0);
                self.request(ScriptRequest::ProcureCropList { crops: counted_triples(&param, "CropCnt", "CropServerID", "CropID", "ManorID") })
            }
            ("ActionAPI", "RequestActionList") => self.request(ScriptRequest::ActionList { kind: 0 }),
            ("ActionAPI", "RequestPetActionList") => self.request(ScriptRequest::ActionList { kind: 1 }),
            ("ActionAPI", "RequestSummonedActionList") => self.request(ScriptRequest::ActionList { kind: 2 }),
            (class, function) if function.starts_with("Request") && (class == "UIScript" || class.ends_with("API")) => {
                let _ = parse_string;
                self.request(ScriptRequest::Unmapped { class: class.to_string(), function: function.to_string() })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use l2_uscript::vm::Host;

    fn host() -> ScriptHost {
        ScriptHost { current: "InventoryWnd".into(), ..Default::default() }
    }

    fn requests(host: &ScriptHost) -> Vec<&ScriptRequest> {
        host.queue
            .iter()
            .filter_map(|item| match item {
                HostEvent::Request(request) => Some(request),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn requests_become_typed_queue_entries_with_the_script_arguments() {
        let mut host = host();
        host.call_native("UIScript", "RequestUseItem", None, &mut [Value::Int(268_480_131)]).unwrap();
        host.call_native("UIScript", "RequestDropItem", None, &mut [Value::Int(7), Value::Int(3), Value::vector(10.0, 20.0, -30.0)]).unwrap();
        host.call_native("UIScript", "RequestTradeDone", None, &mut [Value::Bool(true)]).unwrap();
        host.call_native("UIScript", "RequestRestartPoint", None, &mut [Value::Byte(2)]).unwrap();
        host.call_native("PetAPI", "RequestGetItemFromPet", None, &mut [Value::Int(5), Value::Int(1), Value::Bool(true)]).unwrap();
        host.call_native("SiegeAPI", "RequestJoinCastleSiege", None, &mut [Value::Int(3), Value::Int(1), Value::Int(1)]).unwrap();
        assert_eq!(
            requests(&host),
            [
                &ScriptRequest::UseItem { server_id: 268_480_131 },
                &ScriptRequest::DropItem { server_id: 7, num: 3, loc: super::super::Vector { x: 10.0, y: 20.0, z: -30.0 } },
                &ScriptRequest::TradeDone { done: true },
                &ScriptRequest::RestartPoint { kind: 2 },
                &ScriptRequest::GetItemFromPet { server_id: 5, num: 1, is_equip_item: true },
                &ScriptRequest::JoinCastleSiege { castle_id: 3, is_attacker: 1, is_register: 1 },
            ]
        );
        assert!(host.unhandled_natives.is_empty());
    }

    #[test]
    fn list_requests_parse_the_counted_param_string_the_way_ncparam_does() {
        let mut host = host();
        let sell = "merchant=1001 num=2 serverID0=70 classID0=57 count0=5 serverID1=71 classID1=58 count1=1 ";
        host.call_native("UIScript", "RequestSellItem", None, &mut [Value::Str(sell.into())]).unwrap();
        host.call_native("UIScript", "RequestBuyItem", None, &mut [Value::Str("merchant=9 num=1 classID0=1835 count0=20 ".into())]).unwrap();
        host.call_native("UIScript", "RequestWarehouseWithdraw", None, &mut [Value::Str("num=1 dbID0=268 count0=2 ".into())]).unwrap();
        host.call_native("UIScript", "RequestMultiSellChoose", None, &mut [Value::Str("ShopID=5 ItemID=12 ItemCount=1 RefineryOp1=0 ".into())]).unwrap();
        assert_eq!(
            requests(&host),
            [
                &ScriptRequest::SellItems { merchant: 1001, items: vec![(70, 57, 5), (71, 58, 1)] },
                &ScriptRequest::BuyItems { merchant: 9, items: vec![(1835, 20)] },
                &ScriptRequest::WarehouseWithdraw { items: vec![(268, 2)] },
                &ScriptRequest::MultiSellChoose { shop_id: 5, item_id: 12, item_count: 1, refinery_op1: 0 },
            ]
        );
    }

    #[test]
    fn sounds_options_chat_and_timers_follow_the_dll_tables() {
        let mut host = host();
        host.call_native("UIScript", "PlayConsoleSound", None, &mut [Value::Byte(13)]).unwrap();
        host.call_native("AudioAPI", "PlaySound", None, &mut [Value::Str("ItemSound.quest_finish".into())]).unwrap();
        host.call_native("UIScript", "PlaySound", None, &mut [Value::Str("None".into())]).unwrap();
        host.call_native("UIScript", "SetChatMessage", None, &mut [Value::Str("/target x".into())]).unwrap();
        host.call_native("UIScript", "ProcessChatMessage", None, &mut [Value::Str("oi".into()), Value::Int(0)]).unwrap();
        host.call_native("UIScript", "SetOptionBool", None, &mut [Value::Str("Game".into()), Value::Str("SystemMsgWnd".into()), Value::Bool(false)]).unwrap();
        host.call_native("UIScript", "SetINIInt", None, &mut [Value::Str("global".into()), Value::Str("normal".into()), Value::Int(1), Value::Str("chatfilter.ini".into())]).unwrap();
        let handle = Value::Handle("ChatWnd".into());
        host.call_native("WindowHandle", "SetTimer", Some(&handle), &mut [Value::Int(1), Value::Int(1000)]).unwrap();
        assert_eq!(
            host.queue,
            [
                HostEvent::Sound("InterfaceSound.inventory_open_01".into()),
                HostEvent::Sound("ItemSound.quest_finish".into()),
                HostEvent::ExecuteEvent(560, "String=\u{7f}/target x\u{7f}".into()),
                HostEvent::Request(ScriptRequest::ChatLine { message: "oi".into(), kind: 0 }),
                HostEvent::SetTimer("ChatWnd".into(), 1, 1000),
            ]
        );
        assert_eq!(host.option_writes, [("Game".to_string(), "SystemMsgWnd".to_string(), "false".to_string())]);
        assert_eq!(host.ini_values.get(&("chatfilter.ini".to_string(), "global".to_string(), "normal".to_string())), Some(&"1".to_string()));
    }

    #[test]
    fn unknown_requests_are_queued_as_unmapped_instead_of_silently_dropped() {
        let mut host = host();
        host.call_native("UIScript", "RequestAuthCardKeyLogin", None, &mut [Value::Int(1), Value::Str("x".into())]).unwrap();
        assert_eq!(requests(&host), [&ScriptRequest::Unmapped { class: "UIScript".into(), function: "RequestAuthCardKeyLogin".into() }]);
    }
}
