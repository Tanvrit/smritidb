@file:OptIn(kotlinx.cinterop.ExperimentalForeignApi::class)

package com.tanvrit.smritidb

import kotlinx.cinterop.COpaquePointer
import kotlinx.cinterop.CPointer
import kotlinx.cinterop.CValue
import kotlinx.cinterop.MemScope
import kotlinx.cinterop.addressOf
import kotlinx.cinterop.alloc
import kotlinx.cinterop.cValue
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.ptr
import kotlinx.cinterop.readBytes
import kotlinx.cinterop.reinterpret
import kotlinx.cinterop.useContents
import kotlinx.cinterop.usePinned
import platform.posix.uint8_tVar
import smritidb_ffi.ForeignBytes
import smritidb_ffi.RustBuffer
import smritidb_ffi.RustCallStatus
import smritidb_ffi.ffi_smritidb_ffi_rustbuffer_free
import smritidb_ffi.ffi_smritidb_ffi_rustbuffer_from_bytes

// ---------------------------------------------------------------------------
// UniFFI low-level lift/lower helpers for the Apple Cinterop binding.
//
// UniFFI does not ship native-Kotlin bindings, so this file re-implements
// the slice of its binary protocol used by `Smritidb.kt` and
// `PersistentStore.kt`. Reference:
// https://mozilla.github.io/uniffi-rs/internals/lifting_and_lowering.html
//
// Conventions:
//
//   * Multi-byte integers are written big-endian.
//   * Top-level `String` / `Vec<u8>` cross the FFI as a `RustBuffer`
//     whose `len` IS the byte count and whose bytes are the raw payload
//     (no inner length prefix).
//   * Inside a compound (record field, sequence element, etc.) the
//     same payload is framed as `Int32 len + bytes`.
//   * `Option<T>` is `Int8 tag (0=None | 1=Some)` followed by the body
//     when `Some`.
//   * Sequences are `Int32 len` followed by N inlined elements.
//
// Each `call*` helper provisions a `RustCallStatus`, runs the FFI call,
// and projects any error variant to `PersistentStoreException`.
// ---------------------------------------------------------------------------

/**
 * Thrown when a UniFFI FFI call returns a non-zero `RustCallStatus.code`.
 * The `code` is preserved (0 = success, 1 = expected error variant,
 * 2 = Rust panic) and the message is read either from the panic buffer
 * or from the variant's first field per the UniFFI binary layout.
 *
 * The JVM actual surfaces `SmritidbException` subclasses directly; on
 * Apple we collapse them into a single class with the variant tag
 * preserved.
 */
public class PersistentStoreException internal constructor(
    message: String,
    public val code: Byte = -1,
) : RuntimeException(message)

// ---------- Status / call helpers --------------------------------------------------

/**
 * `code` discrimination per UniFFI's RustCallStatus contract:
 *   0 = success
 *   1 = an expected error (variant encoded in `errorBuf`)
 *   2 = a Rust panic — `errorBuf` is a UTF-8 string
 */
internal fun MemScope.checkStatus(status: RustCallStatus) {
    val code = status.code
    if (code == 0.toByte()) return
    // `status.errorBuf` is the embedded RustBuffer field; read its
    // bytes, then free the Rust-side allocation before throwing.
    val errBuf = status.errorBuf
    val capacity = errBuf.capacity
    val length = errBuf.len
    val ptr = errBuf.data
    val raw: ByteArray = if (length.toLong() == 0L || ptr == null) {
        ByteArray(0)
    } else {
        ptr.readBytes(length.toInt())
    }
    val toFree = cValue<RustBuffer> {
        this.capacity = capacity
        this.len = length
        this.data = ptr
    }
    val freeStatus = alloc<RustCallStatus>()
    freeStatus.code = 0
    ffi_smritidb_ffi_rustbuffer_free(toFree, freeStatus.ptr)
    val message = when (code) {
        2.toByte() -> "rust panic: ${raw.decodeToString()}"
        else -> parseSmritidbError(raw)
    }
    throw PersistentStoreException(message, code = code)
}

private fun parseSmritidbError(raw: ByteArray): String {
    if (raw.isEmpty()) return "unknown error"
    val reader = RustBufferReader(raw)
    val variant = reader.readInt32()
    return when (variant) {
        1 -> "dimension mismatch"
        2 -> "invalid configuration"
        3 -> "value too large"
        4 -> "not found"
        5 -> "empty input"
        6 -> "io: ${reader.readFramedString()}"
        7 -> "database: ${reader.readFramedString()}"
        8 -> "corruption: ${reader.readFramedString()}"
        9 -> "encoding: ${reader.readFramedString()}"
        10 -> "other: ${reader.readFramedString()}"
        else -> "unknown error variant $variant"
    }
}

internal inline fun callPointer(
    block: MemScope.(CPointer<RustCallStatus>) -> COpaquePointer?,
): COpaquePointer = memScoped {
    val status = alloc<RustCallStatus>()
    status.code = 0
    val result = block(status.ptr)
    checkStatus(status)
    result ?: throw PersistentStoreException("uniffi returned a null pointer")
}

internal inline fun callRustBuffer(
    block: MemScope.(CPointer<RustCallStatus>) -> CValue<RustBuffer>,
): CValue<RustBuffer> = memScoped {
    val status = alloc<RustCallStatus>()
    status.code = 0
    val result = block(status.ptr)
    checkStatus(status)
    result
}

internal inline fun callUInt(
    block: MemScope.(CPointer<RustCallStatus>) -> UInt,
): UInt = memScoped {
    val status = alloc<RustCallStatus>()
    status.code = 0
    val result = block(status.ptr)
    checkStatus(status)
    result
}

internal inline fun callByte(
    block: MemScope.(CPointer<RustCallStatus>) -> Byte,
): Byte = memScoped {
    val status = alloc<RustCallStatus>()
    status.code = 0
    val result = block(status.ptr)
    checkStatus(status)
    result
}

internal inline fun callDouble(
    block: MemScope.(CPointer<RustCallStatus>) -> Double,
): Double = memScoped {
    val status = alloc<RustCallStatus>()
    status.code = 0
    val result = block(status.ptr)
    checkStatus(status)
    result
}

internal inline fun callVoid(
    block: MemScope.(CPointer<RustCallStatus>) -> Unit,
) {
    memScoped {
        val status = alloc<RustCallStatus>()
        status.code = 0
        block(status.ptr)
        checkStatus(status)
    }
}

// ---------- Lower (Kotlin -> RustBuffer) -------------------------------------------

internal fun rustBufferEmpty(): CValue<RustBuffer> = cValue {
    capacity = 0u
    len = 0u
    data = null
}

/**
 * Round-trip raw bytes into a Rust-allocated `RustBuffer` via
 * `ffi_smritidb_ffi_rustbuffer_from_bytes`. The Rust side will free
 * the buffer when it consumes the argument; no Kotlin-side free.
 */
internal fun rustBufferFromInlineBytes(bytes: ByteArray): CValue<RustBuffer> {
    if (bytes.isEmpty()) return rustBufferEmpty()
    return memScoped {
        val statusVar = alloc<RustCallStatus>()
        statusVar.code = 0
        bytes.usePinned { pinned ->
            val rawPtr: CPointer<uint8_tVar> = pinned.addressOf(0).reinterpret()
            val foreign: CValue<ForeignBytes> = cValue {
                len = bytes.size
                data = rawPtr
            }
            val buf = ffi_smritidb_ffi_rustbuffer_from_bytes(foreign, statusVar.ptr)
            checkStatus(statusVar)
            buf
        }
    }
}

/**
 * `String` is the one type that lowers as raw UTF-8 in a `RustBuffer`
 * without a length prefix (the buffer's `len` carries that). Every
 * other compound — `Vec<u8>`, `Option<T>`, sequences, records — goes
 * through the framed `FfiConverterRustBuffer` path.
 */
internal fun rustBufferFromUtf8(s: String): CValue<RustBuffer> =
    rustBufferFromInlineBytes(s.encodeToByteArray())

/**
 * `Vec<u8>` at the FFI boundary is the same framed format used inside
 * compound types: `Int32 len` (big-endian) followed by the bytes. See
 * `FfiConverterData` in the UniFFI Swift binding for the reference
 * encoding.
 */
internal fun rustBufferFromBytes(value: ByteArray): CValue<RustBuffer> {
    val out = RustBufferWriter()
    out.writeInt32(value.size)
    out.writeBytes(value)
    return rustBufferFromInlineBytes(out.toByteArray())
}

/**
 * `StoreOptions` lowered. Layout (big-endian):
 *   u32 dimension | u32 valueCapBytes | u32 defaultTopK | f64 defaultMinSimilarity
 *
 * Zeroes for the latter three tell `StoreOptions::to_core` in the Rust
 * shim to substitute the core defaults — matching the JVM actual.
 */
internal fun lowerStoreOptions(dimension: Int): CValue<RustBuffer> {
    val out = RustBufferWriter()
    out.writeUInt32(dimension.toUInt())
    out.writeUInt32(0u)
    out.writeUInt32(0u)
    out.writeDouble(0.0)
    return rustBufferFromInlineBytes(out.toByteArray())
}

internal fun lowerStringList(items: List<String>): CValue<RustBuffer> {
    val out = RustBufferWriter()
    out.writeInt32(items.size)
    for (item in items) {
        val bytes = item.encodeToByteArray()
        out.writeInt32(bytes.size)
        out.writeBytes(bytes)
    }
    return rustBufferFromInlineBytes(out.toByteArray())
}

internal fun lowerByteArrayList(items: List<ByteArray>): CValue<RustBuffer> {
    val out = RustBufferWriter()
    out.writeInt32(items.size)
    for (item in items) {
        out.writeInt32(item.size)
        out.writeBytes(item)
    }
    return rustBufferFromInlineBytes(out.toByteArray())
}

internal fun lowerFloatArray(floats: FloatArray): CValue<RustBuffer> {
    val out = RustBufferWriter()
    out.writeInt32(floats.size)
    for (f in floats) out.writeFloat(f)
    return rustBufferFromInlineBytes(out.toByteArray())
}

/** Encodes the UniFFI `Option<String>` discriminator as `None` (single 0 byte). */
internal fun lowerNoneString(): CValue<RustBuffer> =
    rustBufferFromInlineBytes(byteArrayOf(0))

// ---------- Lift (RustBuffer -> Kotlin) --------------------------------------------

/** Lift a top-level `RustBuffer` carrying raw UTF-8 into a Kotlin `String`. */
internal fun liftString(buf: CValue<RustBuffer>): String =
    buf.useContents {
        val bytes = if (len.toLong() == 0L || data == null) {
            ByteArray(0)
        } else {
            data!!.readBytes(len.toInt())
        }
        // Save the fields before deallocating so we can free in the
        // outer scope; `data` is invalid after the free call.
        val capacityCopy = capacity
        val lenCopy = len
        val dataCopy = data
        freeRustBuffer(capacityCopy, lenCopy, dataCopy)
        bytes.decodeToString()
    }

/**
 * Lift a top-level `RustBuffer` carrying a framed `Vec<u8>`. The buffer
 * starts with an Int32 length prefix followed by the bytes — matching
 * UniFFI's `FfiConverterData.read` in the Swift binding.
 */
internal fun liftBytes(buf: CValue<RustBuffer>): ByteArray {
    val raw = liftRustBufferRaw(buf)
    if (raw.isEmpty()) return ByteArray(0)
    val reader = RustBufferReader(raw)
    return reader.readFramedBytes()
}

private fun liftRustBufferRaw(buf: CValue<RustBuffer>): ByteArray =
    buf.useContents {
        val bytes = if (len.toLong() == 0L || data == null) {
            ByteArray(0)
        } else {
            data!!.readBytes(len.toInt())
        }
        val capacityCopy = capacity
        val lenCopy = len
        val dataCopy = data
        freeRustBuffer(capacityCopy, lenCopy, dataCopy)
        bytes
    }

/** Lift a top-level `RustBuffer` of `Match` records into [RecallMatch]. */
internal fun liftMatchList(buf: CValue<RustBuffer>): List<RecallMatch> {
    val bytes = liftRustBufferRaw(buf)
    val reader = RustBufferReader(bytes)
    val count = reader.readInt32()
    val out = ArrayList<RecallMatch>(count)
    repeat(count) {
        // Match layout per smritidb.swift / FfiConverterTypeMatch.read:
        //   id: String, similarity: f64, value: Vec<u8>,
        //   tags: [String], accessCount: u32
        val id = reader.readFramedString()
        val similarity = reader.readDouble()
        reader.skipFramedBytes() // value (not exposed at the KMP surface)
        val tags = reader.readStringList()
        reader.readInt32() // accessCount (not exposed at the KMP surface)
        out += RecallMatch(id = id, similarity = similarity, tags = tags)
    }
    return out
}

/** Lift a top-level `RustBuffer` of `Match` records into [Match]. */
internal fun liftMatchListFull(buf: CValue<RustBuffer>): List<Match> {
    val bytes = liftRustBufferRaw(buf)
    val reader = RustBufferReader(bytes)
    val count = reader.readInt32()
    val out = ArrayList<Match>(count)
    repeat(count) {
        val id = reader.readFramedString()
        val similarity = reader.readDouble()
        val value = reader.readFramedBytes()
        val tags = reader.readStringList()
        val accessCount = reader.readInt32().toUInt()
        out += Match(
            id = id,
            similarity = similarity,
            value = value,
            tags = tags,
            accessCount = accessCount,
        )
    }
    return out
}

private fun freeRustBuffer(capacity: ULong, len: ULong, data: CPointer<uint8_tVar>?) {
    if (data == null && capacity.toLong() == 0L) return
    memScoped {
        val status = alloc<RustCallStatus>()
        status.code = 0
        val toFree = cValue<RustBuffer> {
            this.capacity = capacity
            this.len = len
            this.data = data
        }
        ffi_smritidb_ffi_rustbuffer_free(toFree, status.ptr)
        // `_rustbuffer_free` doesn't fail in practice; still surface a
        // hard error rather than silently corrupting state.
        checkStatus(status)
    }
}

// ---------- Big-endian byte stream helpers (UniFFI is BE everywhere) --------------

internal class RustBufferWriter {
    private val out = ArrayList<Byte>(64)

    fun writeInt32(value: Int) {
        out += (value ushr 24 and 0xff).toByte()
        out += (value ushr 16 and 0xff).toByte()
        out += (value ushr 8 and 0xff).toByte()
        out += (value and 0xff).toByte()
    }

    fun writeUInt32(value: UInt) = writeInt32(value.toInt())

    fun writeFloat(value: Float) {
        val bits = value.toRawBits()
        out += (bits ushr 24 and 0xff).toByte()
        out += (bits ushr 16 and 0xff).toByte()
        out += (bits ushr 8 and 0xff).toByte()
        out += (bits and 0xff).toByte()
    }

    fun writeDouble(value: Double) {
        val bits = value.toRawBits()
        for (shift in 56 downTo 0 step 8) {
            out += (bits ushr shift and 0xff).toByte()
        }
    }

    fun writeBytes(bytes: ByteArray) {
        for (b in bytes) out += b
    }

    fun toByteArray(): ByteArray = ByteArray(out.size) { out[it] }
}

internal class RustBufferReader(private val bytes: ByteArray) {
    private var offset: Int = 0

    fun readInt32(): Int {
        require(offset + 4 <= bytes.size) { "buffer underflow at $offset" }
        val v = (bytes[offset].toInt() and 0xff shl 24) or
            (bytes[offset + 1].toInt() and 0xff shl 16) or
            (bytes[offset + 2].toInt() and 0xff shl 8) or
            (bytes[offset + 3].toInt() and 0xff)
        offset += 4
        return v
    }

    fun readDouble(): Double {
        require(offset + 8 <= bytes.size) { "buffer underflow at $offset" }
        var bits = 0L
        for (i in 0 until 8) {
            bits = (bits shl 8) or (bytes[offset + i].toLong() and 0xff)
        }
        offset += 8
        return Double.fromBits(bits)
    }

    fun readFramedString(): String {
        val len = readInt32()
        require(offset + len <= bytes.size) { "string underflow at $offset (len=$len)" }
        val s = bytes.decodeToString(offset, offset + len)
        offset += len
        return s
    }

    fun readFramedBytes(): ByteArray {
        val len = readInt32()
        require(offset + len <= bytes.size) { "bytes underflow at $offset (len=$len)" }
        val out = bytes.copyOfRange(offset, offset + len)
        offset += len
        return out
    }

    fun skipFramedBytes() {
        val len = readInt32()
        require(offset + len <= bytes.size) { "bytes underflow at $offset (len=$len)" }
        offset += len
    }

    fun readStringList(): List<String> {
        val n = readInt32()
        val out = ArrayList<String>(n)
        repeat(n) { out += readFramedString() }
        return out
    }
}
