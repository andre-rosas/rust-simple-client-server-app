use std::io::{ErrorKind, Read, Write}; // biblioteca padrão para ler e escrever em um socket
use std::net::TcpListener; // biblioteca padrão para criar um servidor TCP
use std::sync::mpsc; // biblioteca padrão para criar um canal de comunicação entre threads
use std::thread;
use std::time::Duration; // biblioteca padrão para criar uma thread

const LOCAL: &str = "127.0.0.1:6000"; // endereço local do servidor
const MSG_SIZE: usize = 32; // tamanho do mensagem

fn sleep() {
    thread::sleep(Duration::from_millis(100)); // espera 100ms
}

/// Servidor de chat TCP básico
///
/// Funcionalidades:
/// 1. Aceita conexões de múltiplos clientes
/// 2. Recebe mensagens de clientes e retransmite para todos conectados
/// 3. Gerencia desconexões de clientes
fn main() {
    // Cria listener TCP no endereço especificado
    let server = TcpListener::bind(LOCAL).expect("Falha ao iniciar servidor");
    // Configura o listener para operação não-bloqueante
    server
        .set_nonblocking(true)
        .expect("Falha ao configurar não-bloqueante");
    println!("Servidor iniciado em {}", LOCAL);

    // Armazena streams TCP dos clientes conectados
    let mut clients = vec![];
    // Cria canal para comunicação entre threads (transmissor, receptor)
    let (tx, rx) = mpsc::channel::<String>();

    // Loop principal do servidor
    loop {
        // Tenta aceitar nova conexão
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Cliente {} conectado.", addr);

            // Clona o transmissor para uso na thread do cliente
            let tx = tx.clone();
            // Clona o socket para adicionar à lista de clientes
            clients.push(socket.try_clone().expect("Falha ao clonar socket"));

            // Inicia thread para lidar com o cliente
            thread::spawn(move || {
                // Buffer de tamanho fixo para armazenar mensagens recebidas
                let mut buff = [0u8; MSG_SIZE];

                // Loop de comunicação com o cliente
                loop {
                    // Lê dados do socket
                    match socket.read_exact(&mut buff) {
                        // Leitura bem-sucedida
                        Ok(_) => {
                            // Encontra o índice do primeiro byte nulo (fim da mensagem)
                            let end = buff.iter().position(|&x| x == 0).unwrap_or(MSG_SIZE);

                            // Converte bytes para string UTF-8
                            if let Ok(msg) = String::from_utf8(buff[..end].to_vec()) {
                                // Exibe mensagem recebida
                                println!("{}: {:?}", addr, msg);
                                // Envia mensagem para o canal principal
                                tx.send(msg).expect("Falha ao enviar para canal");
                            } else {
                                eprintln!("UTF-8 inválido de {}", addr);
                            }

                            // Reseta o buffer para a próxima mensagem
                            buff = [0u8; MSG_SIZE];
                        }
                        // Erro de operação não-bloqueante (sem dados disponíveis)
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        // Outros erros (normalmente desconexão)
                        Err(_) => {
                            println!("Fechando conexão com {}", addr);
                            break; // Encerra o loop da thread
                        }
                    }
                    // Pausa para evitar uso excessivo da CPU
                    sleep();
                }
            });
        }

        // Verifica se há mensagens no canal para retransmitir
        if let Ok(msg) = rx.try_recv() {
            // Filtra clientes ativos e envia mensagem
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    // Prepara buffer de envio
                    let mut send_buff = [0u8; MSG_SIZE];
                    // Converte mensagem para bytes
                    let msg_bytes = msg.as_bytes();
                    // Determina tamanho real dos dados
                    let len = std::cmp::min(msg_bytes.len(), MSG_SIZE);

                    // Copia dados para o buffer de envio
                    send_buff[..len].copy_from_slice(&msg_bytes[..len]);

                    // Tenta enviar e mantém cliente se bem-sucedido
                    client.write_all(&send_buff).map(|_| client).ok()
                })
                .collect(); // Coleta clientes ativos em novo vetor
        }

        // Pausa o loop principal
        sleep();
    }
}

// Módulo de testes unitários

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;
    use std::net::TcpStream;

    #[test]
    fn test_server_start() {
        let server = TcpListener::bind("127.0.0.1:0");
        assert!(server.is_ok(), "Servidor deveria iniciar em porta livre");
    }

    #[test]
    fn test_nonblocking_setting() {
        let server = TcpListener::bind("127.0.0.1:0").expect("Falha ao iniciar servidor");
        let result = server.set_nonblocking(true);
        assert!(
            result.is_ok(),
            "Configuração não-bloqueante deveria funcionar"
        );
    }

    #[test]
    fn test_client_connection() {
        let server = TcpListener::bind("127.0.0.1:0").expect("Falha ao iniciar servidor");
        let addr = server.local_addr().expect("Falha ao obter endereço local");
        server
            .set_nonblocking(true)
            .expect("Falha na configuração não-bloqueante");
        let client = TcpStream::connect(addr);
        assert!(client.is_ok(), "Cliente deveria conectar com sucesso");
    }

    #[test]
    fn test_message_size() {
        assert_eq!(MSG_SIZE, 32, "Tamanho de mensagem deve ser 32 bytes");
    }

    #[test]
    fn test_sleep_duration() {
        let start = std::time::Instant::now();
        sleep();
        let duration = start.elapsed();
        assert!(
            duration >= Duration::from_millis(90),
            "Sleep deve durar pelo menos 90ms"
        );
    }

    #[test]
    fn test_invalid_port() {
        let server = TcpListener::bind("127.0.0.1:99999");
        assert!(server.is_err(), "Porta inválida deveria falhar");
    }

    #[test]
    fn test_message_echo() {
        let server = TcpListener::bind("127.0.0.1:0").expect("Falha ao iniciar servidor");
        let addr = server.local_addr().expect("Falha ao obter endereço local");
        server
            .set_nonblocking(true)
            .expect("Falha na configuração não-bloqueante");

        // Inicia thread do servidor com loop de aceitação
        thread::spawn(move || {
            loop {
                match server.accept() {
                    Ok((mut socket, _)) => {
                        let mut buff = [0u8; MSG_SIZE];
                        let n = socket.read(&mut buff).expect("Leitura falhou");
                        socket.write_all(&buff[..n]).expect("Escrita falhou");
                        break;
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        // Espera se não houver conexões disponíveis
                        sleep();
                    }
                    Err(e) => panic!("Erro inesperado: {}", e),
                }
            }
        });

        // Dá tempo para o servidor iniciar
        thread::sleep(Duration::from_millis(50));

        let mut client = TcpStream::connect(addr).expect("Falha na conexão");
        let msg = "teste";
        client.write_all(msg.as_bytes()).expect("Falha no envio");

        let mut recv_buff = vec![0; msg.len()];
        client.read_exact(&mut recv_buff).expect("Falha na leitura");

        assert_eq!(
            msg.as_bytes(),
            &recv_buff[..],
            "Mensagem deveria ser ecoada"
        );
    }
}
