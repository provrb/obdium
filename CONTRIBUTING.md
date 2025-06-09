# Contributing to OBDium

OBDium deals with raw vehicle diagnostics (e.g Mode $01, $06, $09), hex decoding, real CAN data, and OBD-II protocol parsing. Contributions are welcome, but if you’re touching the core, **you need to know what you’re doing**.

---

## Guidelines

1. **Know the Protocols**  
   If you're working on anything protocol-related, like parsing responses or decoding monitor data, you should already understand:
   - SAE J1979 / ISO 15031
   - OBD-II mode structure
   - Bit-level interpretation of hex payloads

2. **No Trial-and-Error Commits**  
   If you're trial-and-erroring, your PR will be closed. Incorrect logic can mislead consumers or break core functionality. If you’re unsure, do some quick research. Take some time to understand the area a bit more and come back!

3. **Communicate**
   If you have any questions about an idea, please, feel free to open an [Issue](../../issues).

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

## What Gets Shut Down Fast

- Blatantly incorrect PRs with no evidence the feature works
- AI-generated PRs with no understanding
- Guess and check PRs

## Where Do I Start?
There are plenty of great resources to start. The first step is to have a decent understanding of what goes on under the hood. Here are a couple great resources:

- [Wikipedia - OBD-II PIDs](https://en.wikipedia.org/wiki/OBD-II_PIDs)
- [SAE J1979 - Detailed PDF](https://law.resource.org/pub/us/cfr/ibr/005/sae.j1979.2002.pdf)
- [OBDTester - ELM AT Commands](https://www.obdtester.com/elm-usb-commands)
- [CSS Electronics - A Brief Overview of OBD2](https://www.csselectronics.com/pages/obd2-explained-simple-intro)

Thank you for your contributions to OBDium.