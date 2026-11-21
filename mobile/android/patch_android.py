"""Adds the lock-screen media player to the Tauri-generated Android project.

Run from mobile/ after `cargo tauri android init` (the generated gen/ folder is not committed):
  - copies FocusMedia*.kt next to MainActivity.kt (package filled in automatically)
  - hooks MainActivity.onWebViewCreate -> FocusMedia.attach
  - adds the media foreground service + permissions to AndroidManifest.xml
  - adds androidx.media / androidx.core to the app's Gradle dependencies
  - copies the status-bar icon and the lock-screen artwork
"""
import pathlib
import re
import shutil
import sys

here = pathlib.Path(__file__).resolve().parent          # mobile/android
mobile = here.parent                                     # mobile
app = mobile / "gen" / "android" / "app"
if not app.is_dir():
    sys.exit(f"generated Android app not found at {app}")

main_kt = next(app.rglob("MainActivity.kt"))
src = main_kt.read_text(encoding="utf-8")
pkg = re.search(r"^package\s+([\w.]+)", src, re.M).group(1)

gradle = app / "build.gradle.kts"
g = gradle.read_text(encoding="utf-8")
ns = re.search(r'namespace\s*=\s*"([\w.]+)"', g)
r_import = f"import {ns.group(1)}.R" if ns and ns.group(1) != pkg else ""

# 1) Kotlin sources
for kt in (here / "kotlin").glob("*.kt"):
    text = kt.read_text(encoding="utf-8").replace("__PKG__", pkg).replace("__RIMPORT__", r_import)
    (main_kt.parent / kt.name).write_text(text, encoding="utf-8")

# 2) MainActivity hook
hook = (
    "    override fun onWebViewCreate(webView: android.webkit.WebView) {\n"
    "        super.onWebViewCreate(webView)\n"
    "        FocusMedia.attach(this, webView)\n"
    "    }\n"
)
if "FocusMedia.attach" not in src:
    m = re.search(r"class\s+MainActivity\s*:\s*TauriActivity\(\)\s*\{", src)
    if m:
        src = src[: m.end()] + "\n" + hook + src[m.end():]
    else:
        src = re.sub(r"class\s+MainActivity\s*:\s*TauriActivity\(\)", "class MainActivity : TauriActivity() {\n" + hook + "}", src, count=1)
    main_kt.write_text(src, encoding="utf-8")

# 3) Manifest
man = app / "src" / "main" / "AndroidManifest.xml"
x = man.read_text(encoding="utf-8")
perms = [
    "android.permission.FOREGROUND_SERVICE",
    "android.permission.FOREGROUND_SERVICE_MEDIA_PLAYBACK",
    "android.permission.POST_NOTIFICATIONS",
]
add = "".join(f'    <uses-permission android:name="{p}" />\n' for p in perms if p not in x)
x = x.replace("<application", add + "    <application", 1) if add else x
if "FocusMediaService" not in x:
    svc = ('        <service android:name=".FocusMediaService" android:exported="false"\n'
           '            android:foregroundServiceType="mediaPlayback" />\n')
    if pkg != (ns.group(1) if ns else pkg):
        svc = svc.replace('".FocusMediaService"', f'"{pkg}.FocusMediaService"')
    x = x.replace("</application>", svc + "    </application>", 1)
man.write_text(x, encoding="utf-8")

# 4) Gradle dependencies
if "androidx.media:media" not in g:
    g = re.sub(r"dependencies\s*\{", 'dependencies {\n    implementation("androidx.media:media:1.7.0")\n    implementation("androidx.core:core-ktx:1.13.1")', g, count=1)
    gradle.write_text(g, encoding="utf-8")

# 5) Resources
res = app / "src" / "main" / "res"
(res / "drawable").mkdir(parents=True, exist_ok=True)
shutil.copy(here / "res" / "drawable" / "ic_stat_focus.xml", res / "drawable" / "ic_stat_focus.xml")
(res / "drawable-nodpi").mkdir(parents=True, exist_ok=True)
shutil.copy(mobile / "icon.png", res / "drawable-nodpi" / "focus_art.png")

print(f"patched: package={pkg} namespace={ns.group(1) if ns else '?'} main={main_kt}")
print("---- MainActivity.kt ----\n" + main_kt.read_text(encoding="utf-8"))
