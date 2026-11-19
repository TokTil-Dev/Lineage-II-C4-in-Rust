use crate::game::GameCrypt;
use crate::login::{
    Authorization, CipherError, GameServer, LoginCipher, LoginError, LoginFlow, LoginNotice, LoginPhase, LoginStep,
    init_is_plaintext,
};
use l2_dat::Chronicle;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use thiserror::Error;

pub mod frame {
    use super::*;

    pub fn encode(body: &[u8]) -> Vec<u8> {
        let len = (body.len() + 2) as u16;
        let mut out = Vec::with_capacity(body.len() + 2);
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(body);
        out
    }

    pub fn read(reader: &mut impl Read) -> io::Result<Vec<u8>> {
        let mut header = [0u8; 2];
        reader.read_exact(&mut header)?;
        let len = u16::from_le_bytes(header) as usize;
        if len < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "quadro menor que o cabeçalho"));
        }
        let mut body = vec![0u8; len - 2];
        reader.read_exact(&mut body)?;
        Ok(body)
    }
}

#[derive(Debug, Error)]
pub enum TransportError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Login(#[from] LoginError),
    #[error(transparent)]
    Cipher(#[from] CipherError),
    #[error("conexão encerrada: {0}")]
    Closed(&'static str),
}

fn checked_frame(body: &[u8]) -> io::Result<Vec<u8>> {
    if body.len() > 65533 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "corpo maior que 65533 bytes"));
    }
    Ok(frame::encode(body))
}

fn connect(addr: &str, timeout: Duration) -> io::Result<TcpStream> {
    let resolved = addr
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "endereço não resolvido"))?;
    let stream = TcpStream::connect_timeout(&resolved, timeout)?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    stream.set_nodelay(true)?;
    Ok(stream)
}

pub fn authenticate(
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    timeout: Duration,
) -> Result<Authorization, TransportError> {
    authenticate_for(Chronicle::C4, host, port, user, password, timeout)
}

pub fn authenticate_for(
    chronicle: Chronicle,
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    timeout: Duration,
) -> Result<Authorization, TransportError> {
    let flow = LoginFlow::for_chronicle(chronicle, user, password, false)?;
    run_login(host, port, flow, timeout, |_| true, |_| None, |_, _, _| None)
}

#[allow(clippy::too_many_arguments)]
pub fn authenticate_choosing(
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    timeout: Duration,
    notice: impl FnMut(&LoginNotice) -> bool,
    choose: impl FnMut(&[GameServer]) -> Option<u8>,
    card: impl FnMut(i32, i8, bool) -> Option<i32>,
) -> Result<Authorization, TransportError> {
    authenticate_choosing_for(Chronicle::C4, host, port, user, password, timeout, notice, choose, card)
}

#[allow(clippy::too_many_arguments)]
pub fn authenticate_choosing_for(
    chronicle: Chronicle,
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    timeout: Duration,
    notice: impl FnMut(&LoginNotice) -> bool,
    choose: impl FnMut(&[GameServer]) -> Option<u8>,
    card: impl FnMut(i32, i8, bool) -> Option<i32>,
) -> Result<Authorization, TransportError> {
    let flow = LoginFlow::for_chronicle(chronicle, user, password, true)?;
    run_login(host, port, flow, timeout, notice, choose, card)
}

fn run_login(
    host: &str,
    port: u16,
    mut flow: LoginFlow,
    timeout: Duration,
    mut notice: impl FnMut(&LoginNotice) -> bool,
    mut choose: impl FnMut(&[GameServer]) -> Option<u8>,
    mut card: impl FnMut(i32, i8, bool) -> Option<i32>,
) -> Result<Authorization, TransportError> {
    let mut cipher = if flow.is_modern() { LoginCipher::init() } else { LoginCipher::c4() };
    let mut stream = connect(&format!("{host}:{port}"), timeout)?;
    let mut init_pending = true;
    loop {
        let raw = frame::read(&mut stream).map_err(|e| match e.kind() {
            io::ErrorKind::UnexpectedEof => TransportError::Closed(match flow.phase() {
                LoginPhase::Init => "o login server fechou a conexão antes do Init",
                LoginPhase::GameGuard => "o login server fechou a conexão ao receber o AuthGameGuard",
                LoginPhase::LoginOk => {
                    "o login server fechou a conexão ao receber as credenciais; conta já online ou pedido malformado"
                }
                LoginPhase::Notice | LoginPhase::ServerList => "o login server fechou a conexão antes da lista de servidores",
                LoginPhase::PlayOk | LoginPhase::Complete => {
                    "o login server fechou a conexão antes de liberar o game server"
                }
            }),
            _ => TransportError::Io(e),
        })?;
        let body = if flow.is_modern() && init_pending {
            init_pending = false;
            cipher.open_init(&raw)?
        } else if init_pending && init_is_plaintext(&raw) {
            init_pending = false;
            raw
        } else {
            cipher.open(&raw)?
        };
        let step = flow.step(&body)?;
        if let Some(key) = flow.take_session_key() {
            cipher = LoginCipher::session(&key);
        }
        match step {
            LoginStep::Send(mut reply) => {
                stream.write_all(&checked_frame(&cipher.seal(&reply))?)?;
                reply.fill(0);
            }
            LoginStep::Notice(info) => {
                if !notice(&info) {
                    return Err(TransportError::Closed("termos recusados"));
                }
                let reply = flow.accept_notice()?;
                stream.write_all(&checked_frame(&cipher.seal(&reply))?)?;
            }
            LoginStep::Choose(servers) => {
                let Some(id) = choose(&servers) else {
                    return Err(TransportError::Closed("escolha de servidor cancelada"));
                };
                let reply = flow.choose(servers, id)?;
                stream.write_all(&checked_frame(&cipher.seal(&reply))?)?;
            }
            LoginStep::CardCheck { card_id, position, retry } => {
                let Some(value) = card(card_id, position, retry) else {
                    return Err(TransportError::Closed("cartão de segurança cancelado"));
                };
                let reply = flow.card_answer(card_id, value)?;
                stream.write_all(&checked_frame(&cipher.seal(&reply))?)?;
            }
            LoginStep::Done(auth) => return Ok(auth),
        }
    }
}

pub struct GameConnection {
    stream: TcpStream,
    crypt: Option<GameCrypt>,
}

impl GameConnection {
    pub fn connect(addr: &str, timeout: Duration) -> io::Result<Self> {
        Ok(Self { stream: connect(addr, timeout)?, crypt: None })
    }

    pub fn enable_crypt(&mut self, crypt: GameCrypt) {
        self.crypt = Some(crypt);
    }

    pub fn send(&mut self, mut body: Vec<u8>) -> io::Result<()> {
        if let Some(crypt) = &mut self.crypt {
            crypt.encrypt(&mut body);
        }
        self.stream.write_all(&checked_frame(&body)?)
    }

    pub fn receive(&mut self) -> io::Result<Vec<u8>> {
        let mut body = frame::read(&mut self.stream)?;
        if let Some(crypt) = &mut self.crypt {
            crypt.decrypt(&mut body);
        }
        Ok(body)
    }

    pub fn set_read_timeout(&mut self, d: Option<Duration>) -> io::Result<()> {
        self.stream.set_read_timeout(d)
    }

    pub fn receive_timeout(&mut self, d: Duration) -> io::Result<Option<Vec<u8>>> {
        self.stream.set_read_timeout(Some(d))?;
        let mut probe = [0u8; 1];
        let peeked = self.stream.peek(&mut probe);
        self.stream.set_read_timeout(None)?;
        match peeked {
            Ok(_) => Ok(Some(self.receive()?)),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn frame_roundtrip() {
        let framed = frame::encode(&[9, 8, 7]);
        assert_eq!(framed, [5, 0, 9, 8, 7]);
        let mut cursor = Cursor::new(framed);
        assert_eq!(frame::read(&mut cursor).unwrap(), [9, 8, 7]);
    }

    #[test]
    fn frame_rejects_lengths_below_header() {
        let mut cursor = Cursor::new(vec![1u8, 0]);
        assert!(frame::read(&mut cursor).is_err());
    }

    #[test]
    fn game_connection_runs_the_high_five_opening_against_a_fake_server() {
        use crate::game::high_five;
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let opening = frame::read(&mut socket).unwrap();
            assert_eq!(opening, high_five::protocol_version(high_five::PROTOCOL_VERSION));
            let mut key_packet = vec![0x2e, 1, 9, 8, 7, 6, 5, 4, 3, 2];
            key_packet.extend_from_slice(&[1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0]);
            socket.write_all(&frame::encode(&key_packet)).unwrap();
            let mut crypt = GameCrypt::from_key_packet_high_five(&key_packet).unwrap();
            let mut first = frame::read(&mut socket).unwrap();
            crypt.decrypt(&mut first);
            assert_eq!(first[0], 0x2b);
            let mut second = frame::read(&mut socket).unwrap();
            crypt.decrypt(&mut second);
            assert_eq!(second, high_five::request_character_selection());
            let mut reply = vec![0x09, 0, 0, 0, 0, 7, 0, 0, 0, 0];
            crypt.encrypt(&mut reply);
            socket.write_all(&frame::encode(&reply)).unwrap();
        });
        let mut game = GameConnection::connect(&addr.to_string(), Duration::from_secs(5)).unwrap();
        game.send(high_five::protocol_version(high_five::PROTOCOL_VERSION)).unwrap();
        let key_packet = game.receive().unwrap();
        game.enable_crypt(GameCrypt::from_key_packet_high_five(&key_packet).unwrap());
        game.send(vec![0x2b, 0, 0]).unwrap();
        game.send(high_five::request_character_selection()).unwrap();
        let list = game.receive().unwrap();
        assert_eq!(high_five::read_char_select_info(&list).unwrap().max_slots, 7);
        server.join().unwrap();
    }

    #[test]
    fn game_connection_encrypts_after_key() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let first = frame::read(&mut socket).unwrap();
            let key_packet = vec![0u8, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0];
            socket.write_all(&frame::encode(&key_packet)).unwrap();
            let mut second = frame::read(&mut socket).unwrap();
            GameCrypt::from_key_packet(&key_packet).unwrap().decrypt(&mut second);
            (first, second)
        });
        let mut conn = GameConnection::connect(&addr.to_string(), Duration::from_secs(2)).unwrap();
        conn.send(vec![0, 1, 2]).unwrap();
        let key_packet = conn.receive().unwrap();
        conn.enable_crypt(GameCrypt::from_key_packet(&key_packet).unwrap());
        conn.send(vec![8, 9]).unwrap();
        let (first, second) = server.join().unwrap();
        assert_eq!(first, [0, 1, 2]);
        assert_eq!(second, [8, 9]);
    }

    #[test]
    fn authenticate_runs_the_full_flow_against_a_fake_login_server() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            let cipher = LoginCipher::c4();
            let (mut socket, _) = listener.accept().unwrap();
            let mut init = vec![0u8];
            init.extend_from_slice(&[0u8; 8]);
            socket.write_all(&frame::encode(&init)).unwrap();
            let auth = cipher.open(&frame::read(&mut socket).unwrap()).unwrap();
            assert_eq!(auth[0], 0);
            let mut ok = vec![3u8];
            ok.extend_from_slice(&[1, 0, 0, 0, 2, 0, 0, 0]);
            socket.write_all(&frame::encode(&cipher.seal(&ok))).unwrap();
            let list_req = cipher.open(&frame::read(&mut socket).unwrap()).unwrap();
            assert_eq!(list_req[0], 5);
            let mut list = vec![4u8, 1, 1, 1, 127, 0, 0, 1];
            list.extend_from_slice(&7777u32.to_le_bytes());
            list.extend_from_slice(&[0, 1, 3, 0, 9, 0, 1, 0, 0, 0, 0, 0]);
            socket.write_all(&frame::encode(&cipher.seal(&list))).unwrap();
            let play_req = cipher.open(&frame::read(&mut socket).unwrap()).unwrap();
            assert_eq!(play_req[0], 2);
            let mut playok = vec![7u8];
            playok.extend_from_slice(&[3, 0, 0, 0, 4, 0, 0, 0]);
            socket.write_all(&frame::encode(&cipher.seal(&playok))).unwrap();
        });
        let auth = authenticate("127.0.0.1", addr.port(), "conta", "senha", Duration::from_secs(2)).unwrap();
        assert_eq!(auth.login_key, [1, 2]);
        assert_eq!(auth.play_key, [3, 4]);
        assert_eq!(auth.server.address(), "127.0.0.1:7777");
    }

    #[test]
    fn authenticate_choosing_hands_the_list_to_the_caller_and_logs_into_its_pick() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            let cipher = LoginCipher::c4();
            let (mut socket, _) = listener.accept().unwrap();
            let mut init = vec![0u8];
            init.extend_from_slice(&[0u8; 8]);
            socket.write_all(&frame::encode(&init)).unwrap();
            cipher.open(&frame::read(&mut socket).unwrap()).unwrap();
            let mut ok = vec![3u8];
            ok.extend_from_slice(&[1, 0, 0, 0, 2, 0, 0, 0]);
            socket.write_all(&frame::encode(&cipher.seal(&ok))).unwrap();
            cipher.open(&frame::read(&mut socket).unwrap()).unwrap();
            let mut list = vec![4u8, 2, 1];
            for id in [1u8, 2] {
                list.extend_from_slice(&[id, 127, 0, 0, id]);
                list.extend_from_slice(&7777u32.to_le_bytes());
                list.extend_from_slice(&[0, 1, 3, 0, 9, 0, 1, 0, 0, 0, 0, 0]);
            }
            socket.write_all(&frame::encode(&cipher.seal(&list))).unwrap();
            let play_req = cipher.open(&frame::read(&mut socket).unwrap()).unwrap();
            assert_eq!(play_req[0], 2);
            assert_eq!(play_req[9], 2);
            let mut playok = vec![7u8];
            playok.extend_from_slice(&[3, 0, 0, 0, 4, 0, 0, 0]);
            socket.write_all(&frame::encode(&cipher.seal(&playok))).unwrap();
        });
        let mut offered = Vec::new();
        let mut notices = 0;
        let auth =
            authenticate_choosing(
                "127.0.0.1",
                addr.port(),
                "conta",
                "senha",
                Duration::from_secs(2),
                |_| {
                    notices += 1;
                    true
                },
                |servers| {
                    offered = servers.iter().map(|s| s.id).collect();
                    Some(2)
                },
                |_, _, _| None,
            )
            .unwrap();
        assert_eq!(offered, [1, 2]);
        assert_eq!(notices, 1);
        assert_eq!(auth.server.address(), "127.0.0.2:7777");
    }

    #[test]
    fn send_rejects_body_over_the_frame_limit() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            let _ = listener.accept();
        });
        let mut conn = GameConnection::connect(&addr.to_string(), Duration::from_secs(2)).unwrap();
        let err = conn.send(vec![0u8; 65534]).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn receive_timeout_returns_none_then_the_whole_frame() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            thread::sleep(Duration::from_millis(300));
            let framed = frame::encode(&[1, 2, 3]);
            socket.write_all(&framed[..2]).unwrap();
            thread::sleep(Duration::from_millis(300));
            socket.write_all(&framed[2..]).unwrap();
        });
        let mut conn = GameConnection::connect(&addr.to_string(), Duration::from_secs(2)).unwrap();
        assert!(conn.receive_timeout(Duration::from_millis(200)).unwrap().is_none());
        let body = conn.receive_timeout(Duration::from_millis(200)).unwrap().unwrap();
        assert_eq!(body, [1, 2, 3]);
        server.join().unwrap();
    }

    #[test]
    fn eof_while_sending_credentials_names_the_login_phase() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let mut init = vec![0u8];
            init.extend_from_slice(&[0u8; 8]);
            socket.write_all(&frame::encode(&init)).unwrap();
            let _ = frame::read(&mut socket).unwrap();
        });
        let err = authenticate("127.0.0.1", addr.port(), "conta", "senha", Duration::from_secs(2)).unwrap_err();
        match err {
            TransportError::Closed(msg) => assert!(msg.contains("credenciais")),
            other => panic!("esperava TransportError::Closed, veio {other}"),
        }
    }

    fn modern_modulus() -> [u8; 128] {
        let mut modulus = [0u8; 128];
        for (i, b) in modulus.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(53).wrapping_add(7);
        }
        modulus[0] = 0xc3;
        modulus
    }

    fn modern_server_list() -> Vec<u8> {
        let mut list = vec![4u8, 2, 2];
        for id in [1u8, 2] {
            list.extend_from_slice(&[id, 127, 0, 0, id]);
            list.extend_from_slice(&7777u32.to_le_bytes());
            list.extend_from_slice(&[0, 1, 3, 0, 9, 0, 1, 0, 0, 0, 0, 0]);
        }
        list.extend_from_slice(&0u16.to_le_bytes());
        list.extend_from_slice(&[1, 2, 1, 1]);
        list.extend_from_slice(&3600u32.to_le_bytes());
        list
    }

    #[test]
    fn authenticate_for_high_five_runs_init_game_guard_rsa_login_and_server_list() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let session_key = [0x42u8; 16];
            let guard = [0x4e, 0x95, 0xdd, 0x29, 0xfc, 0x9c, 0xc3, 0x77, 0x20, 0xb6, 0xad, 0x97, 0xf7, 0xe0, 0xbd, 0x07];
            let modulus = modern_modulus();
            let mut scrambled = modulus;
            crate::login::rsa::scramble_modulus(&mut scrambled);
            let mut init = vec![0u8];
            init.extend_from_slice(&0x0102_0304u32.to_le_bytes());
            init.extend_from_slice(&0xc621u32.to_le_bytes());
            init.extend_from_slice(&scrambled);
            init.extend_from_slice(&guard);
            init.extend_from_slice(&session_key);
            init.push(0);
            let (mut socket, _) = listener.accept().unwrap();
            socket.write_all(&frame::encode(&LoginCipher::init().seal_init(&init, 0x5566_7788))).unwrap();
            let cipher = LoginCipher::session(&session_key);
            let gg = cipher.open(&frame::read(&mut socket).unwrap()).unwrap();
            assert_eq!(gg[0], 7);
            assert_eq!(&gg[1..5], &0x0102_0304u32.to_le_bytes());
            assert_eq!(&gg[5..21], &guard);
            let mut ack = vec![0x0bu8];
            ack.extend_from_slice(&0x0102_0304u32.to_le_bytes());
            ack.extend_from_slice(&[0u8; 16]);
            socket.write_all(&frame::encode(&cipher.seal(&ack))).unwrap();
            let login = cipher.open(&frame::read(&mut socket).unwrap()).unwrap();
            assert_eq!(login[0], 0);
            let block = crate::login::rsa::credentials_block(b"conta", b"senha").unwrap();
            let expected = crate::login::rsa::encrypt_no_padding(&modulus, &block).unwrap();
            assert_eq!(&login[1..129], &expected[..]);
            assert_eq!(&login[129..133], &0x0102_0304u32.to_le_bytes());
            socket.write_all(&frame::encode(&cipher.seal(&modern_server_list()))).unwrap();
            let play = cipher.open(&frame::read(&mut socket).unwrap()).unwrap();
            assert_eq!(&play[..10], &[2, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
            let mut playok = vec![7u8];
            playok.extend_from_slice(&[3, 0, 0, 0, 4, 0, 0, 0]);
            socket.write_all(&frame::encode(&cipher.seal(&playok))).unwrap();
        });
        let auth =
            authenticate_for(Chronicle::HighFive, "127.0.0.1", addr.port(), "conta", "senha", Duration::from_secs(2)).unwrap();
        assert_eq!(auth.play_key, [3, 4]);
        assert_eq!(auth.server.address(), "127.0.0.1:7777");
        server.join().unwrap();
    }

    #[test]
    fn high_five_rejects_an_init_with_the_wrong_static_key() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket.write_all(&frame::encode(&[0x11u8; 176])).unwrap();
        });
        let err = authenticate_for(Chronicle::HighFive, "127.0.0.1", addr.port(), "conta", "senha", Duration::from_secs(2))
            .unwrap_err();
        assert!(matches!(err, TransportError::Cipher(_)));
    }
}
