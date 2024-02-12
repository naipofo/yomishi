# Yomishi

An atempt at a ground-up rewrite of [yomichan](https://github.com/FooSoft/yomichan).

> [!NOTE]
> Yomishi is in early stages of development. While it already works quite well, instead you might want to use an up to date fork of yomichan made by the amazing people at themoeway - [yomitan](https://github.com/themoeway/yomitan)

## Goals

I've decided on rewriting yomichan instead of forking to make it easier to achieve some important goals:

- Code decomposition - no huge backed scripts
- Less coupling - i want to be able to run the backed in many places, not only as an extension:
  - Mobile app - handle selected text, similar to how gTransalte does it
  - API and plugins to make it easily extendable
  - Anki plugin to manipulate the cards you already have
- Multilingual - not only Japanese!
- Remote server - host your dictionary away from the client using it

## Screenshots

![Scanning](https://i.imgur.com/RvOSqLy.png)

![Config](https://i.imgur.com/zA6pvHc.png)
