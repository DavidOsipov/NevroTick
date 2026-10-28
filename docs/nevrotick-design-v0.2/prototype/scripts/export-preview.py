"""Bundle the built UI into one offline HTML file. Does not alter image bytes."""
from pathlib import Path
import base64
import re

project = Path(__file__).resolve().parents[1]
build = project / "dist/client"
index = (build / "index.html").read_text(encoding="utf-8")
js_name = re.search(r'<script[^>]+src="([^"]+)"', index).group(1)
css_name = re.search(r'<link[^>]+href="([^"]+\.css)"', index).group(1)
js = (build / js_name.lstrip("/")).read_text(encoding="utf-8")
css = (build / css_name.lstrip("/")).read_text(encoding="utf-8")
for image_name in ("app-icon-concept.png", "logo-light-transparent.png", "mascot-selected.png"):
    payload = base64.b64encode((project / "public/assets" / image_name).read_bytes()).decode("ascii")
    js = js.replace("./assets/" + image_name, "data:image/png;base64," + payload)
js = js.replace("</script", "<\\/script")
document = """<!doctype html>
<html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="description" content="NevroTick interactive UI design. Illustrative data; no network time or system changes.">
<title>NevroTick — UI design preview</title><style>""" + css + """</style></head>
<body><div id="root"></div><script type="module">""" + js + """</script></body></html>"""
destination = project.parent / "NevroTick-UI-preview.html"
destination.write_text(document, encoding="utf-8")
assert not re.search(r'<(?:script|link)[^>]+(?:src|href)="(?:https?:|\./assets|/assets)', document)
print(f"Created {destination.name}: {destination.stat().st_size} bytes; embedded images and UI bundle.")
