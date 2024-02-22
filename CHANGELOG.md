# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.0.16](https://github.com/EKGF/terraform-neptune/compare/0.0.15..0.0.16) - 2024-02-22
#### Features
- more detailed load-request status updates - ([cde3681](https://github.com/EKGF/terraform-neptune/commit/cde3681bccc6b05c9bc2d4ea80a104aeb7c84acf)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.15](https://github.com/EKGF/terraform-neptune/compare/0.0.14..0.0.15) - 2024-02-21

- - -

## [0.0.14](https://github.com/EKGF/terraform-neptune/compare/0.0.13..0.0.14) - 2024-02-21
#### Features
- added additional LambdaDetailStatus enums for DispatchFailures - ([8188fb6](https://github.com/EKGF/terraform-neptune/commit/8188fb6c2109def1b21cce14649d71be8db61162)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.13](https://github.com/EKGF/terraform-neptune/compare/0.0.12..0.0.13) - 2024-02-19
#### Refactoring
- fixing some warnings - ([aacc4d2](https://github.com/EKGF/terraform-neptune/commit/aacc4d285c016790479b4b2796456d9344ea36cf)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.12](https://github.com/EKGF/terraform-neptune/compare/0.0.11..0.0.12) - 2024-02-19
#### Documentation
- updated README.md - ([127f94d](https://github.com/EKGF/terraform-neptune/commit/127f94db71ca56607b873e2c661b75f811fe66f4)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.11](https://github.com/EKGF/terraform-neptune/compare/0.0.10..0.0.11) - 2024-02-19
#### Bug Fixes
- increased operation timeout per AWS call from 5 to 60 seconds - ([bbcdf21](https://github.com/EKGF/terraform-neptune/commit/bbcdf218061f89d817e003ae68725ebba8527573)) - [@jgeluk](https://github.com/jgeluk)
#### Documentation
- updated README.md - ([c7cfec7](https://github.com/EKGF/terraform-neptune/commit/c7cfec7101c978471df511d385293c0866a8a69b)) - [@jgeluk](https://github.com/jgeluk)
#### Refactoring
- minor changes - ([cc68acf](https://github.com/EKGF/terraform-neptune/commit/cc68acfb077f703ca5a45f0855112732a9069076)) - [@jgeluk](https://github.com/jgeluk)
- relying on ekg-rs repo for generic crates - ([715a3f8](https://github.com/EKGF/terraform-neptune/commit/715a3f8f18f3815f79cc259b7dd766f721607e60)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.10](https://github.com/EKGF/terraform-neptune/compare/0.0.9..0.0.10) - 2024-01-18
#### Features
- **(lambda)** less noisy error messages - ([ebfd419](https://github.com/EKGF/terraform-neptune/commit/ebfd419d3b57acc1a46795951aa05005c39696bc)) - [@jgeluk](https://github.com/jgeluk)
- **(lfn-load)** added a "suggested wait time" to the output of the load lambda function for the state machine to pick up as the wait time before retrying - ([1174a49](https://github.com/EKGF/terraform-neptune/commit/1174a4998b23f50468f1552375620a1503a3bad9)) - [@jgeluk](https://github.com/jgeluk)
#### Miscellaneous Chores
- intermediate commit - ([bb39760](https://github.com/EKGF/terraform-neptune/commit/bb39760ba7eccc8bd15ea82ee89dbc2046b044ce)) - [@jgeluk](https://github.com/jgeluk)
#### Refactoring
- **(general)** big refactor - ([c7afa51](https://github.com/EKGF/terraform-neptune/commit/c7afa51ad37c8f543ceee70317f49f39c9aae831)) - [@jgeluk](https://github.com/jgeluk)
- **(namespace)** removed dash from prefix name due to limitation in visual studio code - ([5f2dcd0](https://github.com/EKGF/terraform-neptune/commit/5f2dcd042439951b6f095bf5af25c56684f17f68)) - [@jgeluk](https://github.com/jgeluk)
- many small unrelated changes - ([248bd47](https://github.com/EKGF/terraform-neptune/commit/248bd4736d026d91d8dd8cd21321d2942819042e)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.9](https://github.com/EKGF/terraform-neptune/compare/0.0.8..0.0.9) - 2024-01-10
#### Features
- **(sparql)** donated agnos.ai code to EKGF (crates ekg-sparql and ekg-namespace). ekg-lfn-load crate now updates Neptune using SPARQL with load request information. - ([6a5f588](https://github.com/EKGF/terraform-neptune/commit/6a5f5889186a508704325d738eba61bc809a3bc0)) - [@jgeluk](https://github.com/jgeluk)
#### Refactoring
- **(neptune)** renamed LoadRequest - ([f9207a4](https://github.com/EKGF/terraform-neptune/commit/f9207a426f60a7a513749c26bfc496152dba55ea)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.8](https://github.com/EKGF/terraform-neptune/compare/0.0.7..0.0.8) - 2024-01-05
#### Miscellaneous Chores
- **(ci)** updated .gitignore - ([ba333b5](https://github.com/EKGF/terraform-neptune/commit/ba333b5cd92785fbcd1cbf5ac5f98e925a54d22e)) - [@jgeluk](https://github.com/jgeluk)
- intermediate save, does not work - ([ebb5b58](https://github.com/EKGF/terraform-neptune/commit/ebb5b58a875b8cd665d36f8b167aa59c926752fa)) - [@jgeluk](https://github.com/jgeluk)
#### Refactoring
- **(lambda)** removed old python code - ([b5a4da0](https://github.com/EKGF/terraform-neptune/commit/b5a4da0b464f8dbd23a8df615ee00f7fa1dce51b)) - [@jgeluk](https://github.com/jgeluk)
- **(lambda)** reimplemented check lambda function in Rust - ([cd70c75](https://github.com/EKGF/terraform-neptune/commit/cd70c7583c914d40177edb041b3eef268e6316af)) - [@jgeluk](https://github.com/jgeluk)
- **(lambda)** reimplemented invoke and load lambda functions in Rust - ([0766c59](https://github.com/EKGF/terraform-neptune/commit/0766c5918a58101e05cf62ffd04d5372c045ba27)) - [@jgeluk](https://github.com/jgeluk)
- **(lambda)** minor changes - ([2ef05cb](https://github.com/EKGF/terraform-neptune/commit/2ef05cb69bcda2604df50ed5dbd625c45f8e9205)) - [@jgeluk](https://github.com/jgeluk)
- **(lambda)** boilerplate for rust version of lambdas - ([0c57cc3](https://github.com/EKGF/terraform-neptune/commit/0c57cc3113bafb2387af6161696dc56bfad28337)) - [@jgeluk](https://github.com/jgeluk)
- **(makefiles)** updated makefiles - ([157eabb](https://github.com/EKGF/terraform-neptune/commit/157eabb3b0e45ad117af042173828875e4bc79d0)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.7](https://github.com/EKGF/terraform-neptune/compare/0.0.6..0.0.7) - 2023-12-08
#### Refactoring
- **(rdf-load)** aws step function works now - ([ce2a737](https://github.com/EKGF/terraform-neptune/commit/ce2a737c05ff2e570f4a857003d28da7176e4b8b)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.6](https://github.com/EKGF/terraform-neptune/compare/0.0.5..0.0.6) - 2023-11-28
#### Refactoring
- **(rdf-load)** added new env vars - ([cd46ea0](https://github.com/EKGF/terraform-neptune/commit/cd46ea021a21411d63bd9e53bde6b115c5d3d615)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.5](https://github.com/EKGF/terraform-neptune/compare/0.0.4..0.0.5) - 2023-11-24
#### Bug Fixes
- **(build)** Fixed building of python code - ([b1bfb1b](https://github.com/EKGF/terraform-neptune/commit/b1bfb1be691f644d1c638e182d3ae850ede3047e)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.4](https://github.com/EKGF/terraform-neptune/compare/0.0.3..0.0.4) - 2023-11-23
#### Refactoring
- **(build)** minor issue in build script - ([1c29900](https://github.com/EKGF/terraform-neptune/commit/1c29900584cb7f2c52d24d4688b85dcf2fe188b3)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.3](https://github.com/EKGF/terraform-neptune/compare/0.0.2..0.0.3) - 2023-11-20
#### Refactoring
- **(security-group)** minor change - ([b7a07fe](https://github.com/EKGF/terraform-neptune/commit/b7a07feba9e38843549d9dba3e6b065eb7dca4c7)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.2](https://github.com/EKGF/terraform-neptune/compare/0.0.1..0.0.2) - 2023-11-19
#### Continuous Integration
- **(terraform)** test github actions build - ([863f726](https://github.com/EKGF/terraform-neptune/commit/863f726f106a11b5ca555411d8088375abcce7e0)) - [@jgeluk](https://github.com/jgeluk)
#### Refactoring
- **(archive_zip)** better way to build the zip file - ([b82d847](https://github.com/EKGF/terraform-neptune/commit/b82d84709d81461fb8b8366d8e329c03005e5651)) - [@jgeluk](https://github.com/jgeluk)

- - -

## [0.0.1](https://github.com/EKGF/terraform-neptune/compare/bb5cbe78b8805873b3351b493d83168043cf156a..0.0.1) - 2023-11-12

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).