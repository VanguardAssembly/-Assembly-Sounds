package __PKG__

import android.content.Context
import android.hardware.camera2.CameraCharacteristics
import android.hardware.camera2.CameraManager
import android.os.Handler
import android.os.HandlerThread
import android.os.SystemClock
import android.webkit.JavascriptInterface

/**
 * Flashlight strobe for opt-in "Light journeys" (stroboscopic closed-eye imagery).
 * Safety limits enforced here as well as in the UI: rate clamped to 3–12 Hz,
 * hard auto-stop after at most 30 minutes, torch always switched off on stop.
 * The page shows a full warning screen that must be accepted before this is ever called.
 */
object FocusLight {
    private const val MIN_HZ = 3.0
    private const val MAX_HZ = 12.0
    private const val MAX_MS = 30L * 60L * 1000L

    private var cm: CameraManager? = null
    private var camId: String? = null
    private var thread: HandlerThread? = null
    private var h: Handler? = null
    @Volatile private var hz = 10.0
    @Volatile private var running = false
    private var lit = false
    private var endAt = 0L
    private var next = 0L

    fun init(ctx: Context) {
        if (cm != null) return
        try {
            val m = ctx.getSystemService(CameraManager::class.java) ?: return
            cm = m
            camId = m.cameraIdList.firstOrNull {
                m.getCameraCharacteristics(it).get(CameraCharacteristics.FLASH_INFO_AVAILABLE) == true
            }
        } catch (e: Exception) { camId = null }
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
            lit = !lit
            torch(lit)
            val half = (500.0 / hz).toLong().coerceAtLeast(40L)
            next += half
            if (next < now) next = now + half
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

        @JavascriptInterface
        fun stop() {
            running = false
            val hh = h
            if (hh == null) torch(false) else hh.post { hh.removeCallbacks(tick); lit = false; torch(false) }
        }
    }
}
