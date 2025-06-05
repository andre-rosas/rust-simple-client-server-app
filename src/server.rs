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
fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind."); // cria um servidor TCP
    server
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking."); // define o servidor como não bloqueante

    let mut clients = vec![]; // cria um vetor para armazenar os clientes
    let (tx, rx) = mpsc::channel::<String>(); // cria um canal de comunicação entre threads
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            // aceita uma conexão de cliente
            println!("Client {} connected.", addr); // imprime o endereço do cliente
            let tx = tx.clone(); // clona o canal de comunicação entre threads
            clients.push(socket.try_clone().expect("Failed to clone client.")); // clona o socket do cliente

            let _handle = thread::spawn(move || {
                // cria uma thread para o cliente
                loop {
                    // cria uma thread para o cliente
                    let mut buff = vec![0; MSG_SIZE]; // cria um buffer para armazenar a mensagem
                    match socket.read_exact(&mut buff) {
                        // lê a mensagem do cliente
                        // lê a mensagem do cliente
                        Ok(_) => {
                            // se a mensagem foi lida com sucesso
                            let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(); // converte o buffer em uma string
                            let msg = String::from_utf8(msg).expect("Invalid UTF8 message."); // converte a string em uma mensagem

                            println!("{}: {:?}", addr, msg); // imprime o endereço do cliente e a mensagem
                            tx.send(msg).expect("Failed to send message to rx.");
                            // envia a mensagem para o canal de comunicação entre threads
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (), // se o cliente não enviou uma mensagem, continua o loop
                        Err(_) => {
                            // se o cliente enviou uma mensagem com erro, fecha a conexão
                            println!("Closing connection with {}.", addr); // imprime o endereço do cliente
                            break; // termina o loop
                        }
                    }
                    sleep(); // espera 100ms
                }
            });
        }
        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    // filtra os clientes
                    let mut buff = msg.clone().into_bytes(); // converte a mensagem em um buffer
                    buff.resize(MSG_SIZE, 0); // define o tamanho do buffer
                    client.write_all(&buff).map(|_| client).ok() // envia a mensagem para o cliente
                })
                .collect::<Vec<_>>(); // converte o vetor de clientes em um vetor de clientes
        }
        sleep(); // espera 100ms
    }
}

#[cfg(test)]
mod tests {
    use super::*; // importa todas as funções e variáveis do módulo
    use std::net::TcpStream; // importa a biblioteca TcpStream
                             // use std::thread; // importa a biblioteca thread
    use std::time::Duration; // importa a biblioteca Duration

    #[test]
    fn test_server_connection_success() {
        // testa se o servidor consegue se conectar ao cliente
        let server = TcpListener::bind(LOCAL).expect("Listener failed to bind"); // cria um servidor TCP
        server
            .set_nonblocking(true)
            .expect("Failed to initialize non-blocking"); // define o servidor como não bloqueante

        let client = TcpStream::connect(LOCAL); // cria um cliente TCP
        assert!(
            client.is_ok(), // verifica se o cliente conseguiu se conectar ao servidor
            "Cliente deveria conseguir se conectar ao servidor"  // mensagem de erro
        );
    }

    #[test]
    fn test_server_connection_failure() {
        // testa se o servidor não consegue se conectar ao cliente
        // Tenta conectar a uma porta inválida
        let client = TcpStream::connect("127.0.0.1:99999"); // cria um cliente TCP
        assert!(
            client.is_err(), // verifica se o cliente não conseguiu se conectar ao servidor
            "Cliente não deveria conseguir se conectar a uma porta inválida"  // mensagem de erro
        );
    }

    #[test]
    fn test_message_size_valid() {
        // testa se o tamanho da mensagem é válido
        assert_eq!(MSG_SIZE, 32, "Tamanho da mensagem deve ser 32 bytes"); // verifica se o tamanho da mensagem é 32 bytes
    }

    #[test]
    fn test_message_size_invalid() {
        // testa se o tamanho da mensagem é inválido
        let invalid_size = 0; // define o tamanho da mensagem como 0
        assert_ne!(MSG_SIZE, invalid_size, "Tamanho da mensagem não deve ser 0");
        // verifica se o tamanho da mensagem não é 0
    }

    #[test]
    fn test_sleep_duration() {
        // testa se o tempo de espera é válido
        let start = std::time::Instant::now(); // define o tempo de início
        sleep(); // espera 100ms
        let duration = start.elapsed(); // define o tempo de fim
        assert!(
            duration >= Duration::from_millis(100), // verifica se o tempo de espera é maior ou igual a 100ms
            "Sleep deve durar pelo menos 100ms"     // mensagem de erro
        );
    }
}
