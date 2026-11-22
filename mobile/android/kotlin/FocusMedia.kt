package __PKG__

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.os.Handler
import android.os.Looper
import android.webkit.JavascriptInterface
import android.webkit.WebView
import androidx.core.content.ContextCompat
import java.lang.ref.WeakReference

/**
 * Bridge between the web UI and the lock-screen player.
 * The page calls window.FocusMedia.update(...) / .stop(); lock-screen buttons come back
 * as window.__media(action, arg). Only these two methods are exposed to the page,
 * and the page is the app's own bundled file (no web content is loaded).
 */
object FocusMedia {
    private var web: WeakReference<WebView>? = null
    private var appCtx: Context? = null
    private val main = Handler(Looper.getMainLooper())

    fun attach(activity: Activity, wv: WebView) {
        appCtx = activity.applicationContext
        web = WeakReference(wv)
        wv.addJavascriptInterface(Bridge(), "FocusMedia")
        // Opt-in flashlight strobe for Light journeys (see FocusLight.kt).
        FocusLight.init(activity.applicationContext)
        wv.addJavascriptInterface(FocusLight.Bridge(), "FocusLight")
        // An interface added after the page started loading only appears after a reload.
        wv.postDelayed({
            wv.evaluateJavascript("typeof window.FocusMedia") { r ->
                if (r != "\"object\"" && wv.url != null && wv.url != "about:blank") wv.reload()
            }
        }, 1500)
    }

    fun dispatch(action: String, arg: Long = 0) {
        val wv = web?.get() ?: return
        val safe = action.filter { it.isLetter() }
        wv.post { wv.evaluateJavascript("window.__media&&window.__media('$safe',$arg)", null) }
    }

    class Bridge {
        @JavascriptInterface
        fun update(title: String, sub: String, playing: Boolean, posMs: Double, durMs: Double) {
            val pos = posMs.toLong()
            val dur = durMs.toLong()
            main.post {
                val running = FocusMediaService.instance
                if (running != null) {
                    running.setState(title, sub, playing, pos, dur)
                } else {
                    val ctx = appCtx ?: return@post
                    val i = Intent(ctx, FocusMediaService::class.java).setAction("focus.update")
                        .putExtra("title", title).putExtra("sub", sub).putExtra("playing", playing)
                        .putExtra("pos", pos).putExtra("dur", dur)
                    try { ContextCompat.startForegroundService(ctx, i) } catch (e: Exception) { }
                }
            }
        }

        @JavascriptInterface
        fun stop() {
            main.post { FocusMediaService.instance?.shutdown() }
        }
    }
}
