#!/usr/bin/env python3
"""Validate committed SVG assets and literal source resource references."""
from pathlib import Path
import re
import sys
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parent.parent
SVG_DIR = ROOT / "resources" / "icons"
REFERENCE = re.compile(r'["\'](?:self:)?(resources/[^"\']+\.svg)["\']')
errors = []
assets = sorted(SVG_DIR.rglob("*.svg"))
if not assets:
    errors.append("no SVG assets found under resources/icons")
for asset in assets:
    try:
        root = ET.parse(asset).getroot()
        if root.tag != "{http://www.w3.org/2000/svg}svg":
            errors.append(f"{asset.relative_to(ROOT)}: root is not an SVG element")
    except (OSError, ET.ParseError) as error:
        errors.append(f"{asset.relative_to(ROOT)}: {error}")
references = set()
for source in sorted((ROOT / "src").rglob("*.rs")):
    for match in REFERENCE.finditer(source.read_text(encoding="utf-8")):
        ref = match.group(1)
        references.add(ref)
        path = ROOT / ref
        if ".." in Path(ref).parts or not path.is_file():
            errors.append(f"{source.relative_to(ROOT)}: invalid or missing {ref}")
if not references:
    errors.append("no literal SVG resource references found in src")
if errors:
    for error in errors:
        print(error, file=sys.stderr)
    sys.exit(1)
print(f"Validated {len(assets)} SVG files and {len(references)} literal references")
