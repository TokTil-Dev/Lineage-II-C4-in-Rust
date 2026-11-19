use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use killer_world::session::{Session, SessionEvent};
use l2_dat::Chronicle;
use l2_net::transport::{GameConnection, authenticate_choosing_for};

fn target() -> (String, u16) {
    let host = std::env::var("KILLER_H5_HOST").unwrap_or_else(|_| "200.141.180.56".into());
    let port = std::env::var("KILLER_H5_LOGIN_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(2106);
    (host, port)
}

fn event_name(event: &SessionEvent) -> String {
    let text = format!("{event:?}");
    text.split(|c: char| !c.is_alphanumeric()).next().unwrap_or("").to_string()
}

#[test]
#[ignore]
fn live_high_five_enters_the_world_and_decodes_the_packets_with_account_from_environment() {
    let (Ok(user), Ok(password)) = (std::env::var("KILLER_TEST_USER"), std::env::var("KILLER_TEST_PASS")) else {
        eprintln!("KILLER_TEST_USER / KILLER_TEST_PASS não definidas: passo com conta ignorado");
        return;
    };
    let slot: i32 = std::env::var("KILLER_TEST_SLOT").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
    let (host, port) = target();
    let auth = authenticate_choosing_for(
        Chronicle::HighFive,
        &host,
        port,
        &user,
        &password,
        Duration::from_secs(15),
        |_| true,
        |servers| servers.first().map(|server| server.id),
        |_, _, _| None,
    )
    .unwrap();
    let mut game = GameConnection::connect(&auth.server.address(), Duration::from_secs(10)).unwrap();
    let mut session = Session::new_for(Chronicle::HighFive, user, auth);
    game.send(session.opening()).unwrap();
    let mut events: BTreeMap<String, usize> = BTreeMap::new();
    let mut unknown: BTreeMap<u8, (usize, usize)> = BTreeMap::new();
    let mut malformed: BTreeMap<u8, (usize, usize)> = BTreeMap::new();
    let mut selected_at: Option<Instant> = None;
    let mut got_user_info = false;
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(40) {
        if selected_at.is_some_and(|at| at.elapsed() > Duration::from_secs(12)) {
            break;
        }
        let Some(body) = game.receive_timeout(Duration::from_millis(200)).unwrap() else { continue };
        let (event, outgoing, crypt) = session.on_body(&body);
        if let Some(crypt) = crypt {
            game.enable_crypt(crypt);
        }
        for out in outgoing {
            game.send(out).unwrap();
        }
        let Some(event) = event else { continue };
        match &event {
            SessionEvent::Characters(characters) => {
                eprintln!("personagens: {}", characters.len());
                if selected_at.is_none() {
                    assert!(!characters.is_empty(), "a conta não tem personagem");
                    game.send(session.select(slot)).unwrap();
                    selected_at = Some(Instant::now());
                }
            }
            SessionEvent::UserInfo(head) | SessionEvent::UserInfoHighFive { head, .. } => {
                got_user_info = true;
                eprintln!("UserInfo: {} nível {} em ({}, {}, {})", head.name, head.level, head.x, head.y, head.z);
            }
            SessionEvent::Unknown { opcode, size } => {
                let entry = unknown.entry(*opcode).or_default();
                entry.0 += 1;
                entry.1 = *size;
            }
            SessionEvent::Malformed { opcode, size } => {
                let entry = malformed.entry(*opcode).or_default();
                entry.0 += 1;
                entry.1 = *size;
            }
            SessionEvent::Refused(reason) => panic!("sessão recusada: {reason}"),
            _ => {}
        }
        *events.entry(event_name(&event)).or_default() += 1;
    }
    for (name, count) in &events {
        eprintln!("evento {name}: {count}");
    }
    for (opcode, (count, size)) in &unknown {
        eprintln!("sem leitor {opcode:#04x}: {count} pacote(s), último com {size} bytes");
    }
    for (opcode, (count, size)) in &malformed {
        eprintln!("malformado {opcode:#04x}: {count} pacote(s), último com {size} bytes");
    }
    game.send(session.logout_packet()).unwrap();
    assert!(got_user_info, "o mundo não chegou ao UserInfo");
    assert!(malformed.is_empty(), "pacotes malformados: {malformed:?}");
}
