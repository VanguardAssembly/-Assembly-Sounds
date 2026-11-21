package __PKG__
__RIMPORT__
import android.app.PendingIntent
import android.app.Service
import android.content.Intent
import android.content.pm.ServiceInfo
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.os.Build
import android.os.IBinder
import android.support.v4.media.MediaMetadataCompat
import android.support.v4.media.session.MediaSessionCompat
import android.support.v4.media.session.PlaybackStateCompat
import androidx.core.app.NotificationChannelCompat
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import androidx.core.app.ServiceCompat

/**
 * Lock-screen / notification player for Focus sessions (like a music app).
 * Being a "mediaPlayback" foreground service also keeps audio running with the screen off.
 * Buttons are forwarded to the web UI, which presses its own Play / Stage / Stop buttons.
 */
class FocusMediaService : Service() {
    companion object {
        @Volatile var instance: FocusMediaService? = null
        const val CHANNEL = "focus_playback"
        const val NOTIF_ID = 1
    }

    private lateinit var session: MediaSessionCompat
    private var art: Bitmap? = null
    private var title = "Focus"
    private var sub = ""
    private var playing = false
    private var posMs = 0L
    private var durMs = -1L
    private var ended = false

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onCreate() {
        super.onCreate()
        instance = this
        NotificationManagerCompat.from(this).createNotificationChannel(
            NotificationChannelCompat.Builder(CHANNEL, NotificationManagerCompat.IMPORTANCE_LOW)
                .setName("Session playback")
                .setDescription("Shows the current session with play/pause controls")
                .setShowBadge(false)
                .build()
        )
        art = try {
            BitmapFactory.decodeResource(resources, R.drawable.focus_art, BitmapFactory.Options().apply { inSampleSize = 2 })
        } catch (e: Exception) { null }
        session = MediaSessionCompat(this, "Focus").apply {
            setCallback(object : MediaSessionCompat.Callback() {
                override fun onPlay() = FocusMedia.dispatch("play")
                override fun onPause() = FocusMedia.dispatch("pause")
                override fun onSkipToNext() = FocusMedia.dispatch("next")
                override fun onSkipToPrevious() = FocusMedia.dispatch("prev")
                override fun onStop() = FocusMedia.dispatch("stop")
                override fun onSeekTo(pos: Long) = FocusMedia.dispatch("seek", pos)
            })
            isActive = true
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (val a = intent?.action) {
            "focus.update" -> {
                title = intent.getStringExtra("title") ?: title
                sub = intent.getStringExtra("sub") ?: ""
                playing = intent.getBooleanExtra("playing", false)
                posMs = intent.getLongExtra("pos", 0L)
                durMs = intent.getLongExtra("dur", -1L)
            }
            "focus.play", "focus.pause", "focus.next", "focus.prev", "focus.stop" ->
                FocusMedia.dispatch(a.removePrefix("focus."))
        }
        if (!ended) render()
        return START_NOT_STICKY
    }

    /** Called on the main thread by the UI bridge while the service is running. */
    fun setState(t: String, s: String, pl: Boolean, pos: Long, dur: Long) {
        title = t; sub = s; playing = pl; posMs = pos; durMs = dur
        if (!ended) render()
    }

    private fun pi(action: String, code: Int): PendingIntent =
        PendingIntent.getService(
            this, code, Intent(this, FocusMediaService::class.java).setAction(action),
            PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT
        )

    private fun render() {
        val meta = MediaMetadataCompat.Builder()
            .putString(MediaMetadataCompat.METADATA_KEY_TITLE, title)
            .putString(MediaMetadataCompat.METADATA_KEY_ARTIST, sub)
            .putString(MediaMetadataCompat.METADATA_KEY_ALBUM, "Focus Binaural")
            .putLong(MediaMetadataCompat.METADATA_KEY_DURATION, if (durMs > 0) durMs else -1L)
        art?.let { meta.putBitmap(MediaMetadataCompat.METADATA_KEY_ART, it) }
        session.setMetadata(meta.build())

        var actions = PlaybackStateCompat.ACTION_PLAY or PlaybackStateCompat.ACTION_PAUSE or
            PlaybackStateCompat.ACTION_PLAY_PAUSE or PlaybackStateCompat.ACTION_STOP or
            PlaybackStateCompat.ACTION_SKIP_TO_NEXT or PlaybackStateCompat.ACTION_SKIP_TO_PREVIOUS
        if (durMs > 0) actions = actions or PlaybackStateCompat.ACTION_SEEK_TO
        session.setPlaybackState(
            PlaybackStateCompat.Builder()
                .setActions(actions)
                .setState(
                    if (playing) PlaybackStateCompat.STATE_PLAYING else PlaybackStateCompat.STATE_PAUSED,
                    posMs, if (playing) 1f else 0f
                )
                .build()
        )

        val open = PendingIntent.getActivity(
            this, 0,
            Intent(this, MainActivity::class.java).addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP or Intent.FLAG_ACTIVITY_REORDER_TO_FRONT),
            PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT
        )
        val b = NotificationCompat.Builder(this, CHANNEL)
            .setSmallIcon(R.drawable.ic_stat_focus)
            .setContentTitle(title)
            .setContentText(sub)
            .setContentIntent(open)
            .setDeleteIntent(pi("focus.stop", 5))
            .setVisibility(NotificationCompat.VISIBILITY_PUBLIC)
            .setCategory(NotificationCompat.CATEGORY_TRANSPORT)
            .setOnlyAlertOnce(true)
            .setSilent(true)
            .setOngoing(playing)
            .addAction(android.R.drawable.ic_media_previous, "Previous stage", pi("focus.prev", 1))
            .addAction(
                if (playing) android.R.drawable.ic_media_pause else android.R.drawable.ic_media_play,
                if (playing) "Pause" else "Play",
                pi(if (playing) "focus.pause" else "focus.play", 2)
            )
            .addAction(android.R.drawable.ic_media_next, "Next stage", pi("focus.next", 3))
            .addAction(android.R.drawable.ic_menu_close_clear_cancel, "Stop", pi("focus.stop", 4))
            .setStyle(
                androidx.media.app.NotificationCompat.MediaStyle()
                    .setMediaSession(session.sessionToken)
                    .setShowActionsInCompactView(0, 1, 2)
            )
        art?.let { b.setLargeIcon(it) }
        ServiceCompat.startForeground(
            this, NOTIF_ID, b.build(),
            if (Build.VERSION.SDK_INT >= 29) ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK else 0
        )
    }

    /** Session finished or stopped: remove the player and end the service. */
    fun shutdown() {
        ended = true
        ServiceCompat.stopForeground(this, ServiceCompat.STOP_FOREGROUND_REMOVE)
        stopSelf()
    }

    override fun onDestroy() {
        instance = null
        session.isActive = false
        session.release()
        super.onDestroy()
    }
}
