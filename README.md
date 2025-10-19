# OicFoxFuckery

This is a very basic application that provides an API to annotate a given OIC
beer league hockey calendar with any overlapping shows at the nearby Fox Theater.
You can subscribe to this calendar so that you can automatically learn if a show
at the Fox is going to fuck you over.

This is a super basic app just to learn a bit of rust, I do not know what I am doing.

## Development Details

* [devenv](https://devenv.sh)
  * `devenv build`
  * `devenv test`
  * `devenv watch`
  * `devenv clippy`
  * `devenv tasks run container:local -v` - Build container and copy to local docker daemon
  * `REGISTRY_API_KEY=<key> devenv tasks run container:registry -v` - Build container and copy to remote registry
  * `devenv container run prod` - Works, but container is quite large
  * `devenv container build prod` - Works, but container is quite large
* [Loco](https://loco.rs)
  * `cargo loco start`
* Nix output binary
  * `TM_API_KEY=<ticketmaster API key> oic-fox-fuckery-cli start`
