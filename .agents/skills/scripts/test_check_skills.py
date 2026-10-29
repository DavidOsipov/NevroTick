#!/usr/bin/env python3
"""Behavioral regression tests for the skill-package checker's CLI."""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


CHECKER = Path(__file__).resolve().with_name("check_skills.py")
SKILL_NAME = "probe-skill"
SKILL_PATH = f"testing/{SKILL_NAME}/SKILL.md"


class SkillFixture(unittest.TestCase):
    def setUp(self) -> None:
        temporary = tempfile.TemporaryDirectory(prefix="nevrotick-skill-check-")
        self.addCleanup(temporary.cleanup)
        self.temporary_root = Path(temporary.name)
        self.skills_root = self.temporary_root / ".agents" / "skills"
        self.skill_directory = self.skills_root / "testing" / SKILL_NAME
        (self.skill_directory / "agents").mkdir(parents=True)

        skill_text = (
            "---\n"
            f"name: {json.dumps(SKILL_NAME)}\n"
            'description: "Exercise the package checker with a valid fixture."\n'
            "---\n\n"
            "# Probe skill\n\n"
            "Check a bounded package fixture.\n"
        )
        (self.skill_directory / "SKILL.md").write_text(skill_text, encoding="utf-8")
        inventory = {
            "skills": [
                {
                    "source": SKILL_PATH,
                    "target": SKILL_PATH,
                    "source_sha256": hashlib.sha256(skill_text.encode("utf-8")).hexdigest(),
                }
            ]
        }
        (self.skills_root / "ADAPTATION.json").write_text(
            json.dumps(inventory), encoding="utf-8"
        )

    def run_checker(self, prompt: str) -> subprocess.CompletedProcess[str]:
        interface = {
            "display_name": "Probe Skill",
            "short_description": "Check a bounded skill package fixture",
            "default_prompt": prompt,
        }
        (self.skill_directory / "agents" / "openai.yaml").write_text(
            "interface:\n"
            + "".join(f"  {key}: {json.dumps(value)}\n" for key, value in interface.items()),
            encoding="utf-8",
        )
        environment = os.environ.copy()
        environment["PYTHONDONTWRITEBYTECODE"] = "1"
        return subprocess.run(
            [sys.executable, "-B", str(CHECKER), "--root", str(self.skills_root)],
            cwd=self.temporary_root,
            env=environment,
            capture_output=True,
            text=True,
            check=False,
            timeout=10,
        )

    def assert_prompt_passes(self, prompt: str) -> None:
        result = self.run_checker(prompt)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(result.stderr, "")
        self.assertEqual(
            result.stdout,
            "PASS: 1 skills; metadata, UI prompts, local links and skill inventories.\n",
        )

    def assert_prompt_rejected(self, prompt: str) -> None:
        result = self.run_checker(prompt)
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertEqual(result.stderr, "")
        self.assertEqual(
            result.stdout,
            f"FAIL: {SKILL_PATH}: UI prompt does not invoke ${SKILL_NAME}\n"
            "Failed with 1 error(s) across 1 skills.\n",
        )

class PromptInvocationTests(SkillFixture):
    def test_exact_invocation_passes(self) -> None:
        self.assert_prompt_passes("Use $probe-skill to verify this package.")

    def test_normal_punctuation_after_invocation_passes(self) -> None:
        for prompt in (
            "Use $probe-skill.",
            "Use $probe-skill, then inspect the result.",
            "Use ($probe-skill).",
            "Use `$probe-skill` to verify this package.",
        ):
            with self.subTest(prompt=prompt):
                self.assert_prompt_passes(prompt)

    def test_exact_invocation_among_other_invocations_passes(self) -> None:
        for prompt in (
            "Use $other-skill, then $probe-skill.",
            "Use $probe-skill-missing, then $probe-skill.",
        ):
            with self.subTest(prompt=prompt):
                self.assert_prompt_passes(prompt)

    def test_hyphen_suffix_is_not_an_exact_invocation(self) -> None:
        self.assert_prompt_rejected("Use $probe-skill-missing to verify this package.")

    def test_attached_letter_is_not_an_exact_invocation(self) -> None:
        self.assert_prompt_rejected("Use $probe-skillx to verify this package.")

    def test_attached_digit_is_not_an_exact_invocation(self) -> None:
        self.assert_prompt_rejected("Use $probe-skill7 to verify this package.")

    def test_underscore_suffix_is_not_an_exact_invocation(self) -> None:
        self.assert_prompt_rejected("Use $probe-skill_missing to verify this package.")

    def test_uppercase_suffix_is_not_an_exact_invocation(self) -> None:
        self.assert_prompt_rejected("Use $probe-skillX to verify this package.")

    def test_different_invocation_is_rejected(self) -> None:
        self.assert_prompt_rejected("Use $other-skill to verify this package.")


class LocalInventoryTests(SkillFixture):
    def setUp(self) -> None:
        super().setUp()
        self.local = self.skills_root / "testing" / "local-skill"
        (self.local / "agents").mkdir(parents=True)
        (self.local / "SKILL.md").write_text(
            '---\nname: "local-skill"\n'
            'description: "Exercise an original locally authored workflow."\n'
            '---\n\n# Local skill\n', encoding="utf-8",
        )
        (self.local / "agents/openai.yaml").write_text(
            'interface:\n  display_name: "Local skill"\n'
            '  short_description: "Check an original local workflow"\n'
            '  default_prompt: "Use $local-skill for this workflow."\n', encoding="utf-8",
        )
        self.entry = {"name": "local-skill", "target": "testing/local-skill/SKILL.md",
                      "origin": "locally-authored"}
        self.write_local()

    def write_local(self, entries=None) -> None:
        (self.skills_root / "LOCAL.json").write_text(
            json.dumps({"schema_version": 1, "skills": entries if entries is not None else [self.entry]}),
            encoding="utf-8",
        )

    def assert_local_rejected(self, message: str) -> None:
        result = self.run_checker("Use $probe-skill for the adapted workflow.")
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertEqual(result.stderr, "")
        self.assertIn(message, result.stdout)
        self.assertNotIn("PASS:", result.stdout)

    def test_original_leaf_with_local_inventory_passes(self) -> None:
        result = self.run_checker("Use $probe-skill for the adapted workflow.")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(result.stderr, "")
        self.assertIn("PASS: 2 skills", result.stdout)

    def test_unlisted_local_leaf_is_rejected(self) -> None:
        (self.skills_root / "LOCAL.json").unlink()
        self.assert_local_rejected("inventory mismatch: unlisted=['testing/local-skill/SKILL.md']")

    def test_missing_local_leaf_is_rejected(self) -> None:
        (self.local / "SKILL.md").unlink()
        self.assert_local_rejected("missing=['testing/local-skill/SKILL.md']")

    def test_duplicate_local_target_is_rejected(self) -> None:
        self.write_local([self.entry, self.entry])
        self.assert_local_rejected("duplicate local skill target")

    def test_traversal_and_absolute_local_targets_are_rejected(self) -> None:
        for target in ("testing/../local-skill/SKILL.md", "/testing/local-skill/SKILL.md",
                       "testing//local-skill/SKILL.md", "upstream/local-skill/SKILL.md"):
            with self.subTest(target=target):
                self.entry["target"] = target
                self.write_local()
                self.assert_local_rejected("invalid local skill target")

    def test_local_inventory_cannot_claim_an_adapted_leaf(self) -> None:
        self.entry = {"name": SKILL_NAME, "target": SKILL_PATH, "origin": "locally-authored"}
        self.write_local()
        self.assert_local_rejected("local and adapted/upstream skill targets overlap")

    def test_local_inventory_has_no_fabricated_donor_fields(self) -> None:
        self.entry["source_sha256"] = "a" * 64
        self.write_local()
        self.assert_local_rejected("unsupported local inventory fields")

    def test_local_leaf_symlink_cannot_escape_root(self) -> None:
        outside = self.temporary_root / "outside-skill.md"
        outside.write_text((self.local / "SKILL.md").read_text(), encoding="utf-8")
        (self.local / "SKILL.md").unlink()
        (self.local / "SKILL.md").symlink_to(outside)
        self.assert_local_rejected("local skill target escapes root")


class UpstreamInventoryTests(SkillFixture):
    def setUp(self) -> None:
        super().setUp()
        self.external = self.skills_root / "upstream" / "external-skill"
        (self.external / "agents").mkdir(parents=True)
        (self.external / "SKILL.md").write_text(
            '---\nname: "external-skill"\n'
            'description: "Diagnose a specific upstream technique."\n'
            'license: "BSD-3-Clause"\n---\n\n# External skill\n',
            encoding="utf-8",
        )
        (self.external / "LICENSE").write_text("Fixture license notice.\n", encoding="utf-8")
        (self.external / "reference.md").write_text("# Reviewed reference\n", encoding="utf-8")
        (self.external / "agents/openai.yaml").write_text(
            'interface:\n  display_name: "External skill"\n'
            '  short_description: "Diagnose one upstream technique"\n'
            '  default_prompt: "Use $external-skill for this diagnosis."\n',
            encoding="utf-8",
        )
        self.entry = {
            "name": "external-skill",
            "target": "upstream/external-skill/SKILL.md",
            "repository": "example/rust-skills",
            "commit": "a" * 40,
            "source_path": "skills/external-skill",
            "source_files": {"skills/external-skill/SKILL.md": "b" * 64},
            "license": "BSD-3-Clause",
            "license_file": "upstream/external-skill/LICENSE",
            "files": {
                str(p.relative_to(self.skills_root)): hashlib.sha256(p.read_bytes()).hexdigest()
                for p in self.external.rglob("*")
                if p.is_file()
            },
        }
        self.write_upstream()

    def write_upstream(self) -> None:
        (self.skills_root / "UPSTREAM.json").write_text(
            json.dumps({"skills": [self.entry]}), encoding="utf-8"
        )

    def assert_inventory_rejected(self, message: str) -> None:
        result = self.run_checker("Use $probe-skill for the original workflow.")
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertEqual(result.stderr, "")
        self.assertIn(message, result.stdout)
        self.assertNotIn("PASS:", result.stdout)

    def test_pinned_reviewed_upstream_package_passes(self) -> None:
        result = self.run_checker("Use $probe-skill for the original workflow.")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(result.stderr, "")
        self.assertIn("PASS: 2 skills", result.stdout)

    def test_mutable_branch_is_not_a_commit_pin(self) -> None:
        self.entry["commit"] = "main"
        self.write_upstream()
        self.assert_inventory_rejected("invalid upstream commit")

    def test_changed_upstream_reference_is_rejected(self) -> None:
        (self.external / "reference.md").write_text("Changed after review.\n", encoding="utf-8")
        self.assert_inventory_rejected("upstream file changed")

    def test_unlisted_file_in_upstream_leaf_is_rejected(self) -> None:
        (self.external / "unexpected.txt").write_text("Not in the reviewed inventory.\n", encoding="utf-8")
        self.assert_inventory_rejected("upstream file inventory mismatch")

    def test_missing_license_is_rejected(self) -> None:
        (self.external / "LICENSE").unlink()
        del self.entry["files"][self.entry["license_file"]]
        self.write_upstream()
        self.assert_inventory_rejected("missing upstream license file")

    def test_license_metadata_must_match_provenance(self) -> None:
        self.entry["license"] = "MIT"
        self.write_upstream()
        self.assert_inventory_rejected("upstream license metadata mismatch")

    def test_upstream_file_cannot_escape_its_leaf(self) -> None:
        self.entry["files"]["../outside.txt"] = "c" * 64
        self.write_upstream()
        self.assert_inventory_rejected("invalid upstream file path")

    def test_unregistered_upstream_skill_is_rejected(self) -> None:
        (self.skills_root / "UPSTREAM.json").unlink()
        self.assert_inventory_rejected("inventory mismatch")


if __name__ == "__main__":
    unittest.main()
