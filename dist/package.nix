{
  lib,
  fetchFromGitHub,
  rustPlatform,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "tokenize";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "DefinitelyNotSimon13";
    repo = "tokenize";
    tag = finalAttrs.version;
    hash = "sha256-N02yxkuHCl7HFodDiwKMMo0Thm4JohjCPePVO5aTy2s=";
  };

  cargoHash = "sha256-9dB15miojGSZoT7Z2tLEeU41xVtfedAIGKJDvhmYbb4=";

  meta = {
    description = "Tokenize your codebase blazingly fast into a single file for LLM Context 🚀🤖 ";
    homepage = "https://github.com/DefinitelyNotSimon13/tokenize";
    license = lib.licenses.mit;
  };
})
