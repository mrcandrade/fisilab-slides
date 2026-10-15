# Modificações do Slides em relação ao DeckCraft

O Slides é a versão do [DeckCraft](https://github.com/storytold/deckcraft) (versão 0.7.0)
distribuída na **Suíte FisiLab** pelo FisiLab — LARFIC · UFRGS, em parceria com o CTA — Centro de
Tecnologia Acadêmica. As modificações são © 2026 FisiLab — LARFIC/UFRGS e seguem os mesmos termos
do projeto original (MIT OR Apache-2.0). Este arquivo lista o que mudou e por quê.

## Nome e marca

- **Empacotamento Linux (AppImage e .deb)**: usa os nomes, ícones e ids da suíte (`FisiLab-Slides-<versão>-x86_64.AppImage`, `fisilab-slides_<versão>_amd64.deb`, id `br.ufrgs.fisilab.slides`, binários `fisilab-slides` e `fisilab-slides-cli`).
- **Nome visível**: "DeckCraft" passou a ser "Slides" na interface — título da janela, nome do
  app no `run_native`, tela inicial (Arquivo › Página Inicial), Arquivo › Conta, janela "About
  Slides", título do Modo do Apresentador, mensagens de status e de erro, texto da ajuda e da
  versão das linhas de comando (`fisilab-slides --version`, `fisilab-slides-cli --help`), mensagens do
  servidor MCP e o tipo de arquivo nos diálogos de abrir/salvar. Onde "Slides" sozinho se
  confundiria com o substantivo comum (os slides da apresentação), o programa é chamado de
  "FisiLab Slides": mensagens de erro e de status ("not a presentation FisiLab Slides can read",
  "FisiLab Slides prints through a PDF…"), o título e as instruções do servidor MCP, o tipo de
  arquivo nos diálogos e a apresentação de exemplo. O tema de slides embutido "DeckCraft" chama-se
  agora "FisiLab Slides" (os nomes antigos "DeckCraft" e "FisiSlides" continuam aceitos por
  `file.new` e `design.theme`), e os metadados gravados nos arquivos (aplicativo do PPTX,
  criador/produtor do PDF, `generator` do `.deckcraft`, nomes dos estilos de tabela) dizem
  "FisiLab Slides".
- **Marca ArtCraft removida**: o NOTICE do DeckCraft diz que o nome, o logotipo e a marca ArtCraft
  (`docs/brand/`) são marcas registradas que **não** estão cobertas pela licença livre. Por isso a
  pasta `docs/brand/` foi apagada, o logotipo foi tirado do topo do README original
  (`README.upstream.md`) e nenhuma tela mostra mais o nome ou a marca ArtCraft — exceto o crédito
  em texto ao projeto de origem, que é obrigatório.
- **Ícone**: o executável (recurso do Windows), a janela/barra de tarefas e a página web usam o
  ícone do Slides (`assets/fisilab/`). O logotipo desenhado em código do DeckCraft, que
  aparecia em Sobre, na tela inicial e em Conta, foi trocado pelo ícone do Slides.
- **Recurso do Windows** (`apps/deckcraft/build.rs`): ProductName "Slides", FileDescription
  "Slides — Apresentações (Suíte FisiLab)", CompanyName "FisiLab — LARFIC/UFRGS",
  OriginalFilename `fisilab-slides.exe` e o copyright com o crédito ao DeckCraft.

## Executáveis, identificador e pastas

- Os executáveis agora se chamam `fisilab-slides` (interface gráfica) e `fisilab-slides-cli` (linha de
  comando), por meio de `[[bin]]` explícitos; os nomes dos pacotes Cargo (`deckcraft`,
  `deckcraft-cli`, `deckcraft-*`) não mudaram.
- Identificador do app: `ai.storyteller.deckcraft` → `br.ufrgs.fisilab.slides`.
- Pasta de configurações (`ui.json`, `prefs.json`, `logs/`): `%APPDATA%\FisiLab Slides`
  (macOS `~/Library/Application Support/FisiLab Slides`, Linux `~/.config/fisilab-slides`), e Recuperação
  Automática em `...\FisiLab Slides\Recovery` (Linux `~/.local/share/fisilab-slides/recovery`). Assim o
  Slides não mistura configurações com uma instalação do DeckCraft. O log chama-se
  `fisilab-slides.log`; o filtro padrão do log (`RUST_LOG` vazio) mostra as mensagens `info` dos
  alvos `deckcraft*` e `fisilab_slides*` (o módulo do executável `fisilab-slides`).
- O empacotamento do Windows (`packaging/windows/`) foi ajustado aos novos nomes, com UpgradeCode e
  GUIDs próprios no MSI (para não atualizar nem remover um DeckCraft instalado), ProgId
  `FisiLab.Slides.Presentation` e pasta de instalação `Program Files\Suite FisiLab\Slides`, a mesma
  do instalador oficial da suíte.

## Cores: paleta original, com temas "Noite" e "Dia"

- A paleta é a original do DeckCraft. Os dois temas dela aparecem com os nomes **Noite** (escuro,
  o `dark` original) e **Dia** (claro, o `light` original, que continua o padrão numa instalação
  nova). Os ids salvos em `ui.json` seguem `dark`/`light`.
- O botão Modo Escuro alterna entre Noite e Dia; o comando `view.theme` e os seletores de tema
  (Arquivo › Conta e Preferências) escolhem um dos dois (aceita também `noite`/`dia`,
  `night`/`day`).
- Nada de cores da marca da suíte no tema: a paleta da marca só aparece na tela de abertura e na
  janela "Sobre a Suíte FisiLab".

## Suíte FisiLab

- Tela de abertura com a logo do CTA nos primeiros segundos (`fisilab_brand::splash`), ligada só
  no executável de desktop. `FISILAB_NO_SPLASH=1` a desliga.
- Janela **"Sobre a Suíte FisiLab…"** (`fisilab_brand::about`): comando `help.fisilab`, item do
  menu Ajuda, botões na tela inicial, em Arquivo › Conta e na janela "About Slides".
- A janela "About Slides" do DeckCraft foi mantida (ela credita os contribuidores do projeto
  original), com o nome novo.

## Links e serviços externos

- Os links da comunidade ArtCraft (Discord, getartcraft.com) foram trocados por: Site da Suíte
  FisiLab, Manifesto pelas tecnologias livres, Projeto original (DeckCraft) e Reportar um problema
  (`https://github.com/mrcandrade/fisilab-slides/issues`) — na interface, no comando `app.links`, em
  `fisilab-slides-cli links`, nas instruções do servidor MCP e nos metadados de empacotamento.
- O botão "Learn More" (guia Record) abria a página do DeckCraft em getartcraft.com; agora abre o
  site da Suíte FisiLab.
- O DeckCraft não tem verificação de atualizações, telemetria nem envio de relatórios de falha;
  o único servidor de rede é o canal de controle local (`--control`, só em `localhost`). Nada
  precisou ser desligado.

## Outros

- `.github/` foi removida (os fluxos de CI, lançamento e assinatura do projeto original rodariam
  contra este repositório).
- `README.md` novo, em português; o original está em `README.upstream.md`. `NOTICE` ganhou um
  cabeçalho do Slides, com o texto original preservado logo abaixo.
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
