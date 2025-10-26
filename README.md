## AJAZZ Launchpad Ecosystem

This is a repo developed for my _personal_ tooling and software developed for
the [`AJAZZ` AKP05 launchpad].

> [!WARNING]
> I am in no way affiliated with `AJAZZ`! All development done in this repo is done for the sake of my own convenience
> and leisure!

### Main components

Each of the components in this repo is grouped into separate workspaces. Below is a high level overview of what each is
responsible for (may grow in the future):

- [`firmware-api`]
    - An API to directly communicate with the launchpad over USB

### Rust guidelines

Please use `1.89.0` or above!

Also be aware that we are also using the [new module convention] for Rust instead of the `mod.rs` format

[`AJAZZ` AKP05 launchpad]: https://ajazzbrand.com/products/ajazz-akp05-desk-controller

[new module convention]: https://www.reddit.com/media?url=https%3A%2F%2Fi.redd.it%2F1yy98srxyvx81.png

[`firmware-api`]: https://github.com/choden-dev/ajazz-launchpad/tree/main/firmware-api
