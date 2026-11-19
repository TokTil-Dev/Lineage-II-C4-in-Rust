use l2_dat::Chronicle;
use l2_net::login::packets::{OP_GAME_GUARD_AUTH, read_game_guard_auth, read_init_modern, request_auth_game_guard};
use l2_net::login::LoginCipher;
use l2_net::transport::{authenticate_choosing_for, frame};
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

fn target() -> (String, u16) {
    let host = std::env::var("KILLER_H5_HOST").unwrap_or_else(|_| "200.141.180.56".into());
    let port = std::env::var("KILLER_H5_LOGIN_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(2106);
    (host, port)
}

#[test]
#[ignore]
fn live_init_and_game_guard_without_credentials() {
    let (host, port) = target();
    let addr = (host.as_str(), port).to_socket_addrs().unwrap().next().unwrap();
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10)).unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    let raw = frame::read(&mut stream).unwrap();
    eprintln!("Init: {} bytes cifrados", raw.len());
    let body = LoginCipher::init().open_init(&raw).unwrap();
    let init = read_init_modern(&body).expect("Init H5 legível");
    eprintln!("session id {:#x}, protocolo {:#x}, módulo[0..4] {:02x?}", init.session_id, init.protocol, &init.modulus[..4]);
    eprintln!("GG {:02x?}", init.guard);
    assert_eq!(init.protocol, 0xc621);
    assert!(init.modulus[0] & 0x80 != 0 && init.modulus[127] & 1 == 1);
    let session = LoginCipher::session(&init.blowfish_key);
    let request = request_auth_game_guard(init.session_id, &init.guard);
    stream.write_all(&frame::encode(&session.seal(&request))).unwrap();
    let reply = session.open(&frame::read(&mut stream).unwrap()).unwrap();
    eprintln!("resposta: op {:#04x}, {} bytes", reply[0], reply.len());
    assert_eq!(reply[0], OP_GAME_GUARD_AUTH);
    assert_eq!(read_game_guard_auth(&reply).unwrap(), init.session_id);
}

#[test]
#[ignore]
fn live_full_login_with_account_from_environment() {
    let (Ok(user), Ok(password)) = (std::env::var("KILLER_TEST_USER"), std::env::var("KILLER_TEST_PASS")) else {
        eprintln!("KILLER_TEST_USER / KILLER_TEST_PASS não definidas: passo com conta ignorado");
        return;
    };
    let (host, port) = target();
    let auth = authenticate_choosing_for(
        Chronicle::HighFive,
        &host,
        port,
        &user,
        &password,
        Duration::from_secs(15),
        |notice| {
            eprintln!("LoginOk: flags {:#x}", notice.flags);
            true
        },
        |servers| {
            for server in servers {
                eprintln!(
                    "servidor {} {} jogadores {}/{} online {} extra {:?}",
                    server.id,
                    server.address(),
                    server.players,
                    server.max_players,
                    server.online,
                    server.extra
                );
            }
            servers.first().map(|server| server.id)
        },
        |_, _, _| None,
    )
    .unwrap();
    eprintln!("PlayOk: servidor {} em {}", auth.server.id, auth.server.address());
}

fn game_target() -> String {
    let host = std::env::var("KILLER_H5_HOST").unwrap_or_else(|_| "200.141.180.56".into());
    let port = std::env::var("KILLER_H5_GAME_PORT").unwrap_or_else(|_| "7777".into());
    format!("{host}:{port}")
}

#[test]
#[ignore]
fn live_game_protocol_version_is_answered_with_the_key_packet_without_credentials() {
    use l2_net::game::{GameCrypt, high_five};
    use l2_net::transport::GameConnection;
    let mut game = GameConnection::connect(&game_target(), Duration::from_secs(10)).unwrap();
    game.send(high_five::protocol_version(high_five::PROTOCOL_VERSION)).unwrap();
    game.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    let key_packet = game.receive().unwrap();
    eprintln!("KeyPacket: op {:#04x}, {} bytes, flag {}", key_packet[0], key_packet.len(), key_packet[1]);
    assert_eq!(key_packet.len(), 23);
    assert_eq!(key_packet[0], 0x2e);
    assert_eq!(key_packet[1], 1);
    assert!(GameCrypt::from_key_packet_high_five(&key_packet).is_ok());
}

#[test]
#[ignore]
fn live_game_reaches_the_character_selection_info_with_account_from_environment() {
    use l2_net::game::{GameCrypt, high_five};
    use l2_net::transport::GameConnection;
    let (Ok(user), Ok(password)) = (std::env::var("KILLER_TEST_USER"), std::env::var("KILLER_TEST_PASS")) else {
        eprintln!("KILLER_TEST_USER / KILLER_TEST_PASS não definidas: passo com conta ignorado");
        return;
    };
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
    game.set_read_timeout(Some(Duration::from_secs(15))).unwrap();
    game.send(high_five::protocol_version(high_five::PROTOCOL_VERSION)).unwrap();
    let key_packet = game.receive().unwrap();
    game.enable_crypt(GameCrypt::from_key_packet_high_five(&key_packet).unwrap());
    game.send(high_five::auth_login(&user, &auth, high_five::LANGUAGE_ENGLISH)).unwrap();
    let body = game.receive().unwrap();
    eprintln!("resposta ao AuthLogin: op {:#04x}, {} bytes", body[0], body.len());
    assert_eq!(body[0], high_five::OP_CHAR_SELECT_INFO);
    let info = high_five::read_char_select_info(&body).unwrap();
    eprintln!("slots {}, personagens {}", info.max_slots, info.characters.len());
    for character in &info.characters {
        eprintln!(
            "{} nível {} classe {} exp {} ({:.2}%) vitalidade {} apagar em {}s",
            character.entry.name,
            character.entry.level,
            character.entry.class_id,
            character.entry.exp,
            character.exp_percent,
            character.vitality,
            character.entry.delete_days
        );
    }
    game.send(high_five::logout()).unwrap();
}
