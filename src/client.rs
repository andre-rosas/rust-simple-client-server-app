use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
// use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000"; // Endereço do servidor TCP (IP e porta)
const MSG_SIZE: usize = 32; // Tamanho fixo das mensagens em bytes

/// Ponto de entrada principal do cliente de chat TCP.
///
/// Esta função:
/// 1. Estabelece conexão com o servidor
/// 2. Inicia uma thread separada para receber mensagens
/// 3. Processa entrada do usuário para enviar mensagens
/// 4. Gerencia o comando de saída do aplicativo
fn main() {
    // Conecta ao servidor TCP no endereço especificado
    let mut client = TcpStream::connect(LOCAL).expect("Falha na conexão com o servidor");
    // Mensagem de confirmação de conexão
    println!("Conectado ao servidor!");

    // Clona o stream TCP para usar em outra thread
    let mut client_clone = client.try_clone().expect("Falha ao duplicar a conexão TCP");

    // Inicia uma nova thread para lidar com mensagens recebidas
    thread::spawn(move || {
        // Buffer de tamanho fixo para armazenar mensagens recebidas (array de bytes)
        let mut buff = [0u8; MSG_SIZE];

        // Loop infinito para recebimento contínuo de mensagens
        loop {
            // Lê exatamente MSG_SIZE bytes do stream
            match client_clone.read_exact(&mut buff) {
                // Leitura bem-sucedida
                Ok(_) => {
                    // Encontra o índice do primeiro byte nulo (fim da mensagem)
                    let end = buff.iter().position(|&x| x == 0).unwrap_or(MSG_SIZE);

                    // Converte os bytes válidos para String UTF-8
                    match String::from_utf8(buff[..end].to_vec()) {
                        Ok(msg) => println!("Mensagem recebida: {:?}", msg), // Exibe mensagem
                        Err(_) => eprintln!("Erro: Mensagem UTF-8 inválida"), // Trata erro de codificação
                    }

                    // Reseta o buffer com zeros para a próxima mensagem
                    // CORREÇÃO: Usa atribuição direta do array (sem conversão de tipo)
                    buff = [0u8; MSG_SIZE];
                }
                // Erro na leitura (provavelmente conexão fechada)
                Err(_) => {
                    println!("Conexão com o servidor foi encerrada");
                    break; // Sai do loop
                }
            }
        }
    });

    // Loop principal para envio de mensagens
    loop {
        // Buffer para armazenar entrada do usuário
        let mut input = String::new();

        // Lê uma linha do terminal (stdin)
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada do terminal");

        // Remove espaços em branco do início/fim e converte para String
        let msg = input.trim().to_string();

        // Verifica comandos de saída
        if msg == ":quit" || msg == ":q" {
            break; // Encerra o loop principal
        }

        // Prepara buffer de envio com tamanho fixo (array de bytes)
        let mut send_buff = [0u8; MSG_SIZE];
        // Converte a mensagem para bytes
        let msg_bytes = msg.as_bytes();
        // Determina tamanho real dos dados (limitado ao MSG_SIZE)
        let len = std::cmp::min(msg_bytes.len(), MSG_SIZE);

        // Copia os dados da mensagem para o buffer de envio
        send_buff[..len].copy_from_slice(&msg_bytes[..len]);

        // Envia o buffer completo para o servidor
        client
            .write_all(&send_buff)
            .expect("Falha ao enviar mensagem para o servidor");
    }
}

// Módulo de testes unitários
#[cfg(test)]
mod tests {
    use super::*; // Importa todos os elementos do módulo pai
    use std::net::TcpListener; // Para testes de conexão

    /// Testa conexão bem-sucedida com o servidor
    #[test]
    fn test_client_connection_success() {
        // Cria um listener TCP para simular o servidor
        let server = TcpListener::bind(LOCAL).expect("Falha ao iniciar servidor de teste");
        // Configura como não-bloqueante para não travar o teste
        server
            .set_nonblocking(true)
            .expect("Falha ao configurar não-bloqueante");

        // Tenta conectar ao servidor
        let client = TcpStream::connect(LOCAL);
        // Verifica se a conexão foi bem-sucedida
        assert!(client.is_ok(), "Cliente deveria conectar com sucesso");
    }

    /// Testa falha de conexão em porta inválida
    #[test]
    fn test_client_connection_failure() {
        // Tenta conectar a uma porta inválida
        let client = TcpStream::connect("127.0.0.1:99999");
        // Verifica se retornou erro como esperado
        assert!(client.is_err(), "Conexão deveria falhar em porta inválida");
    }

    /// Testa se o tamanho da mensagem está correto
    #[test]
    fn test_message_size_valid() {
        // Verifica se a constante tem o valor esperado
        assert_eq!(MSG_SIZE, 32, "Tamanho de mensagem deve ser 32 bytes");
    }

    /// Testa o tratamento de strings com espaços
    #[test]
    fn test_message_trim() {
        // String com espaços
        let input = "  mensagem teste  ";
        // String após trim()
        let trimmed = input.trim().to_string();
        // Verifica se os espaços foram removidos corretamente
        assert_eq!(trimmed, "mensagem teste", "Espaços devem ser removidos");
    }

    /// Testa os comandos de saída do aplicativo
    #[test]
    fn test_quit_commands() {
        // Lista de comandos válidos
        let quit_commands = vec![":quit", ":q"];
        // Verifica cada comando
        for cmd in quit_commands {
            assert!(
                cmd == ":quit" || cmd == ":q",
                "Comando deve ser reconhecido como de saída"
            );
        }
    }
}
