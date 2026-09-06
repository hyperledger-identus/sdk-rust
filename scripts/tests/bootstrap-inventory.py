#!/usr/bin/env python3

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
CHECKER = REPOSITORY_ROOT / "scripts/check-bootstrap-inventory.py"


class BootstrapInventoryTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        for relative in (
            ".github/CODEOWNERS",
            ".github/ISSUE_TEMPLATE/component-change.yml",
            ".github/ISSUE_TEMPLATE/delivery-task.yml",
            ".github/pull_request_template.md",
            "CODE_OF_CONDUCT.md",
            "CONTRIBUTING.md",
            "DCO.md",
            "GOVERNANCE.md",
            "LICENSE",
            "MAINTAINERS.md",
            "RELEASING.md",
            "SECURITY.md",
            "Cargo.toml",
            "docs/architecture/sdk-bootstrap-inventory.md",
            "docs/architecture/sdk-bootstrap-inventory.toml",
            "docs/governance/repository-settings.md",
        ):
            target = self.root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(REPOSITORY_ROOT / relative, target)
        shutil.copytree(REPOSITORY_ROOT / "crates", self.root / "crates")

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def run_checker(self) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(CHECKER), str(self.root)],
            check=False,
            capture_output=True,
            text=True,
        )

    def inventory_text(self) -> str:
        return (self.root / "docs/architecture/sdk-bootstrap-inventory.toml").read_text(
            encoding="utf-8"
        )

    def write_inventory(self, text: str) -> None:
        (self.root / "docs/architecture/sdk-bootstrap-inventory.toml").write_text(
            text, encoding="utf-8"
        )

    def package_blocks(self) -> tuple[str, list[str]]:
        parts = self.inventory_text().split("[[packages]]")
        return parts[0], parts[1:]

    def test_canonical_inventory_passes(self) -> None:
        result = self.run_checker()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("contract passed (17 packages)", result.stdout)

    def test_missing_package_fails(self) -> None:
        header, blocks = self.package_blocks()
        blocks = [block for block in blocks if 'name           = "identus-wallet"' not in block]
        self.write_inventory(header + "".join(f"[[packages]]{block}" for block in blocks))
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("inventory missing workspace packages: identus-wallet", result.stderr)

    def test_duplicate_package_fails(self) -> None:
        header, blocks = self.package_blocks()
        wallet = next(block for block in blocks if 'name           = "identus-wallet"' in block)
        self.write_inventory(
            header + "".join(f"[[packages]]{block}" for block in [*blocks, wallet])
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("duplicate inventory package: identus-wallet", result.stderr)

    def test_unknown_package_fails(self) -> None:
        self.write_inventory(
            self.inventory_text().replace(
                'name           = "identus-agent"', 'name           = "identus-unknown"', 1
            )
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("inventory has unknown workspace packages: identus-unknown", result.stderr)

    def test_path_drift_fails(self) -> None:
        self.write_inventory(
            self.inventory_text().replace(
                'path           = "crates/agent"', 'path           = "crates/wallet"', 1
            )
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("duplicate inventory package path: crates/wallet", result.stderr)

    def test_rulebook_layer_drift_fails(self) -> None:
        self.write_inventory(
            self.inventory_text().replace(
                'layer          = "orchestration"\nclassification = "placeholder"',
                'layer          = "foundation"\nclassification = "placeholder"',
                1,
            )
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("inventory layer foundation does not match rulebook orchestration", result.stderr)

    def test_workspace_publish_denial_is_required(self) -> None:
        manifest = self.root / "Cargo.toml"
        manifest.write_text(
            manifest.read_text(encoding="utf-8").replace("publish      = false\n", "", 1),
            encoding="utf-8",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("workspace.package.publish must be false", result.stderr)

    def test_member_publish_inheritance_is_required(self) -> None:
        manifest = self.root / "crates/agent/Cargo.toml"
        manifest.write_text(
            manifest.read_text(encoding="utf-8").replace(
                "publish.workspace      = true\n", "", 1
            ),
            encoding="utf-8",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("identus-agent: package.publish.workspace must be true", result.stderr)

    def test_in_tree_path_dependency_must_be_an_explicit_member(self) -> None:
        payload = self.root / "payload"
        (payload / "src").mkdir(parents=True)
        (payload / "Cargo.toml").write_text(
            '[package]\nname = "payload"\nversion = "0.0.0"\nedition = "2024"\npublish = true\n',
            encoding="utf-8",
        )
        (payload / "src/lib.rs").write_text("pub struct Payload;\n", encoding="utf-8")
        manifest = self.root / "crates/core/Cargo.toml"
        manifest.write_text(
            manifest.read_text(encoding="utf-8").replace(
                "[dependencies]", '[dependencies]\npayload = { path = "../../payload" }', 1
            ),
            encoding="utf-8",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "crates/core: in-tree path dependency payload must be an explicit workspace member: payload",
            result.stderr,
        )

    def test_root_package_is_an_explicit_member(self) -> None:
        manifest = self.root / "Cargo.toml"
        manifest.write_text(
            manifest.read_text(encoding="utf-8")
            + '\n[package]\nname = "root-package"\nversion = "0.0.0"\nedition = "2024"\npublish = true\n',
            encoding="utf-8",
        )
        (self.root / "src").mkdir()
        (self.root / "src/lib.rs").write_text(
            "pub struct RootPackage;\n", encoding="utf-8"
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "root-package: package.publish.workspace must be true", result.stderr
        )
        self.assertIn(
            "inventory missing workspace packages: root-package", result.stderr
        )

    def test_placeholder_dependency_drift_fails(self) -> None:
        manifest = self.root / "crates/agent/Cargo.toml"
        manifest.write_text(
            manifest.read_text(encoding="utf-8").replace(
                "identus-core.workspace = true",
                "identus-core.workspace = true\nidentus-crypto.workspace = true",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "identus-agent: placeholder dependencies must be exactly identus-core",
            result.stderr,
        )

    def test_placeholder_core_dependency_must_inherit_workspace(self) -> None:
        manifest = self.root / "crates/agent/Cargo.toml"
        manifest.write_text(
            manifest.read_text(encoding="utf-8").replace(
                "identus-core.workspace = true", 'identus-core = "0.0.0"', 1
            ),
            encoding="utf-8",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "identus-agent: identus-core must inherit the workspace dependency",
            result.stderr,
        )

    def test_placeholder_custom_build_script_fails(self) -> None:
        manifest = self.root / "crates/agent/Cargo.toml"
        manifest.write_text(
            manifest.read_text(encoding="utf-8").replace(
                '[package]\nname                   = "identus-agent"',
                '[package]\nname                   = "identus-agent"\nbuild                  = "build-script"',
                1,
            ),
            encoding="utf-8",
        )
        (self.root / "crates/agent/build-script").write_text(
            "fn main() {}\n", encoding="utf-8"
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "identus-agent: placeholder must not declare package.build",
            result.stderr,
        )

    def test_placeholder_custom_library_target_fails(self) -> None:
        manifest = self.root / "crates/agent/Cargo.toml"
        manifest.write_text(
            manifest.read_text(encoding="utf-8")
            + '\n[lib]\npath = "payload"\n',
            encoding="utf-8",
        )
        (self.root / "crates/agent/payload").write_text(
            "pub struct Agent;\n", encoding="utf-8"
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "identus-agent: placeholder must not declare lib",
            result.stderr,
        )

    def test_placeholder_custom_test_and_bench_targets_fail(self) -> None:
        manifest = self.root / "crates/agent/Cargo.toml"
        original = manifest.read_text(encoding="utf-8")
        for target in ("test", "bench"):
            with self.subTest(target=target):
                manifest.write_text(
                    original + f'\n[[{target}]]\nname = "payload"\npath = "payload"\n',
                    encoding="utf-8",
                )
                (self.root / "crates/agent/payload").write_text(
                    "fn main() {}\n", encoding="utf-8"
                )
                result = self.run_checker()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(
                    f"identus-agent: placeholder must not declare {target}",
                    result.stderr,
                )

    def test_placeholder_executable_documentation_fails(self) -> None:
        source = self.root / "crates/agent/src/lib.rs"
        original = source.read_text(encoding="utf-8")
        for documentation in (
            "//! ```rust\n//! panic!(\"executed\");\n//! ```\n",
            "//!\n//!     panic!(\"executed\");\n",
        ):
            with self.subTest(documentation=documentation):
                source.write_text(documentation + original, encoding="utf-8")
                result = self.run_checker()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(
                    "identus-agent: placeholder documentation must not contain executable code blocks",
                    result.stderr,
                )

    def test_placeholder_nested_doctest_fences_fail(self) -> None:
        source = self.root / "crates/agent/src/lib.rs"
        original = source.read_text(encoding="utf-8")
        for container in ("> ", "- "):
            with self.subTest(container=container):
                source.write_text(
                    f"//! {container}```rust\n//! panic!(\"executed\");\n//! {container}```\n"
                    + original,
                    encoding="utf-8",
                )
                result = self.run_checker()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(
                    "identus-agent: placeholder documentation must not contain executable code blocks",
                    result.stderr,
                )

    def test_placeholder_source_drift_fails(self) -> None:
        source = self.root / "crates/agent/src/lib.rs"
        source.write_text(source.read_text(encoding="utf-8") + "\npub struct Agent;\n", encoding="utf-8")
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("identus-agent: placeholder source exceeds", result.stderr)

    def test_missing_governance_document_fails(self) -> None:
        (self.root / "SECURITY.md").unlink()
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("governance document is missing or empty: SECURITY.md", result.stderr)

    def test_live_controls_cannot_be_claimed_active(self) -> None:
        self.write_inventory(
            self.inventory_text().replace(
                'live_controls_state = "external-action-required"',
                'live_controls_state = "active"',
                1,
            )
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "repository.live_controls_state must be external-action-required",
            result.stderr,
        )


if __name__ == "__main__":
    unittest.main()
