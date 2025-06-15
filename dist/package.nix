{
  lib,
  fetchFromGitHub,
  rustPlatform,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "tokenize";
  version = "0.1.1";

  src = fetchFromGitHub {
    owner = "DefinitelyNotSimon13";
    repo = "tokenize";
    tag = finalAttrs.version;
    hash = "sha256-3k4nLI7i7wk/u4WDN35YO6N38cfXB76vGiAZByozjBw=";
  };

  cargoHash = "sha256-9dB15miojGSZoT7Z2tLEeU41xVtfedAIGKJDvhmYbb4=";

  meta = {
    description = "Tokenize your codebase blazingly fast into a single file for LLM Context 🚀🤖 ";
    homepage = "https://github.com/DefinitelyNotSimon13/tokenize";
    license = lib.licenses.mit;
  };
})
