# Holochain Developer Setup

[![Project](https://img.shields.io/badge/project-holochain-blue.svg?style=flat-square)](http://holochain.org/)
[![PM](https://img.shields.io/badge/pm-waffle-blue.svg?style=flat-square)](https://waffle.io/holochain/org)
[![Chat](https://img.shields.io/badge/chat-chat%2eholochain%2enet-blue.svg?style=flat-square)](https://chat.holochain.net)

## Possible Setup
This repo is to try out some ways to make onboarding devs smoother.

## Dependency
At this point there is a dependency on libzmq please check Step 4 [here](https://developer.holochain.org/start.html) for your OS.

## Test it out - holochain-rust-todo
To try this dev experience make sure you have node, Rust and libzmq installed.
Set your rust to use "nightly-2019-01-24" like this ```rustup default nightly-2019-01-24```

- In the holochain-rust-todo test folder run ```npm install```
- Next go back up one folder ```cd ..```
- In the holochain-rust-todo folder run ```npm run hc:test-mac```
  - For linux ```npm run hc:test-linux```
  - For Windows ```npm run hc:test-win```
- You should see the hApp being compiled with Rust
  - When the compilation is done the tests run and you will see output like this

  ```
    Created bundle file at "/Users/philipbeadle/holochain/hApps/dev-camp-idea/holochain-rust-todo/dist/bundle.json"
    Running tests in test/index.js
    > node test/index.js
    TAP version 13
    # Can create a list
    Reading DNA from dist/bundle.json
    Starting instance "alice"...
    { Ok: 'QmbJS82C4HLsA8wXWkZLyjL2L4Z6MeP2Brxnf25qNsUcW4' }
    ok 1 should not be equal
    Stopping instance "alice"...

    ... snip ...

    1..4
    # tests 4
    # pass  4

    # ok
  ```

## Test it out - holochain-basic-chat
Let's run the tests first. Do the same as above:

- In the holochain-basic-chat/dna-src/test/ folder run ```npm install```
- Next go back up two folders ```cd ../..```
- In the holochain-basic-chat folder run ```npm run hc:test-mac```
  - For linux ```npm run hc:test-linux```
  - For Windows ```npm run hc:test-win```
- You should see the hApp being compiled with Rust and the test results.

Now let's build and start the holochain-basic-chat and use a browser to chat with people.
- In the holochain-basic-chat folder run ```npm run hc:build-mac```
  - For linux ```npm run hc:build-linux```
  - For Windows ```npm run hc:build-win```
  - Output should say ```Created bundle file at "../dna/holo-chat.dna.json" ```
- Install the UI components ```cd ui-src/ && npm install```
- Build the UI ```npm run build```
- Next go back up one folder ```cd ..```
- Start Holochain conductor with the conductor-config.toml ```npm run hc:start-mac```
  - You will see
  ``` Using config path: ./conductor-config.toml
      Reading DNA from dna/holo-chat.dna.json
      adding ui interface ui-interface
      Successfully loaded 1 instance configurations
      Starting all of them...
      Starting instance "holo-chat"...
      Starting interfaces...
      2019-02-22 13:05:13:holo-chat: debug/conductor:  instantiated
      Done.
      Starting UI servers
      Starting all servers
      About to serve path "./ui" at http://127.0.0.1:4200
      Server started for "ui-interface"
      Listening on http://127.0.0.1:4200
    ```

- Open a browser to ```http://localhost:4200```
- Create yourself a handle and paste in a URL to a shot of your smiling face! You're now running our bsic chat hApp on Holochain!!

### Caveat
Networking is not configured here yet. We will have a solid networking option real soon.

## Contribute
Holochain is an open source project.  We welcome all sorts of participation and are actively working on increasing surface area to accept it.  Please see our [contributing guidelines](../CONTRIBUTING.md) for our general practices and protocols on participating in the community.

## License
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

Copyright (C) 2019, Holochain Foundation

This program is free software: you can redistribute it and/or modify it under the terms of the license p
rovided in the LICENSE file (GPLv3).  This program is distributed in the hope that it will be useful, bu
t WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 PURPOSE.

**Note:** We are considering other 'looser' licensing options (like MIT license) but at this stage are using GPL while we're getting the matter sorted out.  See [this article](https://medium.com/holochain/licensing-needs-for-truly-p2p-software-a3e0fa42be6c) for some of our thinking on licensing for distributed application frameworks.
