with import <nixpkgs> { };
{ lib, fetchFromGitHub, rustPlatform, fetchgit }:

rustPlatform.buildRustPackage rec {
  pname = "mpd-notification-rs";
  version = "0.1.0";

  cargoLock = {
    lockFile = ./Cargo.lock;
    #allowBuiltinFetchGit = true;
  };

  src = fetchgit {
    url = "http://optiplex.home/git/mpd-rust-utils/mpd-notification-rs";
    #owner = "mpd-rust-utils";
    #repo = "mpd-notification-rs";
    #rev = version;
    #hash = "sha256-+s5RBC3XSgb8omTbUNLywZnP6jSxZBKSS1BmXOjRF8M=";
    hash = "sha256-xwhnmS01rKPvaxaORVR+EuiAYLkL4VeWKUvZW466vSA=";
  };

  cargoHash = lib.fakeHash;

  meta = with lib; {
    description = "MPD Notification daemon";
    homepage = "http://optiplex.home/git/mpd-rust-utils/mpd-notification-rs";
    license = licenses.mit;
    maintainers = [ ];
  };
}
