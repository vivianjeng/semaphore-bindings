# Semaphore bindings for mobile native

This is the implentation for generating semaphore mobile bindings for iOS and Android with Mopro and UniFFI.

**📚 To learn more about mopro, visit: https://zkmopro.org**

## Getting Started

To set up and build bindings, follow these steps.

### 1. Install the Mopro CLI Tool

-   Get published CLI

```sh
cargo install mopro-cli
```

-   Or get the latest CLI on GitHub

```sh
git clone https://github.com/zkmopro/mopro
cd mopro/cli
cargo install --path .
```

### 2. Generate Native Bindings

Build bindings for your project by executing:

```sh
mopro build
```

### 3. Create Platform-Specific Templates

To generate templates tailored to your target platform, use:

```sh
mopro create
```

### 4. Open the project

Follow the instructions to open the development tools

For iOS:

```sh
open ios/MoproApp.xcodeproj
```

For Android:

```sh
open android -a Android\ Studio
```

For Web:

```sh
cd web && yarn && yarn start
```

For React Native:
Follow the README in the `react-native` directory. Or [zkmopro/react-native-app/README.md](https://github.com/zkmopro/react-native-app/blob/main/README.md)

For Flutter:
Follow the README in the `flutter` directory. Or [zkmopro/flutter-app/README.md](https://github.com/zkmopro/flutter-app/blob/main/README.md)

### 5. Update bindings

After creating templates, you may still need to update the bindings.

Once you've run `mopro build`, be sure to run mopro update to refresh the bindings in each template. This command will automatically locate the corresponding bindings folders and update them accordingly.

```sh
mopro update
```

## Test

Run tests before building bindings

```sh
cargo test
```

## Swift Usage

Import Mopro Bindings before using semaphore package

```swift
import moproFFI
```

> Will use customized library name after this is done https://github.com/zkmopro/mopro/issues/413

### Identity

#### `Identity`

```swift
let privateKey = "secret".data(using: .utf8)!
// generate an identity from privateKey
let identity = Identity(privateKey: privateKey)
// get the identity commitment
identity.commitment()
// get private key
identity.privateKey()
// get secret scalar
identity.secretScalar()
// convert the type to Element to be used in Group
identity.toElement()
```

#### `Group`

```swift
let group = Group(members: [
    identity.toElement(),
    identity2.toElement()
])
// get root
group.root()
// get depth (TODO: need to be tested)
// get members (TODO: need to be tested)
// index of (TODO: need to be tested)
// add member (TODO: need to be tested)
// add members (TODO: need to be tested)
// update member (TODO: need to be tested)
// remove member (TODO: need to be tested)
```

#### `Proof`

```swift
let message = "message"
let scope = "scope"
// generate semaphore proof
// It will output a JSON string
// You can parse it with swift JSON parser
let proof = try generateSemaphoreProof(
    identity: identity,
    group: group,
    message: message,
    scope: scope,
    merkleTreeDepth: 16
)
// verify semaphore proof
let valid = try verifySemaphoreProof(proof: proof)
```

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>
