"""Ubuntu bundles preserve authenticated native root peers without changing Arch."""
import configparser
import json
from pathlib import Path
import tarfile
import tempfile
from types import SimpleNamespace
import unittest
from scripts.build_bundle import build

REPO = Path(__file__).resolve().parents[1]


def arguments(root):
    ui = root / 'ui'
    for relative in ('bin/forge-store-ui', 'share/applications/forge-store.desktop',
                     'share/icons/hicolor/scalable/apps/forge-store.svg',
                     'share/forge-store/ui-manifest-v1.json'):
        path = ui / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(b'test fixture')
    binary = root / 'forge-store-service'
    binary.write_bytes(b'test service')
    fixture = root / 'fixture'
    fixture.mkdir()
    (fixture / 'receipt.json').write_text(json.dumps({'packages': {}}))
    flatpak = root / 'flatpak'
    flatpak.mkdir()
    (flatpak / 'forge-store-fixture-public.gpg').write_bytes(b'public test fixture')
    for name in ('repo-v1', 'repo-v2'):
        (flatpak / name).mkdir()
    return SimpleNamespace(service_binary=binary, ui_stage=ui,
        candidate_v1=root/'candidate-v1', candidate_v2=root/'candidate-v2',
        fixture_dir=fixture, flatpak_dir=flatpak, source_root=REPO,
        output=root/'bundle.tar.gz', source_commit='a'*40)


def built_unit(args):
    build(args)
    with tarfile.open(args.output) as archive:
        data = archive.extractfile('usr/lib/systemd/user/forge-store.service').read().decode()
    unit = configparser.ConfigParser(interpolation=None)
    unit.read_string(data)
    return unit


class BundleProfileTest(unittest.TestCase):
    def test_default_profile_preserves_arch_fixture_and_namespace_configuration(self):
        with tempfile.TemporaryDirectory() as name:
            unit = built_unit(arguments(Path(name)))
            self.assertEqual(unit['Service']['ProtectSystem'], 'full')
            self.assertEqual(unit['Service']['PrivateTmp'], 'yes')
            self.assertIn('provision-flatpak-fixture', unit['Service']['ExecStartPre'])

    def test_ubuntu_profile_keeps_root_peers_visible_and_never_provisions_arch_fixture(self):
        with tempfile.TemporaryDirectory() as name:
            args = arguments(Path(name))
            args.profile = 'ubuntu'
            unit = built_unit(args)
            self.assertEqual(unit['Service']['PrivateUsers'], 'no')
            self.assertEqual(unit['Service']['ProtectSystem'], 'no')
            self.assertEqual(unit['Service']['PrivateTmp'], 'no')
            self.assertEqual(unit['Service']['NoNewPrivileges'], 'yes')
            self.assertNotIn('ExecStartPre', unit['Service'])
            self.assertEqual(unit['Service']['ExecStart'], '/usr/bin/forge-store-service')

    def test_unknown_profile_cannot_silently_use_arch_configuration(self):
        with tempfile.TemporaryDirectory() as name:
            args = arguments(Path(name))
            args.profile = 'other'
            with self.assertRaisesRegex(ValueError, 'profile'):
                build(args)


if __name__ == '__main__':
    unittest.main()
