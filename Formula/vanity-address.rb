# Homebrew formula for vanity-address CLI
#
# End users (after tap is published):
#   brew tap yudizaxay/tap
#   brew trust yudizaxay/tap
#   brew install vanity-address
#
# From this repo (no tap):
#   brew install --build-from-source ./Formula/vanity-address.rb
#
# Maintainers: ./scripts/update-homebrew-formula.sh 0.3.7   (tag must exist on GitHub first)
# See docs/HOMEBREW.md

class VanityAddress < Formula
  desc "Fast, local multi-chain vanity address generator"
  homepage "https://github.com/yudizaxay/vanity-address"
  url "https://github.com/yudizaxay/vanity-address/archive/refs/tags/v0.3.7.tar.gz"
  sha256 "4081c3241620b36e0b1443d00ca68b1e61edca691e457b01225600e4b8d8dfdd"
  license "MIT"
  head "https://github.com/yudizaxay/vanity-address.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: "vanity-address")
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/vanity-address --version")
    assert_match "vanity-address", shell_output("#{bin}/vanity-address --help")
  end
end
