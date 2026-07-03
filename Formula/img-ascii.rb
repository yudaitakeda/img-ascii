class ImgAscii < Formula
  desc "Convert images into ASCII art"
  homepage "https://github.com/yudaitakeda/img-ascii"
  url "https://github.com/yudaitakeda/img-ascii/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_RELEASE_TARBALL_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")

    generate_completions_from_executable(
      bin/"img-ascii",
      "completions",
      shells: [:bash, :zsh, :fish, :powershell, :elvish],
    )
  end

  test do
    assert_match "img-ascii", shell_output("#{bin}/img-ascii --help")
  end
end
