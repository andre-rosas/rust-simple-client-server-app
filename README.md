# Chat App em Rust

Um aplicativo de chat simples implementado em Rust, usando TCP para comunicação entre cliente e servidor.

## Funcionalidades

- Conexão TCP entre cliente e servidor
- Suporte a múltiplos clientes
- Mensagens em tempo real
- Interface via linha de comando
- Tamanho fixo de mensagem (32 bytes)

## Requisitos

- Rust 1.70.0 ou superior
- Cargo (gerenciador de pacotes do Rust)

## Instalação

1. Clone o repositório:

```bash
git clone https://github.com/seu-usuario/rust-cliente-server-chat-app.git
cd rust-cliente-server-chat-app
```

2. Compile o projeto:

```bash
cargo build
```

## Uso

### Iniciando o Servidor

```bash
cargo run --bin server
```

### Iniciando o Cliente

```bash
cargo run --bin client
```

### Comandos do Cliente

- Digite sua mensagem e pressione Enter para enviar
- Digite `:quit` ou `:q` para sair do chat

## Testes

Para executar os testes unitários:

```bash
cargo test
```

## Estrutura do Projeto

```
rust-cliente-server-chat-app/
├── src/
│   ├── main.rs      # Código do servidor
│   └── client.rs    # Código do cliente
├── Cargo.toml       # Configurações do projeto
└── README.md        # Este arquivo
```

## Limitações

- Tamanho máximo de mensagem: 32 bytes
- Sem criptografia
- Sem persistência de mensagens
- Sem autenticação de usuários

## Contribuindo

1. Faça um fork do projeto
2. Crie uma branch para sua feature (`git checkout -b feature/nova-feature`)
3. Commit suas mudanças (`git commit -m 'Adiciona nova feature'`)
4. Push para a branch (`git push origin feature/nova-feature`)
5. Abra um Pull Request

## Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.
