package __PKG__

import android.app.Activity
import android.hardware.display.DisplayManager
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.view.Display
import android.view.View
import android.webkit.JavascriptInterface
import android.webkit.WebView
import java.lang.ref.WeakReference

/**
 * Runs the app at the phone's highest screen refresh rate (for example 120 Hz instead of 60 Hz),
 * keeping the screen's current resolution. On by default; the page can switch it back to the
 * system's own choice (Settings → "Highest screen refresh rate"). Android still applies the phone's
 * own limits, such as battery saver or a refresh-rate cap the user set in the system settings.
 */
object FocusScreen {
    private var act: WeakReference<Activity>? = null
    private var web: WeakReference<WebView>? = null
    private val main = Handler(Looper.getMainLooper())
    @Volatile private var high = true
    @Volatile private var maxHz = 0f
    @Volatile private var curHz = 0f
    private var listening = false

    fun attach(activity: Activity, wv: WebView) {
        act = WeakReference(activity)
        web = WeakReference(wv)
        wv.addJavascriptInterface(Bridge(), "FocusScreen")
        apply()
        if (!listening) {
            listening = true
            // A foldable's other screen, or a mode change made by the system: pick again.
            try {
                activity.getSystemService(DisplayManager::class.java)?.registerDisplayListener(object : DisplayManager.DisplayListener {
                    override fun onDisplayAdded(id: Int) {}
                    override fun onDisplayRemoved(id: Int) {}
                    override fun onDisplayChanged(id: Int) { apply() }
                }, main)
            } catch (e: Exception) { }
        }
    }

    private fun display(a: Activity): Display? =
        if (Build.VERSION.SDK_INT >= 30) a.display else @Suppress("DEPRECATION") a.windowManager.defaultDisplay

    /** The fastest mode at the current resolution. */
    private fun best(d: Display): Display.Mode? {
        val cur = d.mode
        return d.supportedModes
            .filter { it.physicalWidth == cur.physicalWidth && it.physicalHeight == cur.physicalHeight }
            .maxByOrNull { it.refreshRate }
    }

    /** Main thread only. Changes the window only when the wanted mode differs, so it never loops. */
    fun apply() {
        val a = act?.get() ?: return
        try {
            val w = a.window ?: return
            val d = display(a) ?: return
            val top = best(d)
            maxHz = top?.refreshRate ?: d.refreshRate
            curHz = d.refreshRate
            val want = if (high) top?.modeId ?: 0 else 0
            val lp = w.attributes
            if (lp.preferredDisplayModeId != want) {
                lp.preferredDisplayModeId = want
                w.attributes = lp
            }
            web?.get()?.let { viewRate(it, high) }
        } catch (e: Exception) { }
    }

    /** Android 15+: the WebView also asks for a high frame rate while it draws (looked up by name, so older phones skip it). */
    private fun viewRate(v: View, on: Boolean) {
        if (Build.VERSION.SDK_INT < 35) return
        try {
            val name = if (on) "REQUESTED_FRAME_RATE_CATEGORY_HIGH" else "REQUESTED_FRAME_RATE_CATEGORY_DEFAULT"
            val cat = View::class.java.getField(name).getFloat(null)
            View::class.java.getMethod("setRequestedFrameRate", java.lang.Float.TYPE).invoke(v, cat)
        } catch (e: Throwable) { }
    }

    class Bridge {
        /** true = fastest refresh rate the screen offers (the default); false = the system decides. */
        @JavascriptInterface
        fun setHigh(on: Boolean) {
            high = on
            main.post { apply() }
        }

        /** {"max":120,"cur":120,"high":true} */
        @JavascriptInterface
        fun info(): String = "{\"max\":${Math.round(maxHz)},\"cur\":${Math.round(curHz)},\"high\":$high}"
    }
}
