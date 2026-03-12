import { createHash } from "node:crypto";
import { appendFile, writeFile } from "node:fs/promises";

const [version] = process.argv.slice(2);

if (!version) {
  console.error("Usage: node update-homebrew.mjs <version>");
  process.exit(1);
}

const tarballUrl = `https://github.com/SeokminHong/vltl/archive/refs/tags/v${version}.tar.gz`;

const response = await fetch(tarballUrl);
if (!response.ok) {
  console.error(
    `Failed to download tarball: ${response.status} ${response.statusText}`,
  );
  process.exit(1);
}

const buffer = Buffer.from(await response.arrayBuffer());
const sha256 = createHash("sha256").update(buffer).digest("hex");

const formula = `class Vltl < Formula
  desc "Fix a 2-set Korean typo to English"
  homepage "https://github.com/SeokminHong/vltl"
  url "${tarballUrl}"
  sha256 "${sha256}"
  license "MIT"

  head "https://github.com/SeokminHong/vltl.git", branch: "main"

  depends_on "rust" => :build
  depends_on "fish"

  def install
    system "cargo", "install", "vltl", *std_cargo_args
  end

  test do
    assert_match "vltl ${version}", shell_output("#{bin}/vltl --version")
  end
end
`;

await writeFile("/tmp/vltl.rb", formula);

const githubOutput = process.env.GITHUB_OUTPUT;
if (githubOutput) {
  await appendFile(githubOutput, `sha256=${sha256}\n`);
}

console.log(`Generated formula for v${version} (SHA256: ${sha256})`);
