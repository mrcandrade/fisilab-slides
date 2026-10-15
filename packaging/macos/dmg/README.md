# Janela do DMG (macOS)

Arquivos copiados por `package.sh` para dentro da imagem de disco:

| Arquivo | O que é |
|---|---|
| `background.tiff` | Fundo da janela do Finder (HiDPI, 1x e 2x). Vai para `.background/background.tiff`. |
| `dmg-layout.DS_Store` | Ajustes de visualização do Finder (tamanho da janela, posição dos ícones, fundo). Vai para `.DS_Store`. |

Os dois são gerados por `suite-fisilab/scripts/gerar_mac.py`, em https://github.com/mrcandrade/larfic.
Não são gerados durante o build.

O layout espera: volume chamado `Slides` (sem versão), o aplicativo em `Slides.app` e o atalho para
`/Applications` com o nome `Aplicativos`.
