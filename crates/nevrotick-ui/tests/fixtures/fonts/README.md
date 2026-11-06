# Portable rendering fixture

The unmodified Inter font and its sidecar are copied from `i-slint-common`
1.18.1, revision `372cf0ee5577c3dfec309a45e7b778ba4e81b734`,
`sharedfontique/Inter-VariableFont.ttf`.

Font SHA-256: `c333e1de6d6241530a473f480cd1f59a2578cd509825828b89a751c09fb9b61c`.
The original sidecar and font-embedded copyright notices remain intact. The
complete upstream OFL-1.1-RFN text is included beside them. This font is a
portable test input; it does not replace the Windows product font strategy.

The optional cloud SDK runner selects the fixed sibling `fonts.conf`, this
single font, and an owned writable cache. It admits their exact bytes before
execution and avoids host font configuration, fonts and user search paths.
