# Contributing to OBDium

OBDium deals with raw vehicle diagnostics (e.g Mode $01, $06, $09), hex decoding, real CAN data, and OBD-II protocol parsing. Contributions are welcome, but if you’re touching the core, **you need to know what you’re doing**.

---

## Guidelines
There are two simple guideline for making a commit:

### 1. Be knowledgeable
- If you're working on anything protocol-related, like parsing responses or decoding monitor data, you should already understand:
  - SAE J1979 / ISO 15031
  - OBD-II mode structure
  - Bit-level interpretation of hex payloads

### 2. Test your feature thoroughly
- If you're working on anything protocol-related, it is essential that you either thoroughly test your feature using either an ELM327 emulator or real vehicle. Your code will be tested on a real vehicle before being pushed.

Otherwise, if you're working on something not apart of the core logic, like the frontend, a markdown file, or even documenting logic (including the backend), don't hesitate to submit a pull request.

## To Contribute
1. Fork the repository

2. Create a new branch for your feature or fix

3. Make your changes

4. Submit a pull request describing your changes and include how this was tested

## What a Good PR Looks Like

- **Tested**  
  You tested it on a real car or an emulator (a popular one is [ELM327-emulator](https://github.com/Ircama/ELM327-emulator/)), or you referenced official docs. Protocol logic must be grounded in something concrete.

- **Scoped**  
  One feature or fix per PR. Don’t combine unrelated changes.

- **Clear**  
  Use real commit messages. If something’s not obvious, be sure to explain it.

## Where Do I Start?
There are plenty of great resources to start. The first step is to have a decent understanding of what goes on under the hood. Here are a couple great resources:

- [Wikipedia - OBD-II PIDs](https://en.wikipedia.org/wiki/OBD-II_PIDs)
- [SAE J1979 - Detailed PDF](https://law.resource.org/pub/us/cfr/ibr/005/sae.j1979.2002.pdf)
- [OBDTester - ELM AT Commands](https://www.obdtester.com/elm-usb-commands)
- [CSS Electronics - A Brief Overview of OBD2](https://www.csselectronics.com/pages/obd2-explained-simple-intro)

Thank you for your contributions to OBDium.