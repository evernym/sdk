This repository contains the Evernym SDKs for managing Verifiable Credential eXchange against an Indy network. For specific instructions on building each SDK see the README in the corresponding directory.

To use this repository, you should first be familiar with the [Indy SDK](https://github.com/hyperledger/indy-sdk).


Terms of Use
-------------
Evernym has released this repository under the Apache 2 license.

The branding of Connect.Me, VCX, and Verity are reserved for the exclusive use of Evernym as the trademark holder.

Evernym provides support for using this SDK within the scope of commercial support agreements.


Future Plans
------------
We intend to migrate LibVCX from this repository to
[Hyperledger/indy-sdk](https://github.com/hyperledger/indy-sdk).
* The Indy SDK binary will contain the API functions that are currently contained in LibVCX.
* Because the credential exchange API will become part of LibIndy, the existing CI / CD pipeline will be used.
* Existing automated tests will be executed as part of the build.

Our goal is for the LibVCX API to become the default interface for most developers interacting with Indy, though the current Indy SDK API will continue for low level access.

Incorporating LibVCX into Indy SDK has the following advantages over it remaining a separate binary:
* Developers will only have to work with a single artifact,
* LibVCX development occurs in tandem with the SDK,
* The CI / CD pipeline does not need to significantly change,
* Only one artifact needs to be maintained in the future.

