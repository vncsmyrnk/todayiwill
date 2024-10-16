class App < Formula
  desc "A CLI reminder app that offers a simple yet powerful solution to enhance productivity and ensure that you stay on top of your daily responsibilities"
  homepage "https://github.com/vncsmyrnk/todayiwill"
  url "https://github.com/vncsmyrnk/todayiwill/releases/download/v0.5.3/todayiwill"
  sha256 "36f63e5b6ec2f5ecc0b30b9927ce5923bb09c3f268bdec277f5859ab7cbc3bc3"
  license "MIT"
  version "0.5.3"

  def install
    bin.install "todayiwill"
  end

  test do
    system "#{bin}/todayiwill", "--version"
  end
end
