use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
// use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    println!("Conectado ao servidor!");

    let mut client_clone = client.try_clone().expect("Falha ao clonar o cliente");

    // Thread para receber mensagens
    thread::spawn(move || {
        let mut buff = vec![0; MSG_SIZE];
        loop {
            match client_clone.read_exact(&mut buff) {
                Ok(_) => {
                    let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    let msg = String::from_utf8(msg).expect("Mensagem UTF8 inválida");
                    println!("Mensagem recebida: {:?}", msg);
                }
                Err(_) => {
                    println!("Conexão com o servidor fechada");
                    break;
                }
            }
            buff = vec![0; MSG_SIZE];
        }
    });

    // Loop principal para enviar mensagens
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a entrada");
        let msg = input.trim().to_string();

        if msg == ":quit" || msg == ":q" {
            break;
        }

        let mut buff = msg.clone().into_bytes();
        buff.resize(MSG_SIZE, 0);
        client
            .write_all(&buff)
            .expect("Falha ao escrever no servidor");
    }
}

#[cfg(test)]
mod tests {
    use super::*; // importa todas as funções e variáveis do módulo
    use std::net::TcpListener; // importa a biblioteca TcpListener
                               // use std::thread; // importa a biblioteca thread

    #[test]
    fn test_client_connection_success() {
        // testa se o cliente consegue se conectar ao servidor
        let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
        server
            .set_nonblocking(true)
            .expect("Failed to initialize non-blocking");

        let client = TcpStream::connect(LOCAL); // cria um cliente TCP
        assert!(
            client.is_ok(),
            "Cliente deveria conseguir se conectar ao servidor" // mensagem de erro
        );
    }

    #[test]
    fn test_client_connection_failure() {
        // Tenta conectar a uma porta inválida
        let client = TcpStream::connect("127.0.0.1:99999");
        assert!(
            client.is_err(),
            "Cliente não deveria conseguir se conectar a uma porta inválida" // mensagem de erro
        );
    }

    #[test]
    fn test_message_size_valid() {
        assert_eq!(MSG_SIZE, 32, "Tamanho da mensagem deve ser 32 bytes");
    }

    #[test]
    fn test_message_size_invalid() {
        let invalid_size = 0;
        assert_ne!(MSG_SIZE, invalid_size, "Tamanho da mensagem não deve ser 0");
    }

    #[test]
    fn test_message_trim() {
        let input = "  mensagem com espaços  ";
        let trimmed = input.trim().to_string();
        assert_eq!(
            trimmed, "mensagem com espaços",
            "Espaços devem ser removidos" // mensagem de erro
        );
    }

    #[test]
    fn test_quit_commands() {
        let quit_commands = vec![":quit", ":q"];
        for cmd in quit_commands {
            assert!(cmd == ":quit" || cmd == ":q", "Comando de saída inválido");
        }
    }
}
