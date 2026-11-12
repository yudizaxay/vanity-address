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
  homepage "https://vanity-address.online/"
  url "https://github.com/yudizaxay/vanity-address/archive/refs/tags/v0.8.0.tar.gz"
  sha256 "c34ba14a1eda4b6b5084126676f870c20ee65d8023f4efd9c638236ac64cac19"
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
