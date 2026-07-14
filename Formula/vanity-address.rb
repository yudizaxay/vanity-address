# Homebrew formula for vanity-address CLI
#
# Usage (after cloning this repo or adding as a tap):
#   brew install --build-from-source ./Formula/vanity-address.rb
#
# Or from a tagged release tarball:
#   brew install yudizaxay/tap/vanity-address
#
# Update `url` and `sha256` when cutting a new release.

class VanityAddress < Formula
  desc "Fast, local multi-chain vanity address generator"
  homepage "https://github.com/yudizaxay/vanity-address"
  url "https://github.com/yudizaxay/vanity-address/archive/refs/tags/v0.3.2.tar.gz"
  # Run: curl -L <url> | shasum -a 256
  sha256 "REPLACE_ON_RELEASE"
  license "MIT"
  head "https://github.com/yudizaxay/vanity-address.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_install_args(path: "vanity-address")
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/vanity-address --version")
    assert_match "vanity-address", shell_output("#{bin}/vanity-address --help")
  end
end
