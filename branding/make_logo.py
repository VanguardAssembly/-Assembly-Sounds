"""Generates the Assembly Sounds VHS-style logo SVGs (rendered to PNG with headless Edge).
Style: cream slab letters with a blue 3D extrusion and a slanted blue bar with a yellow stripe."""
import math

def lerp(a, b, t): return tuple(round(a[i] + (b[i] - a[i]) * t) for i in range(3))
def hexc(c): return "#%02x%02x%02x" % c

def letters(x, y, size, depth, letters="AS"):
    far, near = (16, 52, 150), (96, 160, 255)
    out = []
    font = f'font-family="Georgia" font-weight="700" font-size="{size}" text-anchor="middle" letter-spacing="-{size*0.04:.0f}"'
    sw = size * 0.035
    for i in range(depth, 0, -1):
        c = hexc(lerp(near, far, i / depth))
        dx, dy = -i * 0.95, i * 0.8
        out.append(f'<text x="{x+dx:.1f}" y="{y+dy:.1f}" {font} fill="{c}" stroke="{c}" stroke-width="{sw:.1f}" stroke-linejoin="round">{letters}</text>')
    # bevel highlight then cream face
    out.append(f'<text x="{x+1.5}" y="{y-1.5}" {font} fill="#bcd8ff" stroke="#bcd8ff" stroke-width="{sw:.1f}" stroke-linejoin="round">{letters}</text>')
    out.append(f'<text x="{x}" y="{y}" {font} fill="url(#face)" stroke="url(#face)" stroke-width="{sw*0.8:.1f}" stroke-linejoin="round">{letters}</text>')
    return "\n".join(out)

def bar(x0, y0, w, h, slant, label=None, wave=False, fs=0):
    p = lambda pts: " ".join(f"{a:.1f},{b:.1f}" for a, b in pts)
    stripe = h * 0.16
    body = [(x0 + slant, y0), (x0 + w + slant, y0), (x0 + w, y0 + h), (x0, y0 + h)]
    ys = [(x0 + slant, y0), (x0 + w + slant, y0), (x0 + w + slant - slant * stripe / h, y0 + stripe), (x0 + slant - slant * stripe / h, y0 + stripe)]
    s = f'<polygon points="{p(body)}" fill="url(#barg)"/>\n<polygon points="{p(ys)}" fill="#f2d84a"/>'
    cx, cy = x0 + w / 2 + slant / 2, y0 + stripe + (h - stripe) / 2
    if label:
        s += f'\n<text x="{cx:.1f}" y="{cy + fs*0.36:.1f}" font-family="Arial Black" font-size="{fs}" fill="#ffffff" text-anchor="middle" letter-spacing="1">{label}</text>'
    if wave:
        # binaural "beat": a carrier wave inside a slow envelope
        pts = []
        span, amp = w * 0.74, (h - stripe) * 0.34
        for k in range(0, 601):
            t = k / 600
            xx = cx - span / 2 + span * t
            env = math.sin(math.pi * t) * (0.35 + 0.65 * abs(math.cos(2 * math.pi * t)))
            pts.append((xx - slant * 0.0, cy + amp * env * math.sin(2 * math.pi * 9 * t)))
        s += f'\n<polyline points="{p(pts)}" fill="none" stroke="#ffffff" stroke-width="{h*0.055:.1f}" stroke-linejoin="round" stroke-linecap="round"/>'
    return s

DEFS = '''<defs>
<linearGradient id="face" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#fffaf0"/><stop offset="1" stop-color="#e4d8c3"/></linearGradient>
<linearGradient id="barg" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#2f63e6"/><stop offset="1" stop-color="#1a3ca8"/></linearGradient>
<radialGradient id="bg" cx="0.5" cy="0.42" r="0.75"><stop offset="0" stop-color="#1b2150"/><stop offset="0.6" stop-color="#0a0c1f"/><stop offset="1" stop-color="#030409"/></radialGradient>
<pattern id="scan" width="4" height="4" patternUnits="userSpaceOnUse"><rect width="4" height="2" fill="#000" opacity="0.28"/></pattern>
<radialGradient id="vig" cx="0.5" cy="0.5" r="0.7"><stop offset="0.6" stop-color="#000" stop-opacity="0"/><stop offset="1" stop-color="#000" stop-opacity="0.55"/></radialGradient>
</defs>'''

# Wide logo (transparent): used by the intro and the header
wide = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="560" viewBox="0 90 1200 560">{DEFS}
{letters(615, 455, 470, 46)}
{bar(150, 500, 930, 118, 36, label="Assembly Sounds", fs=62)}
</svg>'''

# App icon (square, opaque CRT background; content kept inside the adaptive-icon safe zone)
icon = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024">{DEFS}
<rect width="1024" height="1024" fill="url(#bg)"/>
<g transform="translate(512 520) scale(0.76) translate(-615 -372)">
{letters(615, 455, 470, 46)}
{bar(170, 500, 900, 124, 36, wave=True)}
</g>
<rect width="1024" height="1024" fill="url(#scan)" opacity="0.55"/>
<rect width="1024" height="1024" fill="url(#vig)"/>
</svg>'''

for name, svg, (w, h) in (("logo_wide", wide, (1200, 560)), ("icon", icon, (1024, 1024))):
    open(f"{name}.svg", "w", encoding="utf-8").write(svg)
    open(f"{name}.html", "w", encoding="utf-8").write(
        f'<!doctype html><html><head><style>html,body{{margin:0;background:transparent}}svg{{display:block}}</style></head><body>{svg}</body></html>')
print("ok")
