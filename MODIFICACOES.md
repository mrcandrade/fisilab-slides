# Modificações do FisiSlides em relação ao DeckCraft

O FisiSlides é a versão do [DeckCraft](https://github.com/storytold/deckcraft) (versão 0.7.0)
distribuída na **Suíte FisiLab** pelo FisiLab — LARFIC · UFRGS, em parceria com o CTA — Centro de
Tecnologia Acadêmica. As modificações são © 2026 FisiLab — LARFIC/UFRGS e seguem os mesmos termos
do projeto original (MIT OR Apache-2.0). Este arquivo lista o que mudou e por quê.

## Nome e marca

- **Nome visível**: "DeckCraft" passou a ser "FisiSlides" na interface — título da janela, nome do
  app no `run_native`, tela inicial (Arquivo › Página Inicial), Arquivo › Conta, janela "About
  FisiSlides", título do Modo do Apresentador, mensagens de status e de erro, texto da ajuda e da
  versão das linhas de comando (`fisislides --version`, `fisislides-cli --help`), mensagens do
  servidor MCP e o tipo de arquivo nos diálogos de abrir/salvar. O tema de slides embutido
  "DeckCraft" chama-se agora "FisiSlides" (o nome antigo continua aceito por `file.new` e
  `design.theme`), e os metadados gravados nos arquivos (aplicativo do PPTX, criador/produtor do
  PDF, `generator` do `.deckcraft`, nomes dos estilos de tabela) dizem "FisiSlides".
- **Marca ArtCraft removida**: o NOTICE do DeckCraft diz que o nome, o logotipo e a marca ArtCraft
  (`docs/brand/`) são marcas registradas que **não** estão cobertas pela licença livre. Por isso a
  pasta `docs/brand/` foi apagada, o logotipo foi tirado do topo do README original
  (`README.upstream.md`) e nenhuma tela mostra mais o nome ou a marca ArtCraft — exceto o crédito
  em texto ao projeto de origem, que é obrigatório.
- **Ícone**: o executável (recurso do Windows), a janela/barra de tarefas e a página web usam o
  ícone do FisiSlides (`assets/fisilab/`). O logotipo desenhado em código do DeckCraft, que
  aparecia em Sobre, na tela inicial e em Conta, foi trocado pelo ícone do FisiSlides.
- **Recurso do Windows** (`apps/deckcraft/build.rs`): ProductName "FisiSlides", FileDescription
  "FisiSlides — Apresentações (Suíte FisiLab)", CompanyName "FisiLab — LARFIC/UFRGS",
  OriginalFilename `fisislides.exe` e o copyright com o crédito ao DeckCraft.

## Executáveis, identificador e pastas

- Os executáveis agora se chamam `fisislides` (interface gráfica) e `fisislides-cli` (linha de
  comando), por meio de `[[bin]]` explícitos; os nomes dos pacotes Cargo (`deckcraft`,
  `deckcraft-cli`, `deckcraft-*`) não mudaram.
- Identificador do app: `ai.storyteller.deckcraft` → `br.ufrgs.fisilab.fisislides`.
- Pasta de configurações (`ui.json`, `prefs.json`, `logs/`): `%APPDATA%\FisiSlides`
  (macOS `~/Library/Application Support/FisiSlides`, Linux `~/.config/fisislides`), e Recuperação
  Automática em `...\FisiSlides\Recovery` (Linux `~/.local/share/fisislides/recovery`). Assim o
  FisiSlides não mistura configurações com uma instalação do DeckCraft. O log chama-se
  `fisislides.log`.
- O empacotamento do Windows (`packaging/windows/`) foi ajustado aos novos nomes, com UpgradeCode e
  GUIDs próprios no MSI (para não atualizar nem remover um DeckCraft instalado) e ProgId
  `FisiLab.FisiSlides.Presentation`.

## Cores: temas "UFRGS Noite" e "UFRGS Dia"

- `crates/ui-egui/src/theme.rs` ganhou os temas **UFRGS Noite** (escuro, id `ufrgsNoite`) e
  **UFRGS Dia** (claro, id `ufrgsDia`), montados a partir da paleta da marca
  (`fisilab_brand::palette::NOITE`/`DIA`). **UFRGS Noite é o padrão** numa instalação nova.
- Os temas originais (claro e escuro) continuam disponíveis como "Claro (original)" e "Escuro
  (original)". O botão Modo Escuro alterna entre o claro e o escuro da mesma família; o novo
  comando `view.theme` e os seletores de tema (Arquivo › Conta e Preferências) escolhem qualquer
  um dos quatro.
- Só as cores mudaram: cantos, espaçamentos e tamanhos são os do DeckCraft. Cores que ficam sobre
  o próprio slide (alças de seleção, cursor) e as miniaturas de transições e animações continuam
  neutras.

## Suíte FisiLab

- Tela de abertura com a logo do CTA nos primeiros segundos (`fisilab_brand::splash`), ligada só
  no executável de desktop. `FISILAB_NO_SPLASH=1` a desliga.
- Janela **"Sobre a Suíte FisiLab…"** (`fisilab_brand::about`): comando `help.fisilab`, item do
  menu Ajuda, botões na tela inicial, em Arquivo › Conta e na janela "About FisiSlides".
- A janela "About FisiSlides" do DeckCraft foi mantida (ela credita os contribuidores do projeto
  original), com o nome novo.

## Links e serviços externos

- Os links da comunidade ArtCraft (Discord, getartcraft.com) foram trocados por: Site da Suíte
  FisiLab, Manifesto pelas tecnologias livres, Projeto original (DeckCraft) e Reportar um problema
  (`https://github.com/mrcandrade/fisislides/issues`) — na interface, no comando `app.links`, em
  `fisislides-cli links`, nas instruções do servidor MCP e nos metadados de empacotamento.
- O botão "Learn More" (guia Record) abria a página do DeckCraft em getartcraft.com; agora abre o
  site da Suíte FisiLab.
- O DeckCraft não tem verificação de atualizações, telemetria nem envio de relatórios de falha;
  o único servidor de rede é o canal de controle local (`--control`, só em `localhost`). Nada
  precisou ser desligado.

## Outros

- `.github/` foi removida (os fluxos de CI, lançamento e assinatura do projeto original rodariam
  contra este repositório).
- `README.md` novo, em português; o original está em `README.upstream.md`. `NOTICE` ganhou um
  cabeçalho do FisiSlides, com o texto original preservado logo abaixo.
- `xtask` (tabela de camadas) registra o crate `fisilab-brand` na camada da interface.

## O que não mudou

- O código de edição, os formatos de arquivo e as extensões (`.deckcraft` continua `.deckcraft`,
  por compatibilidade), os comandos e seus ids, os nomes dos crates e módulos Rust, as variáveis
  de ambiente `DECKCRAFT_*`, os comentários do código e a documentação histórica em `docs/`.
- A interface continua em inglês: o DeckCraft não traz traduções, então não há idioma pt-BR para
  tornar padrão.
- `LICENSE-MIT`, `LICENSE-APACHE` e `ATTRIBUTION.md` não foram alterados.
- O empacotamento para Linux, macOS e FreeBSD só teve os links trocados; os nomes de arquivo e o
  identificador desses pacotes continuam os do DeckCraft (a suíte distribui só para Windows).
