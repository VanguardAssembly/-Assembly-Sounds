package __PKG__

import android.app.Activity
import android.app.Application
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.hardware.camera2.CameraCharacteristics
import android.hardware.camera2.CameraManager
import android.os.Bundle
import android.os.Handler
import android.os.HandlerThread
import android.os.Looper
import android.os.SystemClock
import android.view.WindowManager
import android.webkit.JavascriptInterface
import androidx.core.content.ContextCompat
import java.lang.ref.WeakReference

/**
 * Flashlight strobe for opt-in "Light journeys" (stroboscopic closed-eye imagery).
 * Safety limits enforced here as well as in the UI: rate clamped to 3–12 Hz (irregular
 * timing too), hard auto-stop after at most 30 minutes, torch always switched off on stop.
 * The page shows a full warning screen that must be accepted before this is ever called.
 *
 * Research presets can set the share of each cycle the light is on (duty), irregular
 * "arrhythmic" timing (jitter) and a steady, non-flashing light (steady).
 *
 * The light never keeps going when nobody can see the screen to stop it: it switches off at once
 * when the screen locks or turns off, when the app leaves the foreground, and when the page stops
 * drawing (it must check in at least every 3 seconds). The page is told, so it can say why.
 */
object FocusLight {
    private const val MIN_HZ = 3.0
    private const val MAX_HZ = 12.0
    private const val MAX_MS = 30L * 60L * 1000L
    private const val MIN_PART_MS = 40L
    private const val PING_MS = 3000L

    private var cm: CameraManager? = null
    private var camId: String? = null
    private var thread: HandlerThread? = null
    private var h: Handler? = null
    @Volatile private var hz = 10.0
    @Volatile private var duty = 0.5
    @Volatile private var jitter = 0.0
    @Volatile private var steady = false
    @Volatile private var running = false
    private var lit = false
    private var endAt = 0L
    private var next = 0L
    private var cycle = 100.0
    @Volatile private var lastPing = 0L
    private var act: WeakReference<Activity>? = null
    private var hooked = false
    private val main = Handler(Looper.getMainLooper())

    fun init(activity: Activity) {
        act = WeakReference(activity)
        val ctx = activity.applicationContext
        if (!hooked) {
            hooked = true
            // Screen locked or switched off (power button or time-out): light off at once.
            try {
                ContextCompat.registerReceiver(ctx, object : BroadcastReceiver() {
                    override fun onReceive(c: Context?, i: Intent?) { halt("screen") }
                }, IntentFilter(Intent.ACTION_SCREEN_OFF), ContextCompat.RECEIVER_EXPORTED)
            } catch (e: Exception) { }
            // App sent to the background (home, app switcher, another app or the lock screen on top).
            try {
                activity.application.registerActivityLifecycleCallbacks(object : Application.ActivityLifecycleCallbacks {
                    override fun onActivityPaused(a: Activity) { if (a === act?.get()) halt("background") }
                    override fun onActivityStopped(a: Activity) { if (a === act?.get()) halt("background") }
                    override fun onActivityDestroyed(a: Activity) { if (a === act?.get()) halt("background") }
                    override fun onActivityCreated(a: Activity, b: Bundle?) {}
                    override fun onActivityStarted(a: Activity) {}
                    override fun onActivityResumed(a: Activity) {}
                    override fun onActivitySaveInstanceState(a: Activity, b: Bundle) {}
                })
            } catch (e: Exception) { }
        }
        if (cm != null) return
        try {
            val m = ctx.getSystemService(CameraManager::class.java) ?: return
            cm = m
            camId = m.cameraIdList.firstOrNull {
                m.getCameraCharacteristics(it).get(CameraCharacteristics.FLASH_INFO_AVAILABLE) == true
            }
        } catch (e: Exception) { camId = null }
    }

    /** Light off now (screen off, app hidden, page not drawing) and tell the page why. */
    fun halt(why: String) {
        running = false
        val hh = h
        if (hh == null) torch(false) else hh.post { hh.removeCallbacks(tick); lit = false; torch(false) }
        screenOn(false)
        FocusMedia.lightStopped(why)
    }

    /** Keeps the screen from timing out during a light journey (locking it still stops the light). */
    private fun screenOn(on: Boolean) {
        main.post {
            try {
                val w = act?.get()?.window ?: return@post
                if (on) w.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
                else w.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
            } catch (e: Exception) { }
        }
    }

    private fun torch(on: Boolean) {
        val m = cm ?: return
        val id = camId ?: return
        try { m.setTorchMode(id, on) } catch (e: Exception) { running = false }
    }

    private val tick = object : Runnable {
        override fun run() {
            val now = SystemClock.uptimeMillis()
            if (!running || now > endAt) { running = false; lit = false; torch(false); return }
            // the page has not drawn a frame for 3 s: the screen is off or the app is hidden
            if (now - lastPing > PING_MS) { lit = false; torch(false); halt("stalled"); return }
            if (steady) {
                // constant light: stay on and check back for changes or the auto-stop
                if (!lit) { lit = true; torch(true) }
                next = now + 200L
                h?.postAtTime(this, next)
                return
            }
            lit = !lit
            torch(lit)
            if (lit) {
                // each cycle gets its own length; irregular timing never goes faster than 12 Hz
                var per = 1000.0 / hz
                if (jitter > 0.0) per = (per * (1.0 + (Math.random() * 2.0 - 1.0) * jitter)).coerceIn(1000.0 / MAX_HZ, 1000.0 / MIN_HZ)
                cycle = per
            }
            val part = (if (lit) cycle * duty else cycle * (1.0 - duty)).toLong().coerceAtLeast(MIN_PART_MS)
            next += part
            if (next < now) next = now + part
            h?.postAtTime(this, next)
        }
    }

    private fun clampHz(v: Double) = if (v.isFinite()) v.coerceIn(MIN_HZ, MAX_HZ) else 10.0

    class Bridge {
        @JavascriptInterface
        fun available(): Boolean = camId != null

        @JavascriptInterface
        fun start(rate: Double, minutes: Double) {
            if (camId == null) return
            hz = clampHz(rate)
            val ms = if (minutes.isFinite()) (minutes * 60_000).toLong().coerceIn(10_000L, MAX_MS) else MAX_MS
            lastPing = SystemClock.uptimeMillis()
            if (thread == null) {
                thread = HandlerThread("FocusLight").also { it.start() }
                h = Handler(thread!!.looper)
            }
            h?.post {
                endAt = SystemClock.uptimeMillis() + ms
                if (!running) {
                    running = true
                    lit = false
                    next = SystemClock.uptimeMillis()
                    h?.removeCallbacks(tick)
                    h?.post(tick)
                }
            }
        }

        @JavascriptInterface
        fun setRate(rate: Double) { hz = clampHz(rate) }

        /** The page is on screen and drawing (called a few times a second while the flashlight runs). */
        @JavascriptInterface
        fun ping() { lastPing = SystemClock.uptimeMillis() }

        /** Screen stays on for the whole light journey, so it never times out into a locked screen. */
        @JavascriptInterface
        fun keepAwake(on: Boolean) { screenOn(on) }

        /** Share of each cycle the light is on (0.2–0.8) and irregular timing (0–0.4 of a cycle). */
        @JavascriptInterface
        fun setShape(d: Double, j: Double) {
            duty = if (d.isFinite()) d.coerceIn(0.2, 0.8) else 0.5
            jitter = if (j.isFinite()) j.coerceIn(0.0, 0.4) else 0.0
        }

        /** Steady light (no flashing) for "steady" stages; switching restarts the timing at once. */
        @JavascriptInterface
        fun setSteady(on: Boolean) {
            if (steady == on) return
            steady = on
            val hh = h ?: return
            hh.post {
                if (running) {
                    hh.removeCallbacks(tick)
                    lit = !on
                    next = SystemClock.uptimeMillis()
                    hh.post(tick)
                }
            }
        }

        @JavascriptInterface
        fun stop() {
            running = false
            val hh = h
            if (hh == null) torch(false) else hh.post { hh.removeCallbacks(tick); lit = false; torch(false) }
        }
    }
}
