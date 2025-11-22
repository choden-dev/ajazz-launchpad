# `config-frontend` workspace

This is an [`iced.rs`] GUI project which communicates with the `backend-process` to configure the different settings of
the launchpad; as such it depends on `backend-process` running to function. It uses utils from `messaging` to handle the
socketing behaviour.

[`iced.rs`]: https://iced.rs/