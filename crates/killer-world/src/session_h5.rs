use l2_net::game::high_five_world as hw;
use l2_net::game::packets as c4;
use l2_net::wire;

use crate::world::WorldEvent;

#[derive(Debug)]
pub enum HighFiveDecoded {
    World(WorldEvent),
    UserInfo(Box<c4::UserInfoHead>, f64, i32),
    Discarded(&'static str),
    Unknown,
}

pub fn decode_world_high_five(body: &[u8]) -> wire::Result<HighFiveDecoded> {
    let Some(&opcode) = body.first() else {
        return Ok(HighFiveDecoded::Unknown);
    };
    match opcode {
        hw::OP_USER_INFO => hw::read_user_info(body).map(|(head, extras)| HighFiveDecoded::UserInfo(Box::new(head), extras.exp_percent, extras.vitality)),
        hw::OP_CHAR_INFO => hw::read_char_info(body).map(|(info, _)| HighFiveDecoded::World(WorldEvent::CharInfo(Box::new(info)))),
        hw::OP_NPC_INFO => hw::read_npc_info(body).map(|(info, _)| HighFiveDecoded::World(WorldEvent::NpcInfo(info))),
        hw::OP_ITEM_LIST => hw::read_item_list(body).map(|(list, _)| HighFiveDecoded::World(WorldEvent::ItemList(list))),
        hw::OP_INVENTORY_UPDATE => {
            hw::read_inventory_update(body).map(|(update, _)| HighFiveDecoded::World(WorldEvent::InventoryUpdate(update)))
        }
        hw::OP_SKILL_LIST => hw::read_skill_list(body).map(|(list, _)| HighFiveDecoded::World(WorldEvent::SkillList(list))),
        hw::OP_STATUS_UPDATE => c4::read_status_update(body).map(|v| HighFiveDecoded::World(WorldEvent::StatusUpdate(v))),
        hw::OP_MOVE_TO_LOCATION => c4::read_move_to_location(body).map(|v| HighFiveDecoded::World(WorldEvent::MoveToLocation(v))),
        hw::OP_STOP_MOVE => c4::read_stop_move(body).map(|v| HighFiveDecoded::World(WorldEvent::StopMove(v))),
        hw::OP_CHANGE_MOVE_TYPE => {
            c4::read_change_move_type(body).map(|v| HighFiveDecoded::World(WorldEvent::ChangeMoveType(v)))
        }
        hw::OP_CHANGE_WAIT_TYPE => {
            c4::read_change_wait_type(body).map(|v| HighFiveDecoded::World(WorldEvent::ChangeWaitType(v)))
        }
        hw::OP_TELEPORT_TO_LOCATION => {
            c4::read_teleport_to_location(body).map(|v| HighFiveDecoded::World(WorldEvent::TeleportToLocation(v)))
        }
        hw::OP_CREATURE_SAY => hw::read_creature_say(body).map(|(say, _)| HighFiveDecoded::World(WorldEvent::Say2(say))),
        hw::OP_NPC_SAY => hw::read_npc_say(body).map(|(say, _)| HighFiveDecoded::World(WorldEvent::NpcSay(say))),
        hw::OP_SYSTEM_MESSAGE => hw::read_system_message(body).map(|v| HighFiveDecoded::World(WorldEvent::SystemMessage(v))),
        hw::OP_DELETE_OBJECT => c4::read_delete_object(body).map(|v| HighFiveDecoded::World(WorldEvent::DeleteObject(v))),
        hw::OP_SPAWN_ITEM => hw::read_spawn_item(body).map(|v| HighFiveDecoded::World(WorldEvent::SpawnItem(v))),
        hw::OP_DROP_ITEM => hw::read_drop_item(body).map(|v| HighFiveDecoded::World(WorldEvent::DropItem(v))),
        hw::OP_GET_ITEM => c4::read_get_item(body).map(|v| HighFiveDecoded::World(WorldEvent::GetItem(v))),
        hw::OP_TARGET_SELECTED => c4::read_target_selected(body).map(|v| HighFiveDecoded::World(WorldEvent::TargetSelected(v))),
        hw::OP_TARGET_UNSELECTED => {
            c4::read_target_unselected(body).map(|v| HighFiveDecoded::World(WorldEvent::TargetUnselected(v)))
        }
        hw::OP_MY_TARGET_SELECTED => {
            c4::read_my_target_selected(body).map(|v| HighFiveDecoded::World(WorldEvent::MyTargetSelected(v)))
        }
        hw::OP_ATTACK => hw::read_attack(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::Attack(v))),
        hw::OP_MAGIC_SKILL_USE => hw::read_magic_skill_use(body).map(|v| HighFiveDecoded::World(WorldEvent::MagicSkillUse(v))),
        hw::OP_MAGIC_SKILL_LAUNCHED => {
            c4::read_magic_skill_launched(body).map(|v| HighFiveDecoded::World(WorldEvent::MagicSkillLaunched(v)))
        }
        hw::OP_DIE => c4::read_die(body).map(|v| HighFiveDecoded::World(WorldEvent::Die(v))),
        hw::OP_REVIVE => c4::read_revive(body).map(|v| HighFiveDecoded::World(WorldEvent::Revive(v))),
        hw::OP_ACTION_FAILED => c4::read_action_failed(body).map(|v| HighFiveDecoded::World(WorldEvent::ActionFailed(v))),
        hw::OP_VALIDATE_LOCATION => {
            c4::read_validate_location(body).map(|v| HighFiveDecoded::World(WorldEvent::ValidateLocation(v)))
        }
        hw::OP_SOCIAL_ACTION => c4::read_social_action(body).map(|v| HighFiveDecoded::World(WorldEvent::SocialAction(v))),
        hw::OP_AUTO_ATTACK_START => {
            c4::read_auto_attack_start(body).map(|v| HighFiveDecoded::World(WorldEvent::AutoAttackStart(v)))
        }
        hw::OP_AUTO_ATTACK_STOP => c4::read_auto_attack_stop(body).map(|v| HighFiveDecoded::World(WorldEvent::AutoAttackStop(v))),
        hw::OP_ABNORMAL_STATUS_UPDATE => {
            hw::read_abnormal_status_update(body).map(|v| HighFiveDecoded::World(WorldEvent::MagicEffectIcons(v)))
        }
        hw::OP_QUEST_LIST => c4::read_quest_list(body).map(|v| HighFiveDecoded::World(WorldEvent::QuestList(v))),
        hw::OP_HENNA_INFO => c4::read_henna_info(body).map(|v| HighFiveDecoded::World(WorldEvent::HennaInfo(v))),
        hw::OP_SHORT_CUT_INIT => hw::read_short_cut_init(body).map(|v| HighFiveDecoded::World(WorldEvent::ShortCutInit(v))),
        hw::OP_SHORT_CUT_REGISTER => {
            hw::read_short_cut_register(body).map(|v| HighFiveDecoded::World(WorldEvent::ShortCutRegister(v)))
        }
        hw::OP_SHORT_CUT_DELETE => c4::read_short_cut_delete(body).map(|v| HighFiveDecoded::World(WorldEvent::ShortCutDelete(v))),
        hw::OP_SKILL_COOL_TIME => c4::read_skill_cool_time(body).map(|v| HighFiveDecoded::World(WorldEvent::SkillCoolTime(v))),
        hw::OP_ETC_STATUS_UPDATE => {
            hw::read_etc_status_update(body).map(|v| HighFiveDecoded::World(WorldEvent::EtcStatusUpdate(v)))
        }
        hw::OP_NPC_HTML_MESSAGE => {
            c4::read_npc_html_message(body).map(|v| HighFiveDecoded::World(WorldEvent::NpcHtmlMessage(v)))
        }
        hw::OP_EX_SERVER => match c4::read_ex_subopcode(body)? {
            hw::EX_STORAGE_MAX_COUNT => {
                hw::read_storage_max_count(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::StorageMaxCount(v)))
            }
            hw::EX_BASIC_ACTION_LIST => hw::read_basic_action_list(body).map(|_| HighFiveDecoded::Discarded("ExBasicActionList")),
            hw::EX_ROTATION => hw::read_rotation(body).map(|_| HighFiveDecoded::Discarded("ExRotation")),
            hw::EX_VOTE_SYSTEM_INFO => hw::read_vote_system_info(body).map(|_| HighFiveDecoded::Discarded("ExVoteSystemInfo")),
            hw::EX_UI_SETTING => hw::read_ui_setting(body).map(|_| HighFiveDecoded::Discarded("ExUISetting")),
            _ => Ok(HighFiveDecoded::Unknown),
        },
        hw::OP_TRADE_START => hw::read_trade_start(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::TradeStart(v))),
        hw::OP_TRADE_OWN_ADD => hw::read_trade_own_add(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::TradeOwnAdd(v))),
        hw::OP_TRADE_OTHER_ADD => {
            hw::read_trade_other_add(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::TradeOtherAdd(v)))
        }
        hw::OP_TRADE_DONE => c4::read_trade_done(body).map(|v| HighFiveDecoded::World(WorldEvent::TradeDone(v))),
        hw::OP_SEND_TRADE_REQUEST => c4::read_trade_request(body).map(|v| HighFiveDecoded::World(WorldEvent::TradeRequest(v))),
        hw::OP_ASK_JOIN_PARTY => c4::read_ask_join_party(body).map(|v| HighFiveDecoded::World(WorldEvent::AskJoinParty(v))),
        hw::OP_JOIN_PARTY => c4::read_join_party(body).map(|v| HighFiveDecoded::World(WorldEvent::JoinParty(v))),
        hw::OP_PARTY_SMALL_WINDOW_ALL => {
            hw::read_party_small_window_all(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::PartySmallWindowAll(v)))
        }
        hw::OP_PARTY_SMALL_WINDOW_ADD => {
            hw::read_party_small_window_add(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::PartySmallWindowAdd(v)))
        }
        hw::OP_PARTY_SMALL_WINDOW_DELETE_ALL => {
            c4::read_party_small_window_delete_all(body).map(|v| HighFiveDecoded::World(WorldEvent::PartySmallWindowDeleteAll(v)))
        }
        hw::OP_PARTY_SMALL_WINDOW_DELETE => {
            c4::read_party_small_window_delete(body).map(|v| HighFiveDecoded::World(WorldEvent::PartySmallWindowDelete(v)))
        }
        hw::OP_PARTY_SMALL_WINDOW_UPDATE => {
            c4::read_party_small_window_update(body).map(|v| HighFiveDecoded::World(WorldEvent::PartySmallWindowUpdate(v)))
        }
        hw::OP_PARTY_MEMBER_POSITION => {
            c4::read_party_member_position(body).map(|v| HighFiveDecoded::World(WorldEvent::PartyMemberPosition(v)))
        }
        hw::OP_ASK_JOIN_PLEDGE => c4::read_ask_join_pledge(body).map(|v| HighFiveDecoded::World(WorldEvent::AskJoinPledge(v))),
        hw::OP_JOIN_PLEDGE => c4::read_join_pledge(body).map(|v| HighFiveDecoded::World(WorldEvent::JoinPledge(v))),
        hw::OP_PLEDGE_INFO => c4::read_pledge_info(body).map(|v| HighFiveDecoded::World(WorldEvent::PledgeInfo(v))),
        hw::OP_PLEDGE_CREST => c4::read_pledge_crest(body).map(|v| HighFiveDecoded::World(WorldEvent::PledgeCrest(v))),
        hw::OP_ALLY_CREST => c4::read_alliance_crest(body).map(|v| HighFiveDecoded::World(WorldEvent::AllianceCrest(v))),
        hw::OP_PLEDGE_STATUS_CHANGED => {
            c4::read_pledge_status_changed(body).map(|v| HighFiveDecoded::World(WorldEvent::PledgeStatusChanged(v)))
        }
        hw::OP_PLEDGE_SHOW_MEMBER_LIST_ALL => {
            hw::read_pledge_show_member_list_all(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::PledgeShowMemberListAll(v)))
        }
        hw::OP_PLEDGE_SHOW_MEMBER_LIST_ADD => {
            hw::read_pledge_show_member_list_add(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::PledgeShowMemberListAdd(v)))
        }
        hw::OP_PLEDGE_SHOW_MEMBER_LIST_UPDATE => {
            hw::read_pledge_show_member_list_update(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::PledgeShowMemberListUpdate(v)))
        }
        hw::OP_PLEDGE_SHOW_MEMBER_LIST_DELETE => {
            c4::read_pledge_show_member_list_delete(body).map(|v| HighFiveDecoded::World(WorldEvent::PledgeShowMemberListDelete(v)))
        }
        hw::OP_PLEDGE_SHOW_MEMBER_LIST_DELETE_ALL => c4::read_pledge_show_member_list_delete_all(body)
            .map(|v| HighFiveDecoded::World(WorldEvent::PledgeShowMemberListDeleteAll(v))),
        hw::OP_PLEDGE_SHOW_INFO_UPDATE => {
            hw::read_pledge_show_info_update(body).map(|(v, _)| HighFiveDecoded::World(WorldEvent::PledgeShowInfoUpdate(v)))
        }
        hw::OP_FRIEND_LIST => c4::read_friend_list(body).map(|v| HighFiveDecoded::World(WorldEvent::FriendList(v))),
        hw::OP_FRIEND_ADD_REQUEST => {
            c4::read_friend_add_request(body).map(|v| HighFiveDecoded::World(WorldEvent::FriendAddRequest(v)))
        }
        hw::OP_FRIEND_STATUS => c4::read_friend_status_noop(body).map(|v| HighFiveDecoded::World(WorldEvent::FriendStatusNoop(v))),
        hw::OP_L2_FRIEND_SAY => c4::read_friend_recv_msg(body).map(|v| HighFiveDecoded::World(WorldEvent::FriendRecvMsg(v))),
        hw::OP_FRIEND_PACKET => match hw::read_friend_packet(body)? {
            hw::FriendPacketHighFive::Added => Ok(HighFiveDecoded::World(WorldEvent::FriendAdd(c4::FriendNoop))),
            hw::FriendPacketHighFive::Removed => Ok(HighFiveDecoded::World(WorldEvent::FriendRemove(c4::FriendNoop))),
            hw::FriendPacketHighFive::Unknown { .. } => Ok(HighFiveDecoded::Unknown),
        },
        hw::OP_RELATION_CHANGED => match hw::read_relation_changed(body) {
            Ok(mut entries) if !entries.is_empty() => {
                Ok(HighFiveDecoded::World(WorldEvent::RelationChanged(entries.remove(0))))
            }
            Ok(_) => Ok(HighFiveDecoded::Discarded("RelationChanged")),
            Err(why) => Err(why),
        },
        hw::OP_NET_PING => c4::read_net_ping_response(body).map(|v| HighFiveDecoded::World(WorldEvent::NetPingResponse(v))),
        _ => Ok(HighFiveDecoded::Unknown),
    }
}
